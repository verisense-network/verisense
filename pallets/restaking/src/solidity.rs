use crate::String;
use crate::Vec;
use scale_info::prelude::format;
use serde::{Deserialize, Serialize};
use sp_core::{keccak_256, U256};
use sp_std::vec;

pub const GET_VALIDATOR_SET_METHOD: &str = "getOperators()";

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
    for _i in 0..data_size {
        reader.read(12);
        let addr = reader.read(20);
        let operator = format!("0x{}",  hex::encode(addr));
        let stake = U256::from_big_endian(reader.read(32).as_slice()).as_u128();
        let key = reader.read(32);
        let strategies = vec![];
        v.push(ValidatorData { operator, stake, key, strategies });
    }
    v
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidatorData {
    pub operator: String,
    pub stake: u128,
    pub key: Vec<u8>,
    pub strategies: Vec<String>
}

pub struct Reader(Vec<u8>);

impl Reader {
    pub fn read(&mut self, l: usize) -> Vec<u8> {
        let v = self.0[0..l].to_vec();
        self.0 = self.0[l..].to_vec();
        v
    }
}

#[test]
pub fn t() {
    let a = Address::from([23u8;20]);
    println!("{}", format!("0x{}", hex::encode(a)));
}