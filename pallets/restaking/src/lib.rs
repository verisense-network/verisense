#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    dispatch::{GetDispatchInfo, PostDispatchInfo},
    traits::{ConstU32, OneSessionHandler},
    BoundedVec,
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
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
    traits::{Dispatchable, IdentifyAccount},
    RuntimeAppPublic, RuntimeDebug,
};
use sp_std::prelude::*;
use types::{
    AppchainNotification, NotificationResult, Observation, ObservationType, ObservationsPayload,
    Validator, ValidatorSet,
};
use verisense_support::{log, ValidatorsInterface};
use vrs_primitives::keys::RESTAKING_KEY_TYPE as KEY_TYPE;

mod outchain;
pub(crate) mod solidity;
pub mod types;

pub(crate) const LOG_TARGET: &'static str = "runtime::restaking";

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use verisense_support::RestakingInterface;

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

        type ValidatorsInterface: ValidatorsInterface<Self::AccountId>;
    }

    type MaxObservations = ConstU32<100>;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::unbounded]
    pub(crate) type IsActivated<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::storage]
    pub(crate) type NextSetId<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::unbounded]
    pub(crate) type PlannedValidators<T: Config> =
        StorageValue<_, Vec<(T::AccountId, u128)>, ValueQuery>;

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
            <PlannedValidators<T>>::get()
        }

        fn next_validators_set_id() -> u32 {
            NextSetId::<T>::get()
        }

        fn plan_new_era() {
            NeedFetchRestakingValidators::<T>::put(true);
        }
    }

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        pub validators: Vec<(T::AccountId, u128)>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            <NextSetId<T>>::put(1); // set 0 is already in the genesis
            <PlannedValidators<T>>::put(self.validators.clone());
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
    // #[pallet::hooks]
    // impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    //     fn offchain_worker(block_number: BlockNumberFor<T>) {
    //         if !NeedFetchRestakingValidators::<T>::get() {
    //             return;
    //         }
    //         if !sp_io::offchain::is_validator() {
    //             return;
    //         }
    //         if let Some((public, key_data, validator_id)) = Self::get_validator_id() {
    //             Self::submit_unsigned_transaction(block_number, public, key_data, validator_id)
    //                 .unwrap();
    //         }
    //     }
    // }

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
            let r = payload.observations;
            PlannedValidators::<T>::put(r);
            NeedFetchRestakingValidators::<T>::put(false);
            Self::deposit_event(Event::Simple);
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
