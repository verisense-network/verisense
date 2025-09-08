use a2a_rs::AgentCard;
use codec::{Decode, Encode};
use frame_support::pallet_prelude::{OptionQuery, TypeInfo};
use frame_support::{storage_alias, Blake2_128Concat};

/// V0 type for [`crate::Value`].
#[storage_alias]
pub type Agents<T: crate::Config> = StorageMap<
    crate::Pallet<T>,
    Blake2_128Concat,
    <T as frame_system::Config>::AccountId,
    OldAgentInfo<<T as frame_system::Config>::AccountId>,
    OptionQuery,
>;

#[derive(Debug, Clone, Decode, Encode, TypeInfo, Eq, PartialEq)]
pub struct OldAgentInfo<AccountId> {
    pub agent_id: AccountId,
    pub owner_id: AccountId,
    pub agent_card: AgentCard,
}
