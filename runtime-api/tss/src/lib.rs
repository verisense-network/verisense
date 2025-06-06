#![cfg_attr(not(feature = "std"), no_std)]
use sp_runtime::Vec;

sp_api::decl_runtime_apis! {
    #[api_version(1)]
    pub trait VrsTssRuntimeApi {
        fn get_all_validators() -> Vec<sp_runtime::AccountId32>;
    }
}
