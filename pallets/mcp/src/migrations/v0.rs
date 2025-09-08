use crate::Config;
use codec::{Decode, Encode};
use frame_support::pallet_prelude::{OptionQuery, TypeInfo};
use frame_support::{storage_alias, Blake2_128Concat};
use sp_std::vec::Vec;

#[storage_alias]
pub type Servers<T: Config> = StorageMap<
    crate::Pallet<T>,
    Blake2_128Concat,
    <T as frame_system::Config>::AccountId,
    OldMcpServerInfo<<T as frame_system::Config>::AccountId>,
    OptionQuery,
>;

#[derive(Clone, Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
pub struct OldMcpServerInfo<AccountId> {
    pub name: Vec<u8>,
    pub description: Vec<u8>,
    pub url: Vec<u8>,
    pub provider: AccountId,
}
