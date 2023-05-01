use crate::uo::UserOperationData;

pub trait DataBase {
    type Error;
    fn get_last_block(&self) -> u64;
    fn write_user_operation(&self, uos: Vec<UserOperationData>) -> Result<(), Self::Error>;
    fn write_last_block(&self, block_number: u64) -> Result<(), Self::Error>;
}

pub mod filestore;
pub use filestore::FileDB;
