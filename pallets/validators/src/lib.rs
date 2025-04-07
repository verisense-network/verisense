#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
pub mod types;
pub mod weights;

use crate::types::{ActiveEraInfo, Forcing, SessionInterface};
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use pallet_session::historical;
use sp_core::crypto::KeyTypeId;
use sp_runtime::SaturatedConversion;
use sp_staking::{EraIndex, SessionIndex};
use sp_std::vec::Vec;
use vrs_support::{log, EraRewardPoints, RestakingInterface, ValidatorsInterface};
pub use weights::*;

pub(crate) const LOG_TARGET: &'static str = "runtime::pallet-validators";

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use crate::types::{ActiveEraInfo, SessionInterface};
    use frame_support::traits::UnixTime;
    use sp_runtime::Percent;
    use sp_staking::EraIndex;
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_session::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;

        #[pallet::constant]
        type BondingDuration: Get<EraIndex>;

        type UnixTime: UnixTime;

        #[pallet::constant]
        type SessionsPerEra: Get<SessionIndex>;

        type SessionInterface: SessionInterface<Self::AccountId>;

        #[pallet::constant]
        type HistoryDepth: Get<u32>;

        type RestakingInterface: RestakingInterface<Self::AccountId>;
    }

    #[pallet::storage]
    #[pallet::getter(fn eras_validator_reward)]
    pub type ErasValidatorReward<T: Config> = StorageMap<_, Twox64Concat, EraIndex, u128>;

    #[pallet::storage]
    pub type MaxStakedRewards<T> = StorageValue<_, Percent, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn era_payout)]
    pub type EraPayout<T> = StorageValue<_, u128, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn current_planned_session)]
    pub(crate) type CurrentPlannedSession<T> = StorageValue<_, SessionIndex, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn current_era)]
    pub(crate) type CurrentEra<T> = StorageValue<_, EraIndex>;

    #[pallet::storage]
    #[pallet::getter(fn eras_total_stake)]
    pub type ErasTotalStake<T: Config> = StorageMap<_, Twox64Concat, EraIndex, u128, ValueQuery>;

    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn eras_reward_points)]
    pub type ErasRewardPoints<T: Config> =
        StorageMap<_, Twox64Concat, EraIndex, EraRewardPoints<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn bonded_eras)]
    pub(crate) type BondedEras<T: Config> =
        StorageValue<_, Vec<(EraIndex, SessionIndex)>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn active_era)]
    pub type ActiveEra<T> = StorageValue<_, ActiveEraInfo>;

    #[pallet::storage]
    #[pallet::getter(fn eras_start_session_index)]
    pub type ErasStartSessionIndex<T> = StorageMap<_, Twox64Concat, EraIndex, SessionIndex>;

    #[pallet::storage]
    #[pallet::getter(fn eras_stakers)]
    pub(crate) type ErasStakers<T: Config> =
        StorageDoubleMap<_, Twox64Concat, EraIndex, Twox64Concat, T::AccountId, u128, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn force_era)]
    pub type ForceEra<T> = StorageValue<_, Forcing, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ForceEra {
            mode: Forcing,
        },
        TriggerNewEra,
        EraPaid {
            era_index: EraIndex,
            validator_payout: u128,
            staker_payout: u128,
        },
        EraStarted(EraIndex),
    }

    #[pallet::error]
    pub enum Error<T> {}

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_now: BlockNumberFor<T>) -> Weight {
            T::DbWeight::get().reads(1)
        }

        fn on_finalize(_n: BlockNumberFor<T>) {
            if let Some(mut active_era) = Self::active_era() {
                if active_era.start.is_none() {
                    let now_as_millis_u64 = T::UnixTime::now().as_millis().saturated_into::<u64>();
                    active_era.start = Some(now_as_millis_u64);
                    ActiveEra::<T>::put(active_era);
                }
            }
        }
    }
}

impl<T: Config> ValidatorsInterface<T::AccountId> for Pallet<T> {
    fn is_active_validator(id: KeyTypeId, key_data: &[u8]) -> Option<T::AccountId> {
        T::SessionInterface::is_active_validator(id, key_data)
    }

    fn validators() -> Vec<T::AccountId> {
        T::SessionInterface::validators()
    }

