use crate::uo::UserOperationData;
use async_trait::async_trait;
use std::{fs, io::Write, path::PathBuf};

use super::DataBase;

const LAST_BLOCK_FILE: &'static str = "last-block";

pub struct FileDB {
    folder: PathBuf,
}

#[async_trait]
impl DataBase for FileDB {
    type Error = std::io::Error;

    async fn get_last_block(&self) -> Result<u64, Self::Error> {
        let f = self.folder.join(LAST_BLOCK_FILE);
        if f.exists() {
            let data = fs::read_to_string(f)?;
            Ok(data.parse::<u64>().expect("block file data corrupted"))
        } else {
            Ok(0)
        }
    }

    async fn write_last_block(&self, block_number: u64) -> Result<(), Self::Error> {
        let f = self.folder.join(LAST_BLOCK_FILE);
        let mut fd = fs::OpenOptions::new().write(true).create(true).open(f)?;
        fd.write_fmt(format_args!("{block_number}"))?;
        fd.flush()?;
        Ok(())
    }

    async fn write_user_operation(&self, uos: Vec<UserOperationData>) -> Result<(), Self::Error> {
        for uo in uos {
            let f = self.folder.join("data").join(uo.uo_hash.to_string());
            let mut fd = fs::OpenOptions::new().write(true).create(true).open(f)?;
            fd.write_all(serde_json::to_string(&uo)?.as_bytes())?;
            fd.flush()?;
        }
        Ok(())
    }
}

impl FileDB {
    pub fn new(path: PathBuf) -> Result<Self, std::io::Error> {
        fs::create_dir_all(path.clone())?;
        fs::create_dir_all(path.clone().join("data"))?;
        Ok(Self { folder: path })
    }
}
