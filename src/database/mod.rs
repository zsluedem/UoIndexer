use crate::uo::UserOperationData;
use async_trait::async_trait;

#[async_trait]
pub trait DataBase {
    type Error;
    async fn get_last_block(&self) -> Result<u64, Self::Error>;
    async fn write_user_operation(&self, uos: Vec<UserOperationData>) -> Result<(), Self::Error>;
    async fn write_last_block(&self, block_number: u64) -> Result<(), Self::Error>;
}

pub mod filestore;
pub use filestore::FileDB;
pub mod mongodb;
