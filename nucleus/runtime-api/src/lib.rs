#![cfg_attr(not(feature = "std"), no_std)]
use vrs_primitives::NucleusWasmInfo;

sp_api::decl_runtime_apis! {
    #[api_version(1)]
    pub trait NucleusApi {
        fn resolve_deploy_tx(uxt: Block::Extrinsic) -> Option<NucleusWasmInfo>;
    }
}
