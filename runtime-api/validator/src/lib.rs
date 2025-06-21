#![cfg_attr(not(feature = "std"), no_std)]

use sp_core::crypto::KeyTypeId;
use sp_std::vec::Vec;
use vrs_primitives::AccountId;

sp_api::decl_runtime_apis! {
    #[api_version(1)]
    pub trait ValidatorRuntimeApi {
        fn lookup_active_validator(id: KeyTypeId, key_data: Vec<u8>) -> Option<AccountId>;

        fn get_genesis_validators() -> Vec<AccountId>;

        fn get_current_validators() -> Vec<AccountId>;
    }
}
