use crate::{uo::UserOperationData, DataBase};
use async_trait::async_trait;
use mongodb::{
    bson::doc,
    options::{ClientOptions, UpdateOptions},
    Client,
};
use serde::{Deserialize, Serialize};

use super::UoError;

const UO_INDEXER_DB: &'static str = "UoIndexer";
const LATEST_BLOCK_NUMBER: &'static str = "latestBlockNumber";
const DOC_KEY_INDEX: i32 = 0;
const UO_COLLECTION: &'static str = "UserOperation";

pub struct MongoDB {
    _cli_options: ClientOptions,
    client: Client,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
    latest_block_number: i64,
    key: i32,
}

impl MongoDB {
    pub async fn new(url: String) -> Result<Self, UoError> {
        let cli_options = ClientOptions::parse(url).await?;
        let client = Client::with_options(cli_options.clone())?;
        Ok(MongoDB {
            _cli_options: cli_options,
            client,
        })
    }
}

#[async_trait]
impl DataBase for MongoDB {
    async fn get_last_block(&self) -> Result<u64, UoError> {
        let collection = self
            .client
            .clone()
            .database(UO_INDEXER_DB)
            .collection::<MetaData>(LATEST_BLOCK_NUMBER);
        match collection
            .find_one(doc! {"key": DOC_KEY_INDEX}, None)
            .await?
        {
            Some(m) => {
                Ok(<u64>::try_from(m.latest_block_number).expect("We are far from limitation"))
            }
            None => Ok(0),
        }
    }

    async fn write_last_block(&self, block_number: u64) -> Result<(), UoError> {
        let collection = self
            .client
            .clone()
            .database(UO_INDEXER_DB)
            .collection::<MetaData>(LATEST_BLOCK_NUMBER);
        let number = <i64>::try_from(block_number).expect("We are far from limitation");
        collection
            .update_one(
                doc! {"key": DOC_KEY_INDEX},
                doc! {"$set":{ "latest_block_number": number}},
                Some(UpdateOptions::builder().upsert(true).build()),
            )
            .await?;
        Ok(())
    }

    async fn write_user_operation(
        &self,
        uos: Vec<crate::uo::UserOperationData>,
    ) -> Result<(), UoError> {
        if !uos.is_empty() {
            let collection = self
                .client
                .clone()
                .database(UO_INDEXER_DB)
                .collection::<UserOperationData>(UO_COLLECTION);
            collection.insert_many(uos, None).await?;
        }
        Ok(())
    }
}
