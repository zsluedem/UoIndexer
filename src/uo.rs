use ethers::{
    abi::AbiEncode,
    prelude::{EthAbiCodec, EthAbiType, EthDisplay, EthEvent},
    types::{Address, Bytes, H256, U256},
    utils::{keccak256, to_checksum},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserOperationData {
    pub uo: UserOperation,
    pub uo_hash: H256,
    pub transaction_hash: H256,
    pub transaction_index: u64,
    pub block_number: u64,
    pub block_hash: H256,
}

#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, EthAbiCodec, EthAbiType,
)]
#[serde(rename_all = "camelCase")]
pub struct UserOperation {
    #[serde(serialize_with = "as_checksum")]
    pub sender: Address,
    pub nonce: U256,
    pub init_code: Bytes,
    pub call_data: Bytes,
    pub call_gas_limit: U256,
    pub verification_gas_limit: U256,
    pub pre_verification_gas: U256,
    pub max_fee_per_gas: U256,
    pub max_priority_fee_per_gas: U256,
    pub paymaster_and_data: Bytes,
    pub signature: Bytes,
}

impl UserOperation {
    pub fn hash(&self) -> H256 {
        let packed = self.pack();
        keccak256(packed).into()
    }

    pub fn pack(&self) -> Vec<u8> {
        let init_code_hash = keccak256(self.init_code.clone());
        let call_data_hash = keccak256(self.call_data.clone());
        let paymaster_and_data_hash = keccak256(self.paymaster_and_data.clone());
        (
            self.sender,
            self.nonce,
            init_code_hash,
            call_data_hash,
            self.call_gas_limit,
            self.verification_gas_limit,
            self.pre_verification_gas,
            self.max_fee_per_gas,
            self.max_priority_fee_per_gas,
            paymaster_and_data_hash,
        )
            .encode()
    }

    pub fn uo_hash(&self, entry_point_addr: Address, chain_id: u64) -> H256 {
        keccak256((self.hash(), entry_point_addr, U256::from(chain_id)).encode()).into()
    }
}

fn as_checksum<S>(val: &Address, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&to_checksum(val, None))
}

#[derive(Clone, Debug, Eq, PartialEq, EthEvent, EthDisplay, Default)]
#[ethevent(
    name = "UserOperationEvent",
    abi = "UserOperationEvent(bytes32,address,address,uint256,bool,uint256,uint256)"
)]
pub struct UserOperationEvent {
    #[ethevent(indexed)]
    pub user_op_hash: [u8; 32],
    #[ethevent(indexed)]
    pub sender: ethers::core::types::Address,
    #[ethevent(indexed)]
    pub paymaster: ethers::core::types::Address,
    pub nonce: ethers::core::types::U256,
    pub success: bool,
    pub actual_gas_cost: ethers::core::types::U256,
    pub actual_gas_price: ethers::core::types::U256,
}

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthCall,
    ethers :: contract :: EthDisplay,
    Default,
)]
#[ethcall(
    name = "handleOps",
    abi = "handleOps((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes)[],address)"
)]
pub struct HandleOpsCall {
    pub ops: Vec<UserOperation>,
    pub beneficiary: Address,
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use ethers::types::{Bytes, H160, U256};

    use super::UserOperation;

    #[test]
    fn user_operation_hash() {
        let user_operation = UserOperation {
            sender: H160::from_str("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266").unwrap(),
            nonce: U256::from(0),
            init_code: Bytes::new(),
            call_data: Bytes::new(),
            call_gas_limit: U256::from(0),
            verification_gas_limit: U256::from(150000),
            pre_verification_gas: U256::from(21000),
            max_fee_per_gas: U256::from(0),
            max_priority_fee_per_gas: U256::from(1000000000),
            paymaster_and_data: Bytes::new(),
            signature: Bytes::new(),
        };
        assert_eq!(
            Bytes::from_str("0x000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb922660000000000000000000000000000000000000000000000000000000000000000c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000249f000000000000000000000000000000000000000000000000000000000000052080000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003b9aca00c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470").unwrap().0.to_vec(), 
            user_operation.pack());
    }
}
