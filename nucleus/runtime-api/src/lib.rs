#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use vrs_primitives::*;

#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo)]
pub struct NucleusWasmInfo {
    pub nucleus_id: NucleusId,
    pub wasm_hash: Hash,
    pub node_id: NodeId,
}

sp_api::decl_runtime_apis! {
    #[api_version(1)]
    pub trait NucleusApi {
        fn resolve_deploy_tx(uxt: Block::Extrinsic) -> Option<NucleusWasmInfo>;
    }
}
