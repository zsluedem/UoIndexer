use std::fmt::{ Display};

use crate::uo::UserOperationData;
use async_trait::async_trait;

pub mod filestore;
use ::mongodb::error::Error;
pub use filestore::FileDB;
use thiserror::Error;

pub mod mongodb;

#[derive(Error, Debug)]
pub struct UoError(String);

impl Display for UoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<std::io::Error> for UoError {
    fn from(value: std::io::Error) -> Self {
        UoError(value.to_string())
    }
}

impl From<Error> for UoError {
    fn from(value: Error) -> Self {
        UoError(value.to_string())
    }
}

#[async_trait]
pub trait DataBase {
    async fn get_last_block(&self) -> Result<u64, UoError>;
    async fn write_user_operation(&self, uos: Vec<UserOperationData>) -> Result<(), UoError>;
    async fn write_last_block(&self, block_number: u64) -> Result<(), UoError>;
}

pub struct Storage {
    inner: Box<dyn DataBase>,
}

impl Storage {
    pub async fn new(db: Box<dyn DataBase>) -> Self {
        Self { inner: db }
    }
}

impl Storage {
    pub async fn get_last_block(&self) -> Result<u64, UoError> {
        self.inner.get_last_block().await
    }
    pub async fn write_user_operation(&self, uos: Vec<UserOperationData>) -> Result<(), UoError> {
        self.inner.write_user_operation(uos).await
    }
    pub async fn write_last_block(&self, block_number: u64) -> Result<(), UoError> {
        self.inner.write_last_block(block_number).await
    }
}
