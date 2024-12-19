#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::KeyTypeId;
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
