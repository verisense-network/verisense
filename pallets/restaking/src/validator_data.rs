use alloc::string::String;
use codec::{Decode, Encode};
use ethabi::{ParamType, Token};
use scale_info::TypeInfo;
use serde::Deserialize;
use serde::Serialize;
use sp_core::bounded::alloc;
use sp_std::boxed::Box;
use sp_std::vec;
use sp_std::vec::Vec;
#[derive(
    Serialize, Default, Decode, Encode, Deserialize, Clone, PartialEq, Eq, Debug, TypeInfo,
)]
pub struct ValidatorData {
    pub operator: [u8; 20],
    pub stake: u128,
    pub key: Vec<u8>,
    pub strategies: Vec<[u8; 20]>,
     pub source: String,
}

impl TryFrom<Vec<Token>> for ValidatorData {
    type Error = String;
    fn try_from(vr: Vec<Token>) -> Result<Self, Self::Error> {
        let operator = vr[0]
            .clone()
            .into_address()
            .ok_or("Invalid validator address")?
            .0;
        let stake = vr[1]
            .clone()
            .into_uint()
            .ok_or("Invalid validator amount")?
            .as_u128();
        let key = vr[2]
            .clone()
            .into_fixed_bytes()
            .ok_or("Invalid validator key")?
            .to_vec();
        let strategies = vr[3]
            .clone()
            .into_array()
            .ok_or("invalid arr")?
            .iter()
            .map(|t| t.clone().into_address().unwrap().0)
            .collect::<Vec<[u8; 20]>>();
        Ok(ValidatorData {
            operator,
            stake,
            key,
            strategies,
            source: String::new(),
        })
    }
}

pub fn decode_validator_datas(vr: Vec<Token>) -> Result<Vec<ValidatorData>, String> {
    if vr.is_empty() {
        return Ok(vec![]);
    }
    let token = vr[0].clone();
    let mut rs = vec![];
    if let Token::Array(vs) = token {
        for v in vs {
            if let Token::Tuple(vr) = v {
                let data = ValidatorData::try_from(vr)?;
                rs.push(data);
            }
        }
    }
    Ok(rs)
}

pub fn parse_to_tokens(value: &str) -> Vec<Token> {
    if value.is_empty() {
        return vec![];
    }
    let value = value.replace("0x", "");
    match hex::decode(value) {
        Ok(v) => ethabi::decode(
            &[ethabi::ParamType::Array(Box::new(
                ethabi::ParamType::Tuple(vec![
                    ParamType::Address,
                    ParamType::Uint(256),
                    ParamType::FixedBytes(32),
                    ParamType::Bool,
                    ParamType::Array(Box::new(ParamType::Address)),
                ]),
            ))],
            &v,
        )
        .unwrap_or(vec![]),
        Err(e) => {
            return vec![];
        }
    }
}
