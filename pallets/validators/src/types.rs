use crate::{Config, Pallet};
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::traits::Convert;
use sp_runtime::KeyTypeId;
use sp_runtime::RuntimeDebug;
use sp_staking::{EraIndex, SessionIndex};
use sp_std::vec::Vec;

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

/// Information regarding the active era (era in used in session).
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
pub struct ActiveEraInfo {
    pub index: EraIndex,
    pub set_id: u32,
    pub start: Option<u64>,
}

impl<T: Config> SessionInterface<<T as frame_system::Config>::AccountId> for T
where
    T: pallet_session::Config<ValidatorId = <T as frame_system::Config>::AccountId>,
    T: pallet_session::historical::Config<
        FullIdentification = u128,
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

    fn is_active_validator(
        id: KeyTypeId,
        key_data: &[u8],
    ) -> Option<<T as frame_system::Config>::AccountId> {
        let who = <pallet_session::Pallet<T>>::key_owner(id, key_data);
        if who.is_none() {
            return None;
        }

        Self::validators()
            .into_iter()
            .find(|v| T::ValidatorIdOf::convert(v.clone()) == who)
    }
}

pub trait SessionInterface<AccountId> {
    fn disable_validator(validator_index: u32) -> bool;
    fn validators() -> Vec<AccountId>;
    fn prune_historical_up_to(up_to: SessionIndex);
    fn is_active_validator(id: KeyTypeId, key_data: &[u8]) -> Option<AccountId>;
}

pub struct ExposureOf<T>(sp_std::marker::PhantomData<T>);

impl<T: Config> Convert<T::AccountId, Option<u128>> for ExposureOf<T> {
    fn convert(validator: T::AccountId) -> Option<u128> {
        <Pallet<T>>::active_era()
            .map(|active_era| <Pallet<T>>::eras_stakers(active_era.index, &validator))
    }
}
