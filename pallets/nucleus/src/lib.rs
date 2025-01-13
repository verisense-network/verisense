#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// #[cfg(test)]
// mod mock;
// #[cfg(test)]
// mod tests;

pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use codec::{Decode, Encode};
    use frame_support::{pallet_prelude::*, traits::OneSessionHandler};
    use frame_system::pallet_prelude::*;
    use sp_core::crypto::VrfPublic;
    use sp_core::sr25519::{
        vrf::{VrfSignature, VrfTranscript},
        Public,
    };
    use sp_runtime::{
        traits::{Hash, LookupError, MaybeDisplay, One, StaticLookup},
        RuntimeAppPublic,
    };
    use sp_std::prelude::*;
    use vrs_primitives::{keys::NUCLEUS_VRF_KEY_TYPE, NucleusInfo};
    use vrs_support::ValidatorsInterface;

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

    #[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, Debug)]
    pub struct NucleusChallenge<AccountId, Hash> {
        pub submissions: Vec<(AccountId, u64)>,
        pub public_input: Hash,
        pub requires: u8,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type WeightInfo: WeightInfo;

        type AuthorityId: Member + Parameter + RuntimeAppPublic + MaybeSerializeDeserialize;

        type NucleusId: Parameter
            + Member
            + MaybeSerializeDeserialize
            + core::fmt::Debug
            + MaybeDisplay
            + MaxEncodedLen;

        type Validators: ValidatorsInterface<Self::AccountId>;

        type NodeId: Parameter + Member + core::fmt::Debug;

        type RegistryDuration: Get<BlockNumberFor<Self>>;

        type ControllerLookup: StaticLookup<Source = Self::AccountId, Target = Self::NodeId>;
    }

    #[pallet::storage]
    #[pallet::unbounded]
    pub type Nuclei<T: Config> = StorageMap<
        Hasher = Blake2_128Concat,
        Key = T::NucleusId,
        Value = NucleusEquation<T::AccountId, T::Hash, T::NodeId>,
        QueryKind = OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::unbounded]
    pub type NodeControllers<T: Config> = StorageMap<
        Hasher = Blake2_128Concat,
        Key = T::AccountId,
        Value = T::NodeId,
        QueryKind = OptionQuery,
    >;

    // TODO we need to use FHE to hide the real account
    #[pallet::storage]
    #[pallet::unbounded]
    pub type Instances<T: Config> = StorageMap<
        Hasher = Blake2_128Concat,
        Key = T::NucleusId,
        Value = Vec<T::AccountId>,
        QueryKind = ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::unbounded]
    pub type OnCreationNuclei<T: Config> = StorageMap<
        Hasher = Blake2_128Concat,
        Key = BlockNumberFor<T>,
        Value = Vec<T::NucleusId>,
        QueryKind = ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::unbounded]
    pub type RegistrySubmissions<T: Config> = StorageMap<
        Hasher = Blake2_128Concat,
        Key = T::NucleusId,
        Value = NucleusChallenge<T::AccountId, T::Hash>,
        QueryKind = OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        NucleusCreated {
            id: T::NucleusId,
            name: Vec<u8>,
            manager: T::AccountId,
            energy: u128,
            capacity: u8,
            public_input: T::Hash,
        },
        NucleusUpgraded {
            id: T::NucleusId,
            wasm_hash: T::Hash,
            wasm_version: u32,
            wasm_location: T::NodeId,
        },
        // TODO
        InstanceRegistered {
            id: T::NucleusId,
            controller: T::AccountId,
            node_id: Option<T::NodeId>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        NucleusIdAlreadyExists,
        NucleusNotFound,
        NotAuthorized,
        InvalidVrfProof,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T>
    where
        T::AccountId: Into<[u8; 32]>,
    {
        // TODO check the capacity
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::create_nucleus())]
        pub fn create_nucleus(
            origin: OriginFor<T>,
            name: Vec<u8>,
            energy: Option<u128>,
            capacity: u8,
        ) -> DispatchResult {
            let manager = ensure_signed(origin)?;
            ensure!(name.len() <= 80, "Name too long");
            let hash = T::Hashing::hash_of(&(manager.clone(), name.clone()));
            let id = <T::NucleusId>::decode(&mut &hash.as_ref()[..]).expect("qed;");
            ensure!(
                !Nuclei::<T>::contains_key(&id),
                Error::<T>::NucleusIdAlreadyExists
            );
            let current_block = frame_system::Pallet::<T>::block_number();
            let public_input = frame_system::Pallet::<T>::block_hash(current_block - One::one());
            OnCreationNuclei::<T>::mutate(current_block + T::RegistryDuration::get(), |pendings| {
                pendings.push(id.clone());
            });
            RegistrySubmissions::<T>::insert(
                &id,
                NucleusChallenge {
                    submissions: Vec::new(),
                    public_input: public_input.clone(),
                    requires: capacity,
                },
            );
            Nuclei::<T>::insert(
                &id,
                NucleusEquation {
                    name: name.clone(),
                    manager: manager.clone(),
                    wasm_hash: Default::default(),
                    wasm_version: 0,
                    wasm_location: None,
                    energy: energy.unwrap_or_default(),
                    current_event: 0,
                    root_state: T::Hash::default(),
                    capacity,
                },
            );
            Self::deposit_event(Event::NucleusCreated {
                id,
                name,
                manager,
                energy: energy.unwrap_or_default(),
                capacity,
                public_input,
            });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::create_nucleus())]
        pub fn upload_nucleus_wasm(
            origin: OriginFor<T>,
            nucleus_id: T::NucleusId,
            to: T::NodeId,
            hash: T::Hash,
        ) -> DispatchResult {
            let manager = ensure_signed(origin)?;
            let id = nucleus_id.clone();
            Nuclei::<T>::try_mutate_exists(&nucleus_id, |nucleus| -> DispatchResult {
                let nucleus = nucleus.as_mut().ok_or(Error::<T>::NucleusNotFound)?;
                ensure!(nucleus.manager == manager, Error::<T>::NotAuthorized);
                if nucleus.wasm_hash != hash {
                    nucleus.wasm_version += 1;
                    nucleus.wasm_hash = hash;
                }
                nucleus.wasm_location = Some(to.clone());
                Self::deposit_event(Event::NucleusUpgraded {
                    id,
                    wasm_hash: hash,
                    wasm_version: nucleus.wasm_version,
                    wasm_location: to,
                });
                Ok(())
            })?;
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight((T::WeightInfo::register(), Pays::No))]
        pub fn register(
            origin: OriginFor<T>,
            nucleus_id: T::NucleusId,
            signature: VrfSignature,
        ) -> DispatchResult {
            let submitter = ensure_signed(origin)?;
            let raw = submitter.into();
            let controller = T::Validators::is_active_validator(NUCLEUS_VRF_KEY_TYPE, &raw)
                .ok_or(Error::<T>::NotAuthorized)?;
            let public = Public::from_raw(raw);
            let challenge =
                RegistrySubmissions::<T>::get(&nucleus_id).ok_or(Error::<T>::NucleusNotFound)?;
            let ctx = b"vrfq";
            let input = VrfTranscript::new(
                b"nucleus",
                &[(b"register", challenge.public_input.as_ref())],
            );
            let data = input.clone().into_sign_data();
            if !public.vrf_verify(&data, &signature) {
                return Err(Error::<T>::InvalidVrfProof.into());
            }
            let out = public
                .make_bytes::<8>(ctx, &input, &signature.pre_output)
                .expect("make bytes won't fail;qed");
            let out = u64::from_le_bytes(out);
            RegistrySubmissions::<T>::mutate(&nucleus_id, |challenge| {
                let challenge = challenge.as_mut().expect("already checked");
                challenge.submissions.push((controller, out));
            });
            Ok(())
        }
    }

    // TODO
    impl<T: Config> StaticLookup for Pallet<T> {
        type Source = T::AccountId;
        type Target = T::NodeId;

        fn lookup(a: Self::Source) -> Result<Self::Target, LookupError> {
            NodeControllers::<T>::get(&a).ok_or(LookupError)
        }

        fn unlookup(_n: Self::Target) -> Self::Source {
            unimplemented!()
        }
    }

    impl<T: Config> sp_runtime::BoundToRuntimeAppPublic for Pallet<T> {
        type Public = T::AuthorityId;
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

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(now: BlockNumberFor<T>) -> Weight {
            let mut weight = Weight::from_parts(0, 500);
            let nuclei = OnCreationNuclei::<T>::take(now);
            for id in nuclei {
                weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
                let mut task = RegistrySubmissions::<T>::take(&id)
                    .expect("this is a bug: registry submission not found");
                task.submissions.sort_by_key(|(_, v)| *v);
                task.submissions
                    .into_iter()
                    .take(task.requires as usize)
                    .for_each(|(controller, _)| {
                        weight = weight.saturating_add(T::DbWeight::get().writes(1));
                        Instances::<T>::mutate(&id, |peers| {
                            peers.push(controller.clone());
                        });
                        Self::deposit_event(Event::InstanceRegistered {
                            id: id.clone(),
                            controller,
                            node_id: None,
                        });
                    });
            }
            // TODO rotate the members
            weight
        }

        fn on_finalize(_now: BlockNumberFor<T>) {}
    }

    impl<T: Config> Pallet<T> {
        pub fn get_nucleus_info(
            nucleus_id: &T::NucleusId,
        ) -> Option<NucleusInfo<T::AccountId, T::Hash, T::NodeId>> {
            let eqution = Nuclei::<T>::get(nucleus_id)?;
            let peers = Instances::<T>::get(nucleus_id);
            let peers = peers
                .iter()
                .filter_map(|p| T::ControllerLookup::lookup(p.clone()).ok())
                .collect();
            Some(NucleusInfo {
                name: eqution.name,
                manager: eqution.manager,
                wasm_hash: eqution.wasm_hash,
                wasm_version: eqution.wasm_version,
                wasm_location: eqution.wasm_location,
                // energy: eqution.energy,
                current_event: eqution.current_event,
                root_state: eqution.root_state,
                peers,
            })
        }
    }
}
