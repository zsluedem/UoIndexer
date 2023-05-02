use async_trait::async_trait;
use rocksdb::{DBWithThreadMode, SingleThreaded, DB};
use std::path::PathBuf;

use crate::uo::UserOperationData;

use super::{DataBase, UoError};

const LAST_BLOCK_DB: &str = "lastBlock";
const UO: &str = "UserOperation";

pub struct RocksDb {
    _db_path: PathBuf,
    instance: DBWithThreadMode<SingleThreaded>,
}

fn u8_vec_to_u64(vec: Vec<u8>) -> u64 {
    let mut arr = [0u8; 8];
    arr.copy_from_slice(&vec[..8]); // copy first 8 bytes into array
    u64::from_le_bytes(arr)
}

fn u64_to_u8_vec(num: u64) -> Vec<u8> {
    num.to_le_bytes().to_vec()
}

#[async_trait]
impl DataBase for RocksDb {
    async fn get_last_block(&self) -> Result<u64, UoError> {
        let cf = self
            .instance
            .cf_handle(LAST_BLOCK_DB)
            .ok_or_else(|| UoError("Could not find lastBlock column family".to_string()))?;
        match self.instance.get_cf(cf, b"lastBlock")? {
            Some(res) => Ok(u8_vec_to_u64(res)),
            None => Ok(0u64),
        }
    }
    async fn write_user_operation(&self, uos: Vec<UserOperationData>) -> Result<(), UoError> {
        let cf = self
            .instance
            .cf_handle(UO)
            .ok_or_else(|| UoError("Could not find lastBlock column family".to_string()))?;
        for uo in uos {
            let j = serde_json::to_vec(&uo).map_err(|e| UoError(e.to_string()))?;
            self.instance.put_cf(cf, uo.uo_hash.to_string(), j)?;
        }
        self.instance.flush()?;
        Ok(())
    }
    async fn write_last_block(&self, block_number: u64) -> Result<(), UoError> {
        let cf = self
            .instance
            .cf_handle(LAST_BLOCK_DB)
            .ok_or_else(|| UoError("Could not find lastBlock column family".to_string()))?;
        self.instance
            .put_cf(cf, b"lastBlock", u64_to_u8_vec(block_number))?;
        self.instance.flush()?;
        Ok(())
    }
}

impl RocksDb {
    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        let mut options = rocksdb::Options::default();
        options.set_error_if_exists(false);
        options.create_if_missing(true);
        options.create_missing_column_families(true);
        let cfs = DB::list_cf(&options, path.clone()).unwrap_or(vec![]);

        if !cfs.iter().any(|cf| cf.as_str() == LAST_BLOCK_DB) {
            let mut instance = rocksdb::DB::open_cf(&options, path.clone(), cfs.clone())?;
            let options = rocksdb::Options::default();
            instance.create_cf(LAST_BLOCK_DB, &options)?;
        }

        if !cfs.iter().any(|cf| cf.as_str() == UO) {
            let mut instance = rocksdb::DB::open_cf(&options, path.clone(), cfs)?;

            let options = rocksdb::Options::default();
            instance.create_cf(UO, &options)?;
        }
        let instance = rocksdb::DB::open_cf(&options, path.clone(), vec![LAST_BLOCK_DB, UO])?;
        Ok(Self {
            _db_path: path,
            instance,
        })
    }
}
