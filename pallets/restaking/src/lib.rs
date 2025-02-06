#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    dispatch::{GetDispatchInfo, PostDispatchInfo},
    traits::{OneSessionHandler},
};
use frame_system::offchain::{
    AppCrypto, CreateSignedTransaction, SendUnsignedTransaction, SignedPayload, Signer,
    SigningTypes,
};
use scale_info::{
    prelude::string::{String, ToString},
    TypeInfo,
};
use serde::{de, Deserialize, Deserializer};
use sp_runtime::{
    RuntimeAppPublic,
    RuntimeDebug, traits::{Dispatchable, IdentifyAccount},
};
use sp_std::prelude::*;
use crate::types::NotificationResult;
pub use pallet::*;
use types::{Observation, ObservationsPayload, ObservationType};
use vrs_primitives::keys::RESTAKING_KEY_TYPE as KEY_TYPE;
use vrs_support::{log, ValidatorsInterface};

mod outchain;
pub(crate) mod solidity;
pub mod types;
mod merkle;

pub(crate) const LOG_TARGET: &'static str = "runtime::restaking";

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use vrs_support::{EraRewardPoints, RestakingInterface};

    use super::*;

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

        #[pallet::constant]
        type RequestEventLimit: Get<u32>;

        #[pallet::constant]
        type MaxValidators: Get<u32>;

        #[pallet::constant]
        type RestakingEnable: Get<bool>;

        type ValidatorsInterface: ValidatorsInterface<Self::AccountId>;

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
    pub(crate) type RewardsAmountPerPoint<T: Config> = StorageValue<_, u128, ValueQuery,DefaultRewardsPerPoint<T>>;

    #[pallet::storage]
    pub(crate) type NextSetId<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::unbounded]
    pub(crate) type PlannedValidators<T: Config> =
        StorageValue<_, Vec<(T::AccountId, u128)>, ValueQuery>;

    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn validator_source)]
    pub(crate) type ValidatorsSource<T: Config> =
    StorageMap<_,Twox64Concat, T::AccountId, (String,String), ValueQuery>; //EvmAddr, restaking platform

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
    #[pallet::unbounded]
    pub(crate) type Observing<T: Config> = StorageMap<
        _,
        Twox64Concat,
        Observation<T::AccountId>,
        BoundedVec<T::AccountId, T::MaxValidators>,
        ValueQuery,
    >;

    #[pallet::storage]
    pub(crate) type NeedFetchRestakingValidators<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::storage]
    pub(crate) type LatestClosedEra<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    pub(crate) type EraTotalRewards<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128, ValueQuery>;

    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn total_rewards)]
    pub(crate) type TotalRewards<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u128, ValueQuery>;

    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn restaking_platform)]
    pub(crate) type RestakingPlatform<T: Config> = StorageMap<_, Blake2_128Concat, String, (String, String), OptionQuery>;

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
        Simple,
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
    }

    impl<T: Config> sp_runtime::BoundToRuntimeAppPublic for Pallet<T> {
        type Public = T::AuthorityId;
    }

    impl<T: Config> RestakingInterface<T::AccountId> for Pallet<T> {
        fn provide() -> Vec<(T::AccountId, u128)> {
            PlannedValidators::<T>::get()
        }

        fn next_validators_set_id() -> u32 {
            NextSetId::<T>::get()
        }

        fn plan_new_era() {
            NeedFetchRestakingValidators::<T>::put(true);
        }

        fn on_end_era(era_idx: u32, era_reward_points: EraRewardPoints<T::AccountId>) {
            let reward_per_point = Self::rewards_per_point();
            let mut total_era_rewards = 0u128;
            for (acc, point) in era_reward_points.individual {
                let rewds = point * reward_per_point;
                total_era_rewards += rewds;
                TotalRewards::<T>::mutate(acc, |r| {
                     *r += rewds;
                });
            };
            EraTotalRewards::<T>::insert(era_idx, total_era_rewards);
            LatestClosedEra::<T>::put(era_idx);
            Self::calculate_rewards_root();
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
            <NextSetId<T>>::put(1); // set 0 is already in the genesis
            let mut validators = vec![];
            for v in self.validators.clone() {
                validators.push((v.0.clone(), v.1));
                <ValidatorsSource<T>>::insert(v.0,(v.2, v.3));
            }
            <PlannedValidators<T>>::put(validators);
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
            payload: ObservationsPayload<T::AccountId, T::Public, BlockNumberFor<T>>,
            _signature: T::Signature,
        ) -> DispatchResultWithPostInfo {
            ensure_none(origin)?;
            let val_id = T::ValidatorsInterface::is_active_validator(KEY_TYPE, &payload.key_data);
            if val_id.is_none() {
                log!(
                    warn,
                    "Not a validator in current validator set, key_data: {:?}",
                    payload.key_data
                );
                return Err(Error::<T>::NotValidator.into());
            }
            let validators_with_source = payload.observations;
            let mut validators = vec![];
            for x in validators_with_source {
                ValidatorsSource::<T>::insert(x.0.clone(),(x.2, x.3));
                validators.push((x.0, x.1));
            }
            PlannedValidators::<T>::put(validators);
            NeedFetchRestakingValidators::<T>::put(false);
            Self::deposit_event(Event::Simple);
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
                let val_id = T::ValidatorsInterface::is_active_validator(KEY_TYPE, &key_data);
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
