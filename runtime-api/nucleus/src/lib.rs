#![cfg_attr(not(feature = "std"), no_std)]
use a2a_rs::AgentCard;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use vrs_primitives::*;

#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo)]
pub struct NucleusUpgradingTxInfo {
    pub nucleus_id: NucleusId,
    pub wasm_hash: Hash,
    pub node_id: NodeId,
    pub agent_card: Option<AgentCard>,
}

sp_api::decl_runtime_apis! {
    #[api_version(2)]
    pub trait NucleusRuntimeApi {
        fn resolve_deploy_tx(uxt: Block::Extrinsic) -> Option<NucleusUpgradingTxInfo>;

        fn get_nucleus_info(nucleus_id: &NucleusId) -> Option<NucleusInfo<AccountId, Hash, NodeId>>;

        fn is_member_of(
            nucleus_id: &NucleusId,
            account_id: &AccountId,
        ) -> bool;
    }
}
