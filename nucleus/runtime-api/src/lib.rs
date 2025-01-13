#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Codec, Decode, Encode};
use scale_info::TypeInfo;
use sp_core::bytes::to_hex;
use sp_core::crypto::KeyTypeId;
use sp_runtime::Vec;
use vrs_primitives::*;

#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo)]
pub struct NucleusUpgradingTxInfo {
    pub nucleus_id: NucleusId,
    pub wasm_hash: Hash,
    pub node_id: NodeId,
}

sp_api::decl_runtime_apis! {
    #[api_version(1)]
    pub trait NucleusApi<Address>
    where
        Address: Codec,
    {
        fn resolve_deploy_tx(uxt: Block::Extrinsic) -> Option<NucleusUpgradingTxInfo>;

        fn get_nucleus_info(nucleus_id: NucleusId) -> Option<NucleusInfo<AccountId, Hash, NodeId>>;
    }
}

sp_api::decl_runtime_apis! {
    #[api_version(1)]
    pub trait ValidatorApi {
        fn is_active_validator(id: KeyTypeId, key_data: Vec<u8>) -> Option<AccountId>;
    }
}

#[derive(Decode, Encode)]
pub struct TT {
    b:u64,
    v:u32,
}

#[test]
pub fn test() {
    let r = TT {
        b: 840000,
        v: 846,
    };
    println!("{:?}", to_hex(r.encode().as_slice(), false));
}