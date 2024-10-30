use crate::{Config, Pallet};
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::traits::Convert;
use sp_runtime::RuntimeDebug;
use sp_staking::{EraIndex, Exposure, SessionIndex};
use sp_std::collections::btree_map::BTreeMap;
use sp_std::vec::Vec;
pub type RewardPoint = u32;

#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    Encode,
    Decode,
    RuntimeDebug,
    TypeInfo,
    MaxEncodedLen,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Forcing {
    NotForcing,
    ForceNew,
    ForceNone,
    ForceAlways,
}

impl Default for Forcing {
    fn default() -> Self {
        Forcing::NotForcing
    }
}

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

/// Information regarding the active era (era in used in session).
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct ActiveEraInfo {
    pub index: EraIndex,
    pub set_id: u32,
    pub start: Option<u64>,
}

impl<T: Config> SessionInterface<<T as frame_system::Config>::AccountId> for T
where
    T: pallet_session::Config<ValidatorId = <T as frame_system::Config>::AccountId>,
    T: pallet_session::historical::Config<
        FullIdentification = u128, //TODO Exposure<<T as frame_system::Config>::AccountId, BalanceOf<T>>,
        FullIdentificationOf = ExposureOf<T>,
    >,
    T::SessionHandler: pallet_session::SessionHandler<<T as frame_system::Config>::AccountId>,
    T::SessionManager: pallet_session::SessionManager<<T as frame_system::Config>::AccountId>,
    T::ValidatorIdOf: Convert<
        <T as frame_system::Config>::AccountId,
        Option<<T as frame_system::Config>::AccountId>,
    >,
{
    fn disable_validator(validator_index: u32) -> bool {
        <pallet_session::Pallet<T>>::disable_index(validator_index)
    }

    fn validators() -> Vec<<T as frame_system::Config>::AccountId> {
        <pallet_session::Pallet<T>>::validators()
    }

    fn prune_historical_up_to(up_to: SessionIndex) {
        <pallet_session::historical::Pallet<T>>::prune_up_to(up_to);
    }
}

pub trait SessionInterface<AccountId> {
    /// Disable the validator at the given index, returns `false` if the validator was already
    /// disabled or the index is out of bounds.
    fn disable_validator(validator_index: u32) -> bool;
    /// Get the validators from session.
    fn validators() -> Vec<AccountId>;
    /// Prune historical session tries up to but not including the given index.
    fn prune_historical_up_to(up_to: SessionIndex);
}

pub struct ExposureOf<T>(sp_std::marker::PhantomData<T>);

impl<T: Config> Convert<T::AccountId, Option<u128>> for ExposureOf<T> {
    fn convert(validator: T::AccountId) -> Option<u128> {
        //TODO
        <Pallet<T>>::active_era()
            .map(|active_era| <Pallet<T>>::eras_stakers(active_era.index, &validator))
    }
}
