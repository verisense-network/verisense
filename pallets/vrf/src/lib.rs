#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// #[cfg(test)]
// mod mock;
// #[cfg(test)]
// mod tests;

pub mod constants;
use constants::VRF_PERIOD;
pub mod weights;
use frame_support::{
    traits::{DisabledValidators, FindAuthor, Get, OnTimestampSet, OneSessionHandler},
    BoundedSlice, BoundedVec, ConsensusEngineId, Parameter,
};
use scale_info::TypeInfo;
use sp_core::crypto::KeyTypeId;
use sp_core::hashing::{blake2_256, keccak_256, sha2_256};
pub use sp_core::sr25519::*;
use sp_runtime::traits::SaturatedConversion;
use sp_runtime::traits::UniqueSaturatedInto;
use sp_runtime::traits::Verify;
use sp_runtime::{
    traits::{Dispatchable, IdentifyAccount},
    RuntimeAppPublic, RuntimeDebug,
};
use sr25519::VrfSignature;
use verisense_support::{log, ValidatorsInterface};
pub use weights::*;
pub(crate) const LOG_TARGET: &'static str = "runtime::vrf";

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"vrf!");

pub mod sr25519 {
    mod app_sr25519 {
        use sp_runtime::app_crypto::{app_crypto, sr25519};

        use super::super::KEY_TYPE;
        use scale_info::prelude::string::String;
        app_crypto!(sr25519, KEY_TYPE);
    }

    sp_application_crypto::with_pair! {
        pub type VrfPair = app_sr25519::Pair;
    }

    pub type VrfSignature = app_sr25519::Signature;

