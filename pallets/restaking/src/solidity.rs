use crate::String;
use scale_info::prelude::format;
use sp_core::keccak_256;
use sp_std::vec;
use sp_std::vec::Vec;

pub const GET_VALIDATOR_SET_METHOD: &str = "getOperators()";

pub fn query_validators_params() -> String {
    let method_signature = keccak_256(GET_VALIDATOR_SET_METHOD.as_bytes());
    let mut v = vec![];
    v.append(&mut method_signature[0..4].to_vec());
    format!("0x{}", hex::encode(v))
}

pub struct Reader(Vec<u8>);

impl Reader {
    pub fn read(&mut self, l: usize) -> Vec<u8> {
        let v = self.0[0..l].to_vec();
        self.0 = self.0[l..].to_vec();
        v
    }
}
