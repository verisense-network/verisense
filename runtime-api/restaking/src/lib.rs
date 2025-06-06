#![cfg_attr(not(feature = "std"), no_std)]

use vrs_primitives::*;
use codec::Codec;
use sp_runtime::traits::MaybeDisplay;

sp_api::decl_runtime_apis! {
    #[api_version(1)]
    pub trait VrsRestakingRuntimeApi<AccountId>
    where AccountId: Codec + MaybeDisplay {
        fn get_rewards_proof(account_id: AccountId) -> RewardsProof;
    }
}
