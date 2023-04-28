use lazy_static::lazy_static;

pub const ENTRY_POINT_ADDR: &'static str = "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789";

#[derive(Debug, Clone)]
pub struct ChainSpec {
    pub chain_id: u64,
    pub name: String,
    pub contract_deployed_block_number: u64,
}

lazy_static! {
    pub static ref SUPPORT_CHAIN: [ChainSpec; 2] = [
        ChainSpec {
            chain_id: 1,
            name: "ETH".to_string(),
            contract_deployed_block_number: 17012204u64
        },
        ChainSpec {
            chain_id: 5,
            name: "Goerli".to_string(),
            contract_deployed_block_number: 8801632u64
        }
    ];
}
