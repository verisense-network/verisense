use crate::String;
use crate::Vec;
use codec::Encode;
use ethabi::Address;
use scale_info::prelude::format;
use serde::{Deserialize, Serialize};
use sp_core::{keccak_256, U256};
use sp_std::vec;

pub const GET_VALIDATOR_SET_METHOD: &str = "getValidatorSet()";

pub fn query_validators_params() -> String {
    let method_signature = keccak_256(GET_VALIDATOR_SET_METHOD.as_bytes());
    let mut v = vec![];
    v.append(&mut method_signature[0..4].to_vec());
    format!("0x{}", hex::encode(v))
}

pub fn decode_query_validators_resp(resp: String) -> Vec<ValidatorData> {

    if resp.is_empty() {
        return vec![];
    }
    let s = if resp.starts_with("0x") {
        resp.strip_prefix("0x").unwrap()
    } else {
        resp.as_str()
    };
    let mut reader = Reader(hex::decode(s).unwrap());
    reader.read(32);
    let data_size = U256::from_big_endian(reader.read(32).as_slice()).as_u32();
    let mut v = vec![];
    for i in 0..data_size {
        reader.read(12);
        let addr = reader.read(20);
        let evm_address = format!("0x{}",  hex::encode(addr));
        let stake = U256::from_big_endian(reader.read(32).as_slice()).as_u128();
        let key = reader.read(32);
        v.push(ValidatorData { stake, evm_address, key })
    }
    v
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidatorData {
    pub stake: u128,
    pub evm_address: String,
    pub key: Vec<u8>,
}

pub struct Reader(Vec<u8>);

impl Reader {
    pub fn read(&mut self, l: usize) -> Vec<u8> {
        let v = self.0[0..l].to_vec();
        self.0 = self.0[l..].to_vec();
        v
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[test]
pub fn t() {
    let a = Address::from([23u8;20]);
    println!("{}", format!("0x{}", hex::encode(a)));
}