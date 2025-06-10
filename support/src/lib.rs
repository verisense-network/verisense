#![cfg_attr(not(feature = "std"), no_std)]

pub mod consts;

use a2a_rs::AgentInfo;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{KeyTypeId, RuntimeDebug};
use sp_std::collections::btree_map::BTreeMap;
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

pub trait RestakingInterface<AccountId: Ord> {
    fn provide() -> Vec<(AccountId, u128)>;
    fn next_validators_set_id() -> u32;
    fn plan_new_era();
    fn on_end_era(era_idx: u32, era_reward_points: EraRewardPoints<AccountId>);
}

pub trait ValidatorsInterface<AccountId> {
    fn lookup_active_validator(id: KeyTypeId, key_data: &[u8]) -> Option<AccountId>;
    fn validators() -> Vec<AccountId>;
    fn active_stake_of(who: &AccountId) -> u128;
    fn active_total_stake() -> Option<u128>;
}

pub trait AgentRegistry<AccountId> {
    type Err;

    fn register_agent(agent: AgentInfo<AccountId>) -> Result<(), Self::Err>;

    fn find_agent(agent_id: &AccountId) -> Option<AgentInfo<AccountId>>;
}

pub type RewardPoint = u128;
pub type EvmAddress = [u8; 20];

#[derive(PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct EraRewardPoints<AccountId: Ord> {
    pub total: RewardPoint,
    pub individual: BTreeMap<AccountId, RewardPoint>,
}

impl<AccountId: Ord> Default for EraRewardPoints<AccountId> {
    fn default() -> Self {
        EraRewardPoints {
            total: Default::default(),
            individual: BTreeMap::new(),
        }
    }
}
