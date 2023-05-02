use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(long)]
    pub rpc_url: String,

    #[arg(long)]
    pub chain_id: u64,

    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand, Debug)]
pub enum Mode {
    /// Run the indexer with file base storage
    File(FileArgs),
    /// Run the indexer with MongoDB storage
    MongoDB(MongoArgs),
}

#[derive(Args, Debug)]
pub struct FileArgs {
    pub db_path: String,
}

#[derive(Args, Debug)]
pub struct MongoArgs {
    pub uri: String,
}
