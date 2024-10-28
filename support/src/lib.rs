#![cfg_attr(not(feature = "std"), no_std)]

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

pub trait ValidatorsProvider<AccountId> {
    fn provide() -> Vec<(AccountId, u128)>;
	fn next_validators_set_id() -> u32;
}
