#![cfg_attr(not(feature = "std"), no_std)]
use vrs_primitives::*;
use codec::{Encode, Decode};
use scale_info::TypeInfo;

#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo)]
pub struct NucleusWasmInfo {
    pub nucleus_id: NucleusId,
    pub wasm_hash: Hash,
    pub node_addr: NodeAddress,
}

sp_api::decl_runtime_apis! {
    #[api_version(1)]
    pub trait NucleusApi {
        fn resolve_deploy_tx(uxt: Block::Extrinsic) -> Option<NucleusWasmInfo>;
    }
}