    fn active_stake_of(_who: &T::AccountId) -> u128 {
        1u128
    }

    fn active_total_stake() -> Option<u128> {
        Some(1u128)
    }
}

impl<T: Config> Pallet<T> {
    pub fn reward_by_ids(validators_points: impl IntoIterator<Item = (T::AccountId, u128)>) {
        if let Some(active_era) = Self::active_era() {
            <ErasRewardPoints<T>>::mutate(active_era.index, |era_rewards| {
                for (validator, points) in validators_points.into_iter() {
                    *era_rewards.individual.entry(validator).or_default() += points;
                    era_rewards.total += points;
                }
            });
        }
    }

    fn new_session(session_index: SessionIndex, is_genesis: bool) -> Option<Vec<T::AccountId>> {
        if let Some(current_era) = Self::current_era() {
            let current_era_start_session_index = Self::eras_start_session_index(current_era)
                .unwrap_or_else(|| {
                    frame_support::print("Error: start_session_index must be set for current_era");
                    0
                });

            let era_length = session_index.saturating_sub(current_era_start_session_index); // Must never happen.

            log!(info, "Era length: {:?}", era_length);
            if era_length < T::SessionsPerEra::get() {
                // The 5th session of the era.
                if era_length == T::SessionsPerEra::get() - 1 {
                    T::RestakingInterface::plan_new_era();
                }
                return None;
            }
            Self::try_trigger_new_era(session_index, is_genesis)
        } else {
            // Set initial era.
            log!(debug, "Starting the first era.");
            Self::try_trigger_new_era(session_index, is_genesis)
        }
    }

    fn start_session(start_session: SessionIndex) {
        let next_active_era = Self::active_era().map(|e| e.index + 1).unwrap_or(0);
        if let Some(next_active_era_start_session_index) =
            Self::eras_start_session_index(next_active_era)
        {
            if next_active_era_start_session_index == start_session {
                Self::start_era(start_session);
            } else if next_active_era_start_session_index < start_session {
                frame_support::print("Warning: A session appears to have been skipped.");
                Self::start_era(start_session);
            }
        }
    }

    fn end_session(session_index: SessionIndex) -> DispatchResult {
        if let Some(active_era) = Self::active_era() {
            if let Some(next_active_era_start_session_index) =
                Self::eras_start_session_index(active_era.index + 1)
            {
                if next_active_era_start_session_index == session_index + 1 {
                    Self::end_era(active_era, session_index);
                }
            }
        }
        Ok(())
    }

    fn start_era(start_session: SessionIndex) {
        let active_era = ActiveEra::<T>::mutate(|active_era| {
            let next_set_id = T::RestakingInterface::next_validators_set_id();
            let new_index = active_era.as_ref().map(|info| info.index + 1).unwrap_or(0);
            *active_era = Some(ActiveEraInfo {
                index: new_index,
                set_id: next_set_id - 1,
                start: None,
            });
            new_index
        });

        let bonding_duration = T::BondingDuration::get();
        BondedEras::<T>::mutate(|bonded| {
            bonded.push((active_era, start_session));
            if active_era > bonding_duration {
                if let Some(&(_, first_session)) = bonded.first() {
                    T::SessionInterface::prune_historical_up_to(first_session);
                }
            }
        });
        Self::deposit_event(EraStarted(active_era));
    }

    fn end_era(active_era: ActiveEraInfo, _session_index: SessionIndex) {
        let v = Self::eras_reward_points(active_era.index);
        T::RestakingInterface::on_end_era(active_era.index, v);
    }

    fn validators_with_exposure(validators: Vec<T::AccountId>) -> Vec<(T::AccountId, u128)> {
        let current_era = Self::current_era()
            // Must be some as a new era has been created.
            .unwrap_or(0);

        validators
            .into_iter()
            .map(|v| {
                let exposure = Self::eras_stakers(current_era, &v);
                (v, exposure)
            })
            .collect()
    }

    pub(crate) fn try_trigger_new_era(
        start_session_index: SessionIndex,
        _is_genesis: bool,
    ) -> Option<Vec<T::AccountId>> {
        let validators = T::RestakingInterface::provide();
        Self::deposit_event(Event::TriggerNewEra);
        Some(Self::trigger_new_era(start_session_index, validators))
    }

