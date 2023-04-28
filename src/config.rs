use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(long)]
    pub rpc_url: String,

    #[arg(long)]
    pub chain_id: u64,

    #[arg(long)]
    pub db_path: String,
}
