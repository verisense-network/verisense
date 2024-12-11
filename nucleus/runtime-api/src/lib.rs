#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, Encode};
use scale_info::prelude::vec::Vec;
use scale_info::TypeInfo;
use vrs_primitives::*;

#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo)]
pub struct NucleusUpgradingTxInfo {
    pub nucleus_id: NucleusId,
    pub wasm_hash: Hash,
    pub node_id: NodeId,
}

sp_api::decl_runtime_apis! {
    #[api_version(1)]
    pub trait NucleusApi {
        fn resolve_deploy_tx(uxt: Block::Extrinsic) -> Option<NucleusUpgradingTxInfo>;

        fn get_nucleus_info(nucleus_id: NucleusId) -> Option<NucleusInfo<AccountId, Hash, NodeId>>;
        fn get_validators(nucleus_id: &NucleusId) -> Option<Vec<AccountId>>;
    }

}