    #[allow(deprecated)]
    pub fn clear_era_information(era_index: EraIndex) {
        <ErasStakers<T>>::remove_prefix(era_index, None);
        <ErasValidatorReward<T>>::remove(era_index);
        <ErasRewardPoints<T>>::remove(era_index);
        <ErasTotalStake<T>>::remove(era_index);
        ErasStartSessionIndex::<T>::remove(era_index);
    }

    pub fn store_stakers_info(
        validators: Vec<(T::AccountId, u128)>,
        new_planned_era: EraIndex,
    ) -> Vec<T::AccountId> {
        let elected_stashes = validators
            .iter()
            .cloned()
            .map(|(x, _)| x)
            .collect::<Vec<_>>();
        let mut total_stake: u128 = 0;
        validators.into_iter().for_each(|(who, weight)| {
            total_stake = total_stake.saturating_add(weight);
            <ErasStakers<T>>::insert(new_planned_era, &who, weight);
        });
        <ErasTotalStake<T>>::insert(&new_planned_era, total_stake);
        if new_planned_era > 0 {
            log!(
                info,
                "New validator set of size {:?} has been processed for era {:?}",
                elected_stashes.len(),
                new_planned_era,
            );
        }
        elected_stashes
    }

    pub fn trigger_new_era(
        start_session_index: SessionIndex,
        exposures: Vec<(T::AccountId, u128)>,
    ) -> Vec<T::AccountId> {
        let new_planned_era = CurrentEra::<T>::mutate(|s| {
            *s = Some(s.map(|s| s + 1).unwrap_or(0));
            s.unwrap()
        });
        ErasStartSessionIndex::<T>::insert(&new_planned_era, &start_session_index);
        if let Some(old_era) = new_planned_era.checked_sub(T::HistoryDepth::get() + 1) {
            Self::clear_era_information(old_era);
        }
        Self::store_stakers_info(exposures, new_planned_era)
    }
}

impl<T: Config> pallet_session::SessionManager<T::AccountId> for Pallet<T> {
    fn new_session(new_index: SessionIndex) -> Option<Vec<T::AccountId>> {
        log!(info, "planning new session {}", new_index);
        CurrentPlannedSession::<T>::put(new_index);
        Self::new_session(new_index, false)
    }

    fn new_session_genesis(new_index: SessionIndex) -> Option<Vec<T::AccountId>> {
        log!(info, "planning new session {} at genesis", new_index);
        CurrentPlannedSession::<T>::put(new_index);
        Self::new_session(new_index, true)
    }

    fn end_session(end_index: SessionIndex) {
        log!(info, "ending session {}", end_index);
        match Self::end_session(end_index) {
            Ok(_) => {}
            Err(e) => {
                log!(error, "ending session failed error({:?})", e);
            }
        }
    }

    fn start_session(start_index: SessionIndex) {
        log!(info, "starting session {}", start_index);
        Self::start_session(start_index)
    }
}

impl<T: Config> historical::SessionManager<T::AccountId, u128> for Pallet<T> {
    fn new_session(new_index: SessionIndex) -> Option<Vec<(T::AccountId, u128)>> {
        <Self as pallet_session::SessionManager<_>>::new_session(new_index)
            .map(|validators| Self::validators_with_exposure(validators))
    }

    fn new_session_genesis(new_index: SessionIndex) -> Option<Vec<(T::AccountId, u128)>> {
        <Self as pallet_session::SessionManager<_>>::new_session_genesis(new_index)
            .map(|validators| Self::validators_with_exposure(validators))
    }

    fn start_session(start_index: SessionIndex) {
        <Self as pallet_session::SessionManager<_>>::start_session(start_index)
    }

    fn end_session(end_index: SessionIndex) {
        <Self as pallet_session::SessionManager<_>>::end_session(end_index)
    }
}

use crate::Event::EraStarted;
use sp_std::vec;

impl<T> pallet_authorship::EventHandler<T::AccountId, BlockNumberFor<T>> for Pallet<T>
where
    T: Config + pallet_authorship::Config + pallet_session::Config,
{
    fn note_author(author: T::AccountId) {
        Self::reward_by_ids(vec![(author, 1u128)])
    }
}
