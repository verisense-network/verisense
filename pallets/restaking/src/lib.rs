#![cfg_attr(not(feature = "std"), no_std)]

use crate::types::EraRewardDetailsValue;
use crate::types::NotificationResult;
use crate::types::OperatorReward;
use codec::{Decode, Encode};
use frame_support::{
    dispatch::{GetDispatchInfo, PostDispatchInfo},
    traits::OneSessionHandler,
};
use frame_system::offchain::{
    AppCrypto, CreateSignedTransaction, SendUnsignedTransaction, SignedPayload, Signer,
    SigningTypes,
};
pub use pallet::*;
use scale_info::{
    prelude::string::{String, ToString},
    TypeInfo,
};
use serde::{de, Deserialize, Deserializer};
use sp_runtime::{
    traits::{Dispatchable, IdentifyAccount},
    RuntimeAppPublic, RuntimeDebug,
};
use sp_std::prelude::*;
use types::{Observation, ObservationType, ObservationsPayload};
use vrs_primitives::keys::RESTAKING_KEY_TYPE as KEY_TYPE;
use vrs_support::{log, ValidatorsInterface};

mod outchain;
pub(crate) mod solidity;
pub mod types;
pub mod validator_data;
pub type ValidatorSource = String;
pub(crate) const LOG_TARGET: &'static str = "runtime::restaking";
#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use crate::validator_data::ValidatorData;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::{Currency, UnixTime};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::CheckedConversion;
    use vrs_support::consts::ORIGINAL_VALIDATOR_SOURCE;
    use vrs_support::{EraRewardPoints, RestakingInterface};

    #[pallet::config]
    pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
        type AuthorityId: Member + Parameter + RuntimeAppPublic + MaybeSerializeDeserialize;

        type AppCrypto: AppCrypto<Self::Public, Self::Signature>;

        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type RuntimeCall: Parameter
            + Dispatchable<RuntimeOrigin = Self::RuntimeOrigin, PostInfo = PostDispatchInfo>
            + GetDispatchInfo
            + From<frame_system::Call<Self>>;

        #[pallet::constant]
        type UnsignedPriority: Get<TransactionPriority>;

        type UnixTime: UnixTime;

        #[pallet::constant]
        type RequestEventLimit: Get<u32>;

        #[pallet::constant]
        type MaxValidators: Get<u32>;

        #[pallet::constant]
        type RestakingEnable: Get<bool>;

        type ValidatorsInterface: ValidatorsInterface<Self::AccountId>;

        type Currency: Currency<Self::AccountId>;
    }

    type MaxObservations = ConstU32<100>;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::unbounded]
    pub(crate) type IsActivated<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::type_value]
    pub fn DefaultRewardsPerPoint<T: Config>() -> u128 {
        1u128
    }

    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn rewards_per_point)]
    pub(crate) type RewardsAmountPerPoint<T: Config> =
        StorageValue<_, u128, ValueQuery, DefaultRewardsPerPoint<T>>;

    #[pallet::storage]
    pub(crate) type NextSetId<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::unbounded]
    pub(crate) type PlannedValidators<T: Config> =
        StorageValue<_, Vec<(T::AccountId, u128, ValidatorSource)>, ValueQuery>;

    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn validator_source)]
    pub(crate) type ValidatorsSource<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, ValidatorData, ValueQuery>; //EvmAddr, restaking platform

    #[pallet::storage]
    pub(crate) type NextNotificationId<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::unbounded]
    pub(crate) type Observations<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        ObservationType,
        Twox64Concat,
        u32,
        BoundedVec<Observation<T::AccountId>, MaxObservations>,
        ValueQuery,
    >;

    #[pallet::storage]
    pub(crate) type NeedFetchRestakingValidators<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::storage]
    pub(crate) type LatestClosedEra<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::unbounded]
    pub(crate) type EraRewardsDetail<T: Config> =
        StorageMap<_, Blake2_128Concat, u32, EraRewardDetailsValue, ValueQuery>;

    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn total_rewards)]
    pub(crate) type TotalRewards<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, u128, ValueQuery>;

    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn restaking_platform)]
    pub(crate) type RestakingPlatform<T: Config> =
        StorageMap<_, Blake2_128Concat, String, (String, String), OptionQuery>;

    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn rewards_root)]
    pub(crate) type RewardsRoot<T: Config> = StorageValue<_, String, ValueQuery>;

    #[pallet::storage]
    #[pallet::unbounded]
    pub(crate) type NotificationHistory<T: Config> =
        StorageMap<_, Twox64Concat, u32, Option<NotificationResult>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        NewPlannedValidators {
            set_id: u32,
            validators: Vec<(T::AccountId, u128)>,
        },
        UnlockFailed {
            sender: Vec<u8>,
            receiver: T::AccountId,
            amount: u128,
            sequence: u32,
        },
        MintNep141Failed {
            token_id: Vec<u8>,
            sender: Vec<u8>,
            receiver: T::AccountId,
            amount: u128,
            sequence: u32,
        },
        UnlockNonfungibleFailed {
            collection: u128,
            item: u128,
            sender: Vec<u8>,
            receiver: T::AccountId,
            sequence: u32,
        },
        RewardsPerPointUpdated {
            value: u128,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        WrongSetId,
        InvalidNotificationId,
        NotValidator,
        NextNotificationIdOverflow,
        InvalidActiveTotalStake,
        NotActivated,
        InvalidReceiverId,
        NextSetIdOverflow,
        ObservationsExceededLimit,
        InvalidRewardsPerPoint,
    }

    impl<T: Config> sp_runtime::BoundToRuntimeAppPublic for Pallet<T> {
        type Public = T::AuthorityId;
    }

    impl<T: Config> RestakingInterface<T::AccountId> for Pallet<T> {
        fn provide() -> Vec<(T::AccountId, u128)> {
            PlannedValidators::<T>::get()
                .iter()
                .map(|s| (s.0.clone(), s.1))
                .collect()
        }

        fn next_validators_set_id() -> u32 {
            NextSetId::<T>::get()
        }

        fn plan_new_era() {
            NeedFetchRestakingValidators::<T>::put(true);
        }

        fn on_end_era(era_idx: u32, era_reward_points: EraRewardPoints<T::AccountId>) {
            let reward_per_point = Self::rewards_per_point();
            let mut rewards = EraRewardDetailsValue::default();
            rewards.timestamp = T::UnixTime::now().as_secs();
            for (acc, point) in era_reward_points.individual {
                let rewds = point * reward_per_point;
                TotalRewards::<T>::mutate(acc.clone(), |r| {
                    *r += rewds;
                });
                let source = Self::validator_source(&acc);
                rewards.total += rewds;
                rewards.details.push(OperatorReward {
                    validator: source,
                    amount: rewds,
                });
                let _ =
                    T::Currency::deposit_creating(&acc, rewds.checked_into().unwrap_or_default());
            }
            EraRewardsDetail::<T>::insert(era_idx, rewards);
            LatestClosedEra::<T>::put(era_idx);
        }
    }

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        pub validators: Vec<(T::AccountId, u128, String, String)>, //AccountId, total_staking, evm_addr, platform
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            use crate::validator_data::ValidatorData;
            <NextSetId<T>>::put(1); // set 0 is already in the genesis
            let mut planned_validators = vec![];
            for v in self.validators.clone() {
                let mut substrate_key = [0u8; 32];
                substrate_key.copy_from_slice(v.0.clone().encode().as_slice());
                let validator = ValidatorData {
                    operator: [0u8; 20],
                    stake: 0,
                    key: v.0.clone().encode(),
                    strategies: vec![],
                    source: v.3.clone(),
                };
                <ValidatorsSource<T>>::insert(v.0.clone(), validator);
                planned_validators.push((v.0, v.1, v.3.clone()));
            }
            <PlannedValidators<T>>::put(planned_validators);
        }
    }

    impl<T: Config> OneSessionHandler<T::AccountId> for Pallet<T> {
        type Key = T::AuthorityId;

        fn on_genesis_session<'a, I: 'a>(_authorities: I)
        where
            I: Iterator<Item = (&'a T::AccountId, Self::Key)>,
        {
            // ignore
        }

        fn on_new_session<'a, I: 'a>(_changed: bool, _validators: I, _queued_validators: I)
        where
            I: Iterator<Item = (&'a T::AccountId, Self::Key)>,
        {
            // ignore
        }

        fn on_disabled(_i: u32) {
            // ignore
        }
    }

    // TODO ignore on devnet
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(block_number: BlockNumberFor<T>) {
            if !T::RestakingEnable::get() {
                return;
            }
            if !NeedFetchRestakingValidators::<T>::get() {
                return;
            }
            if !sp_io::offchain::is_validator() {
                return;
            }
            if let Some((public, key_data, validator_id)) = Self::get_validator_id() {
                Self::submit_unsigned_transaction(block_number, public, key_data, validator_id)
                    .unwrap();
            }
        }
    }

    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;
        fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
            if let Call::update_validators {
                ref payload,
                ref signature,
            } = call
            {
                let signature_valid =
                    SignedPayload::<T>::verify::<T::AppCrypto>(payload, signature.clone());
                if !signature_valid {
                    return InvalidTransaction::BadProof.into();
                }
                Self::validate_transaction_parameters(
                    &payload.block_number,
                    payload.public.clone().into_account(),
                )
            } else {
                InvalidTransaction::Call.into()
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1)]
        #[pallet::call_index(0)]
        pub fn update_validators(
            origin: OriginFor<T>,
            payload: ObservationsPayload<T::Public, BlockNumberFor<T>>,
            _signature: T::Signature,
        ) -> DispatchResultWithPostInfo {
            ensure_none(origin)?;
            let val_id =
                T::ValidatorsInterface::lookup_active_validator(KEY_TYPE, &payload.key_data);
            if val_id.is_none() {
                log!(
                    warn,
                    "Not a validator in current validator set, key_data: {:?}",
                    payload.key_data
                );
                return Err(Error::<T>::NotValidator.into());
            }
            let mut validators_with_source = payload.observations;
            validators_with_source.sort_by(|a, b| b.stake.cmp(&a.stake));
            use sp_runtime::traits::TrailingZeroInput;
            let mut planned_validators = PlannedValidators::<T>::get()
                .iter()
                .filter(|s| s.2 == ORIGINAL_VALIDATOR_SOURCE)
                .cloned()
                .collect::<Vec<(T::AccountId, u128, String)>>();
            let max_validators_size = T::MaxValidators::get();
            for x in validators_with_source.clone() {
                let operator_account =
                    T::AccountId::decode(&mut TrailingZeroInput::new(x.key.as_slice())).unwrap();
                ValidatorsSource::<T>::insert(operator_account.clone(), x.clone());
                planned_validators.push((operator_account, x.stake, x.source.clone()));
                if planned_validators.len() >= max_validators_size as usize {
                    break;
                }
            }
            PlannedValidators::<T>::put(planned_validators);
            NeedFetchRestakingValidators::<T>::put(false);
            Ok(().into())
        }

        #[pallet::weight(1)]
        #[pallet::call_index(1)]
        pub fn add_restaking_platform(
            origin: OriginFor<T>,
            platform_source_name: String,
            url: String,
            middleware_address: String,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            RestakingPlatform::<T>::insert(platform_source_name, (url, middleware_address));
            Ok(().into())
        }

        #[pallet::weight(1)]
        #[pallet::call_index(2)]
        pub fn set_rewards_pre_point(
            origin: OriginFor<T>,
            value: u128,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            ensure!(value > 0, Error::<T>::InvalidRewardsPerPoint);
            RewardsAmountPerPoint::<T>::put(value);
            Self::deposit_event(Event::RewardsPerPointUpdated { value });
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        fn get_validator_id() -> Option<(<T as SigningTypes>::Public, Vec<u8>, T::AccountId)> {
            for key in <T::AppCrypto as AppCrypto<
                <T as SigningTypes>::Public,
                <T as SigningTypes>::Signature,
            >>::RuntimeAppPublic::all()
            .into_iter()
            {
                let key_data = key.to_raw_vec();
                let val_id = T::ValidatorsInterface::lookup_active_validator(KEY_TYPE, &key_data);
                if val_id.is_none() {
                    continue;
                }
                let generic_public = <T::AppCrypto as AppCrypto<
                    <T as SigningTypes>::Public,
                    <T as SigningTypes>::Signature,
                >>::GenericPublic::from(key);
                let public: <T as SigningTypes>::Public = generic_public.into();
                return Some((public, key_data, val_id.unwrap()));
            }
            None
        }

        fn validate_transaction_parameters(
            block_number: &BlockNumberFor<T>,
            account_id: <T as frame_system::Config>::AccountId,
        ) -> TransactionValidity {
            let current_block = <frame_system::Pallet<T>>::block_number();
            if &current_block < block_number {
                log!(
                    warn,
                    "InvalidTransaction => current_block: {:?}, block_number: {:?}",
                    current_block,
                    block_number
                );
                return InvalidTransaction::Future.into();
            }
            ValidTransaction::with_tag_prefix("Restaking")
                .priority(T::UnsignedPriority::get())
                .and_provides(account_id)
                .longevity(5)
                .propagate(true)
                .build()
        }
    }
}