    pub type VrfId = app_sr25519::Public;
}
use scale_info::prelude::cmp::Ordering;
#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::{ValidatorRegistration, ValidatorSet};
    use frame_system::pallet_prelude::*;
    use parity_scale_codec::Codec;
    use sp_application_crypto::AppSignature;
    use sp_core::crypto::{VrfCrypto, VrfPublic};
    use sp_core::storage::StorageChangeSet;
    use sp_runtime::codec::{Decode, Encode};
    use sp_runtime::traits::{Hash, LookupError, MaybeDisplay, StaticLookup};
    use sp_std::prelude::*;
    use verisense_support::{ValidatorsInterface, VrfInterface};
    use vrs_primitives::{AppCryptoApproach, CryptoApproach, SeedsInfo};

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Default, TypeInfo, Debug)]
    pub struct NucleusEquation<AccountId, Hash, NodeId> {
        pub name: Vec<u8>,
        pub manager: AccountId,
        pub wasm_hash: Hash,
        pub wasm_version: u32,
        pub wasm_location: Option<NodeId>,
        pub energy: u128,
        pub current_event: u64,
        pub root_state: Hash,
        pub capacity: u8,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);
    impl<T: Config> sp_runtime::BoundToRuntimeAppPublic for Pallet<T> {
        type Public = T::VrfId;
    }

    impl<T: Config> OneSessionHandler<T::AccountId> for Pallet<T> {
        type Key = T::VrfId;

        fn on_genesis_session<'a, I: 'a>(validators: I)
        where
            I: Iterator<Item = (&'a T::AccountId, Self::Key)>,
        {
            // let authorities = validators.map(|(_, k)| k).collect::<Vec<_>>();
            // Self::initialize_authorities(&authorities);
        }

        fn on_new_session<'a, I: 'a>(changed: bool, validators: I, _queued_validators: I)
        where
            I: Iterator<Item = (&'a T::AccountId, Self::Key)>,
        {
            // // instant changes
            // if changed {
            //     let next_authorities = validators.map(|(_, k)| k).collect::<Vec<_>>();
            //     let last_authorities = Authorities::<T>::get();
            //     if last_authorities != next_authorities {
            //         if next_authorities.len() as u32 > T::MaxAuthorities::get() {
            //             log::warn!(
            //                 target: LOG_TARGET,
            //                 "next authorities list larger than {}, truncating",
            //                 T::MaxAuthorities::get(),
            //             );
            //         }
            //         let bounded =
            //             <BoundedVec<_, T::MaxAuthorities>>::truncate_from(next_authorities);
            //         Self::change_authorities(bounded);
            //     }
            // }
        }

        fn on_disabled(i: u32) {
            // let log = DigestItem::Consensus(
            //     AURA_ENGINE_ID,
            //     ConsensusLog::<T::AuthorityId>::OnDisabled(i as AuthorityIndex).encode(),
            // );

            // <frame_system::Pallet<T>>::deposit_log(log);
        }
    }
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type WeightInfo: WeightInfo;
        type Validators: ValidatorsInterface<Self::AccountId>;

        type NucleusId: Parameter
            + Member
            + MaybeSerializeDeserialize
            + core::fmt::Debug
            + MaybeDisplay
            + MaxEncodedLen;

        type VrfId: Member
            + Parameter
            + RuntimeAppPublic<Signature = Self::VrfSignature>
            + MaybeSerializeDeserialize;
        type VrfSignature: Member + Parameter + AppSignature + Codec;
    }

    #[pallet::storage]
    #[pallet::unbounded]
    pub type VRFSeeds<T: Config> = StorageDoubleMap<
        Hasher1 = Blake2_128Concat,
        Hasher2 = Blake2_128Concat,
        Key1 = T::NucleusId,
        Key2 = T::AccountId,
        Value = (Vec<u8>, T::VrfId),
        QueryKind = OptionQuery,
    >;
    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn active_nucleus)]
    pub(crate) type ActiveNucleus<T: Config> =
        StorageValue<_, Vec<(T::NucleusId, BlockNumberFor<T>)>, ValueQuery>;

    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn active_nucleus_map)]
    pub type ActiveNucleusMap<T: Config> = StorageMap<
        Hasher = Blake2_128Concat,
        Key = T::NucleusId,
        Value = (BlockNumberFor<T>, u32, Vec<T::AccountId>),
        QueryKind = OptionQuery,
    >;
    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn nucleus_validators)]
    pub type NucleusValidators<T: Config> = StorageMap<
        Hasher = Blake2_128Concat,
        Key = T::NucleusId,
        Value = Vec<T::AccountId>,
        QueryKind = OptionQuery,
    >;

    #[pallet::error]
    pub enum Error<T> {
        NotValidator,
        VrfTimeExpired,
        NoNucleusId,
        VerificationNotPass,
    }
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        VRFSeedsUpdated {
            nucleus_id: T::NucleusId,
            account_id: T::AccountId,
            vrf_id: T::VrfId,
            seed: Vec<u8>,
        },
    }

    // #[pallet::error]
    // pub enum Error<T> {
    //     NucleusIdAlreadyExists,
    //     NucleusNotFound,
    //     NotAuthorized,
    // }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::update_seed())]
        pub fn register_vrf(
            origin: OriginFor<T>,
            nucleus_id: T::NucleusId,
            vrf_id: T::VrfId,
            seed: Vec<u8>,
            signature: T::VrfSignature,
        ) -> DispatchResult {
            let controller = ensure_signed(origin)?;
            if !Self::is_validator(&controller) {
                return Err(Error::<T>::NotValidator.into());
            }
            let (start_bn_n, vn, mut account_list) =
                ActiveNucleusMap::<T>::get(&nucleus_id.clone()).ok_or(Error::<T>::NoNucleusId)?;
            let start_bn: u64 = start_bn_n.saturated_into();
            let now_bn: u64 = <frame_system::Pallet<T>>::block_number().saturated_into();
            if now_bn > start_bn + VRF_PERIOD {
                return Err(Error::<T>::VrfTimeExpired.into());
            }
            if vrf_id.verify(&seed, &signature) {
                log::info!(
                    target: LOG_TARGET,
                    "VRF seed updated for nucleus_id: {:?}, account_id: {:?}",
                    nucleus_id.clone(),
                    controller.clone()
                );
            } else {
                log::error!(
                    target: LOG_TARGET,
                    "VRF seed update failed for nucleus_id: {:?}, account_id: {:?}",
                    nucleus_id.clone(),
                    controller.clone()
                );
                return Err(Error::<T>::VerificationNotPass.into());
            }
            VRFSeeds::<T>::insert(
                &nucleus_id.clone(),
                &controller.clone(),
                (seed.clone(), vrf_id.clone()),
            );
            Self::deposit_event(Event::VRFSeedsUpdated {
                nucleus_id: nucleus_id.clone(),
                account_id: controller.clone(),
                vrf_id: vrf_id.clone(),

                seed: seed.clone(),
            });
            account_list.push(controller.clone());
            ActiveNucleusMap::<T>::insert(nucleus_id.clone(), (start_bn_n, vn, account_list));
            Ok(())
        }
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::update_seed())]
        pub fn register_vrf_25519(
            origin: OriginFor<T>,
            nucleus_id: T::NucleusId,
            vrf_id: T::VrfId,
            vrf_public: sp_core::sr25519::Public,
            vrf_signature: sp_core::sr25519::vrf::VrfSignature,
        ) -> DispatchResult {
            let controller = ensure_signed(origin)?;
            if !Self::is_validator(&controller) {
                return Err(Error::<T>::NotValidator.into());
            }
            let (start_bn_n, vn, mut account_list) =
                ActiveNucleusMap::<T>::get(&nucleus_id.clone()).ok_or(Error::<T>::NoNucleusId)?;
            let start_bn: u64 = start_bn_n.saturated_into();
            let now_bn: u64 = <frame_system::Pallet<T>>::block_number().saturated_into();
            if now_bn > start_bn + VRF_PERIOD {
                return Err(Error::<T>::VrfTimeExpired.into());
            }
            let transcript = sp_core::sr25519::vrf::VrfTranscript::new(
                b"nucleus",
                &[(b"nucleus_id", nucleus_id.encode().as_slice())],
            )
            .into_sign_data();
            if !vrf_public.vrf_verify(&transcript, &vrf_signature) {
                return Err(Error::<T>::VerificationNotPass.into());
            }
            VRFSeeds::<T>::insert(
                &nucleus_id.clone(),
                &controller.clone(),
                (vrf_signature.encode(), vrf_id.clone()),
            );
            Self::deposit_event(Event::VRFSeedsUpdated {
                nucleus_id: nucleus_id.clone(),
                account_id: controller.clone(),
                vrf_id: vrf_id.clone(),

                seed: vrf_signature.encode().clone(),
            });
            account_list.push(controller.clone());
            ActiveNucleusMap::<T>::insert(nucleus_id.clone(), (start_bn_n, vn, account_list));
            Ok(())
        }
    }
    impl<T: Config> VrfInterface<T::NucleusId, BlockNumberFor<T>, T::AccountId> for Pallet<T> {
        fn register_nucleus_blocknumber(
            nucleus_id: T::NucleusId,
            block_number: BlockNumberFor<T>,
            validators_num: u32,
        ) -> Result<(), DispatchError> {
            let mut active_nucleus = ActiveNucleus::<T>::get();
            active_nucleus.push((nucleus_id.clone(), block_number));
            active_nucleus.sort_by_key(|x| x.1);
            ActiveNucleusMap::<T>::insert(
                nucleus_id.clone(),
                (block_number, validators_num, Vec::<T::AccountId>::new()),
            );
            ActiveNucleus::<T>::put(active_nucleus);
            Ok(())
        }
        fn get_validators(nucleus_id: &T::NucleusId) -> Option<Vec<T::AccountId>> {
            NucleusValidators::<T>::get(nucleus_id)
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn get_seeds_info(
            nucleus_id: &T::NucleusId,
            account_id: &T::AccountId,
        ) -> Option<SeedsInfo<T::AccountId, T::NucleusId, T::VrfId>> {
            if let Some((seed, vrf_id)) =
                VRFSeeds::<T>::get(&nucleus_id.clone(), &account_id.clone())
            {
                return Some(SeedsInfo {
                    nucleus_id: nucleus_id.clone(),
                    account_id: account_id.clone(),
                    seed,
                    vrf_id: vrf_id.clone(),
                });
            }
            return None;
        }
        fn is_validator(account_id: &T::AccountId) -> bool {
            T::Validators::validators().iter().any(|x| x == account_id)
        }
    }
    fn xor(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
        let mut result = [0u8; 32];
        for i in 0..32 {
            result[i] = a[i] ^ b[i];
        }
        result
    }
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_now: BlockNumberFor<T>) -> Weight {
            Weight::default()
        }

        fn on_finalize(now: BlockNumberFor<T>) {
            let mut active_nucleus = ActiveNucleus::<T>::get();
            while active_nucleus.len() > 0 {
                let (nucleus_id, start_bn) = active_nucleus[0].clone();
                let now_bn: u64 = now.saturated_into();
                let start_bn: u64 = start_bn.saturated_into();
                if now_bn >= start_bn + VRF_PERIOD {
                    active_nucleus.remove(0);
                    let (_, vn, account_list) = ActiveNucleusMap::<T>::get(&nucleus_id).unwrap();
                    let hash_total = account_list.iter().fold([0u8; 32], |acc, x| {
                        if let Some((seed, vrf_id)) = VRFSeeds::<T>::get(&nucleus_id, x) {
                            let hash = sha2_256(seed.as_slice());
                            xor(&acc, &hash)
                        } else {
                            acc
                        }
                    });
                    let mut result = account_list
                        .iter()
                        .map(|x| {
                            if let Some((seed, vrf_id)) = VRFSeeds::<T>::get(&nucleus_id, x) {
                                let hash = keccak_256(seed.as_slice());
                                let r = xor(&hash_total, &hash);
                                (x, r)
                            } else {
                                (x, [0u8; 32])
                            }
                        })
                        .collect::<Vec<_>>();
                    result.sort_by(|a, b| {
                        let second_cmp = b.1.cmp(&a.1);
                        if second_cmp == Ordering::Equal {
                            a.0.cmp(&b.0)
                        } else {
                            second_cmp
                        }
                    });
                    let validators = result
                        .iter()
                        .take(vn as usize)
                        .map(|x| x.0.clone())
                        .collect::<Vec<_>>();
                    NucleusValidators::<T>::insert(nucleus_id.clone(), validators);
                    ActiveNucleusMap::<T>::remove(&nucleus_id);
                } else {
                    break;
                }
            }
        }
    }
}
