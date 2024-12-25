#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Codec, Decode, Encode};
use scale_info::TypeInfo;
use sp_core::crypto::KeyTypeId;
use sp_core::sr25519::vrf::VrfSignature;
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
    pub trait NucleusApi<Address, Call, Extra>
    where
        Address: Codec,
        Call: Codec,
        Extra: Codec,
    {
        fn resolve_deploy_tx(uxt: Block::Extrinsic) -> Option<NucleusUpgradingTxInfo>;

        fn get_nucleus_info(nucleus_id: NucleusId) -> Option<NucleusInfo<AccountId, Hash, NodeId>>;

        fn compose_vrf_tx(
            nucleus_id: NucleusId,
            account_id: AccountId,
            nonce: u32,
            vrf: VrfSignature,
        ) -> (Address, Call, Extra);
    }
}

sp_api::decl_runtime_apis! {
    #[api_version(1)]
    pub trait ValidatorApi {
        fn is_active_validator(id: KeyTypeId, key_data: Vec<u8>) -> Option<AccountId>;
    }
}
