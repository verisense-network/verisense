#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::{traits::BlockNumber, DispatchError, KeyTypeId};
use sp_std::vec::Vec;
#[macro_export]
macro_rules! log {
	($level:tt, $patter:expr $(, $values:expr)* $(,)?) => {
		log::$level!(
			target: crate::LOG_TARGET,
			concat!("[{:?}] ", $patter), <frame_system::Pallet<T>>::block_number() $(, $values)*
		)
	};
}

pub trait RestakingInterface<AccountId> {
    fn provide() -> Vec<(AccountId, u128)>;
    fn next_validators_set_id() -> u32;
    fn plan_new_era();
}

pub trait ValidatorsInterface<AccountId> {
    fn is_active_validator(id: KeyTypeId, key_data: &[u8]) -> Option<AccountId>;
    fn validators() -> Vec<AccountId>;
    fn active_stake_of(who: &AccountId) -> u128;
    fn active_total_stake() -> Option<u128>;
}

pub trait VrfInterface<NucleusId, BlockNumber, AccountId> {
    fn register_nucleus_blocknumber(
        nucleus_id: NucleusId,
        block_number: BlockNumber,
        validators_num: u32,
    ) -> Result<(), DispatchError>;

    fn get_validators(nucleus_id: &NucleusId) -> Option<Vec<AccountId>>;
    // fn is_validator(who: &AccountId) -> bool;
    // fn get_seeds_info(
    //     nucleus_id: &NucleusId,
    //     account_id: &AccountId,
    // ) -> Option<SeedsInfo<AccountId, NucleusId, VrfId>>;
    // fn update_seed(
    //     nucleus_id: NucleusId,
    //     vrf_id: VrfId,
    //     seed: Vec<u8>,
    //     signature: VrfSignature,
    // ) -> Result<(), DispatchError>;
}
