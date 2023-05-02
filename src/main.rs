mod cli;
mod constrant;
mod database;
mod uo;

use std::{path::PathBuf, str::FromStr, sync::Arc, time::Duration};

use clap::Parser;
use cli::Cli;
use ethers::{
    prelude::EthEvent,
    providers::{Http, Middleware, Provider},
    types::{Address, Filter},
};

use tokio::time;
use tokio_retry::{strategy::FixedInterval, Retry};
use tracing::{debug, info};

use crate::{
    cli::Mode,
    constrant::{ENTRY_POINT_ADDR, SUPPORT_CHAIN},
    database::{mongodb::MongoDB, rocksdb_storage::RocksDb, DataBase, FileDB, Storage},
    uo::{HandleOpsCall, UserOperationData, UserOperationEvent},
};

const RETRY_INTERVAL_MILLI: u64 = 5000;
const MAX_STEP: u64 = 10;
const INTERBAL: u64 = 13;

async fn fetch_uo_logs(
    start: u64,
    end: u64,
    provider: Arc<Provider<Http>>,
    chain_id: u64,
) -> anyhow::Result<Vec<UserOperationData>> {
    info!("Trying to get user operations from {} to {}", start, end);
    let filter = Filter::new()
        .from_block(start)
        .to_block(end)
        .address(Address::from_str(ENTRY_POINT_ADDR).expect("Const is formal address"))
        .topic0(UserOperationEvent::signature());
    let p = provider.clone();
    let results = p.get_logs(&filter).await?;
    let mut data_result = Vec::with_capacity(results.len());
    for log in results {
        let transaction_hash = log
            .transaction_hash
            .expect("Log belongs to transaction hash.");
        let res = p.get_transaction(transaction_hash).await?;
        let transaction = res.expect("Transaction should exist");
        let handles = <HandleOpsCall as ethers::core::abi::AbiDecode>::decode(transaction.input)?;
        let result = handles
            .ops
            .iter()
            .find(|&op| {
                log.topics[1]
                    == op.uo_hash(Address::from_str(ENTRY_POINT_ADDR).expect("Good"), chain_id)
            })
            .unwrap()
            .to_owned();
        let uo_hash = result.hash();
        let data = UserOperationData {
            uo: result,
            uo_hash: uo_hash,
            transaction_hash,
            transaction_index: log.transaction_index.unwrap().as_u64(),
            block_number: log.block_number.unwrap().as_u64(),
            block_hash: log.block_hash.unwrap(),
        };
        println!("Find {data:?}");
        data_result.push(data);
    }
    info!("Done getting data from {} to {}", start, end);
    Ok(data_result)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let config = Cli::parse();
    debug!("Starting BIndexer with config {config:?}");

    let db: Storage = match config.mode {
        Mode::File(args) => {
            Storage::new(Box::new(FileDB::new(PathBuf::from_str(&args.db_path)?)?)).await
        }
        Mode::MongoDB(args) => Storage::new(Box::new(MongoDB::new(args.uri).await?)).await,
        Mode::RocksDB(args) => {
            Storage::new(Box::new(RocksDb::new(PathBuf::from_str(&args.db_path)?)?)).await
        }
    };
    let retry = FixedInterval::from_millis(RETRY_INTERVAL_MILLI);

    let last_block = db.get_last_block().await?;
    let provider = Arc::new(Provider::<Http>::try_from(config.rpc_url.clone())?);
    let mut latest_block = provider.clone().get_block_number().await?.as_u64();
    info!("Latest block in the network is {:?}", latest_block);

    let chain_id = provider.clone().get_chainid().await?.as_u64();
    if chain_id != config.chain_id {
        return Err(anyhow::anyhow!(
            "The rpc chain id is not the same chain id as the config."
        ));
    }

    let chain_spec = match SUPPORT_CHAIN.iter().find(|&c| c.chain_id == chain_id) {
        Some(spec) => spec.clone(),
        None => {
            return Err(anyhow::anyhow!(
                "The chain id {chain_id} is not supported right now. "
            ))
        }
    };

    let mut current_block = {
        if last_block == 0 {
            info!("There is no history user operation in current database now. The indexer would start from scratch and it would take some time to finish.");
            chain_spec.contract_deployed_block_number
        } else {
            last_block
        }
    };
    info!("Current block is {current_block:?}");

    let mut interval = time::interval(Duration::from_secs(INTERBAL)); // 13 is basically the interval between each block in eth

    loop {
        if current_block + 1 <= latest_block {
            if current_block + MAX_STEP > latest_block {
                let res =
                    fetch_uo_logs(current_block, latest_block, provider.clone(), chain_id).await?;

                db.write_user_operation(res).await?;
                current_block = latest_block;
                db.write_last_block(current_block).await?;
            } else {
                info!("Indexer is going to continuously fetching logs from {current_block} to {latest_block}");
                // When the current indexer is not up to date and fall behind more than MAX_STEP blocks
                for target in (current_block + MAX_STEP..latest_block).step_by(MAX_STEP as usize) {
                    let res =
                        fetch_uo_logs(current_block, target, provider.clone(), chain_id).await?;
                    db.write_user_operation(res).await?;
                    current_block = target;
                    db.write_last_block(current_block).await?;
                }
            };
        }

        interval.tick().await;
        latest_block = Retry::spawn(retry.clone(), || {
            info!("Trying to get the latest block.");
            provider.get_block_number()
        })
        .await?
        .as_u64();
        info!("Latest block is {latest_block:?}")
    }
}
