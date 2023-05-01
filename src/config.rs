use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(long)]
    pub rpc_url: String,

    #[arg(long)]
    pub chain_id: u64,

    #[arg(long)]
    pub db_path: String,

    #[arg(long, default_value = "mongodb://localhost:27017")]
    pub mongodb_uri: String,

    #[arg(value_enum)]
    pub mode: Mode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Mode {
    File,
    MongoDB,
}
