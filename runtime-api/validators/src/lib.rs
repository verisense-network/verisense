#![cfg_attr(not(feature = "std"), no_std)]
use sp_core::crypto::KeyTypeId;
use sp_runtime::Vec;
use vrs_primitives::*;

sp_api::decl_runtime_apis! {
    #[api_version(1)]
    pub trait ValidatorApi {
        fn is_active_validator(id: KeyTypeId, key_data: Vec<u8>) -> Option<AccountId>;
    }
}
