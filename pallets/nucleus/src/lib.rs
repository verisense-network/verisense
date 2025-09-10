#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// #[cfg(test)]
// mod mock;
// #[cfg(test)]
// mod tests;

pub mod check_nonce;
pub mod weights;

pub use weights::*;

type AssetId<T> = <<T as pallet::Config>::Assets as frame_support::traits::fungibles::Inspect<
    <T as frame_system::Config>::AccountId,
>>::AssetId;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use codec::{Decode, Encode};
    use frame_support::traits::fungibles;
    use frame_support::{pallet_prelude::*, traits::OneSessionHandler};
    use frame_system::pallet_prelude::*;
    use sp_core::crypto::VrfPublic;
    use sp_core::sr25519::{
        vrf::{VrfSignature, VrfTranscript},
        Public,
    };
    use sp_runtime::{
        traits::{Hash, MaybeDisplay, One},
        RuntimeAppPublic,
    };
    use sp_std::prelude::*;
    use vrs_primitives::{keys::NUCLEUS_VRF_KEY_TYPE, NucleusInfo};
    use vrs_support::{AgentRegistry, ValidatorsInterface};

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Default, TypeInfo, Debug)]
    pub struct NucleusEquation<AccountId, Hash, NodeId> {
        pub name: Vec<u8>,
        pub manager: AccountId,
        pub a2a_compatible: bool,
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

        type Weight: WeightInfo;

        type AuthorityId: Member + Parameter + RuntimeAppPublic + MaybeSerializeDeserialize;

        type NucleusId: Parameter
            + Member
            + MaybeSerializeDeserialize
            + core::fmt::Debug
            + MaybeDisplay
            + MaxEncodedLen;

        type Validators: ValidatorsInterface<Self::AccountId>;

        type AgentRegistry: AgentRegistry<Self::AccountId>;

        type NodeId: Parameter + Member + core::fmt::Debug;

        type RegistryDuration: Get<BlockNumberFor<Self>>;

        type Assets: fungibles::approvals::Mutate<<Self as frame_system::Config>::AccountId>;

        type FeeCollector: Get<Self::AccountId>;

        #[pallet::constant]
        type FeeAssetId: Get<AssetId<Self>>;
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
    pub type ForcedInstances<T: Config> =
        StorageValue<Value = Vec<T::AccountId>, QueryKind = ValueQuery>;

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
        UnsupportedOperation,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T>
    where
        T::AccountId: Into<[u8; 32]>,
        <<T as pallet::Config>::Assets as frame_support::traits::fungibles::Inspect<
            <T as frame_system::Config>::AccountId,
        >>::Balance: From<u128>,
        <T as frame_system::Config>::AccountId: From<<T as pallet::Config>::NucleusId>,
    {
        // TODO check the capacity
        #[pallet::call_index(0)]
        #[pallet::weight(T::Weight::create_nucleus())]
        pub fn create_nucleus(
            origin: OriginFor<T>,
            name: Vec<u8>,
            energy: Option<u128>,
            capacity: u8,
            a2a_compatible: bool,
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
                    a2a_compatible,
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
        #[pallet::weight(T::Weight::create_nucleus())]
        pub fn upload_nucleus_wasm(
            origin: OriginFor<T>,
            nucleus_id: T::NucleusId,
            to: T::NodeId,
            hash: T::Hash,
            agent_card: Option<a2a_rs::AgentCard>,
        ) -> DispatchResult {
            let manager = ensure_signed(origin)?;
            let id = nucleus_id.clone();
            Nuclei::<T>::try_mutate_exists(&nucleus_id, |nucleus| -> DispatchResult {
                let mut mutate = nucleus.take().ok_or(Error::<T>::NucleusNotFound)?;
                ensure!(mutate.manager == manager, Error::<T>::NotAuthorized);
                if mutate.wasm_hash != hash {
                    mutate.wasm_version += 1;
                    mutate.wasm_hash = hash;
                }
                mutate.wasm_location = Some(to.clone());
                nucleus.replace(mutate.clone());
                if mutate.a2a_compatible {
                    if let Some(agent_card) = agent_card {
                        let agent_info = a2a_rs::AgentInfo {
                            agent_id: T::AccountId::from(nucleus_id.clone()),
                            owner_id: manager.clone(),
                            url_verified: false,
                            price_rate: 100,
                            agent_card,
                        };
                        T::AgentRegistry::update_agent(agent_info)
                            .map_err(|_| Error::<T>::UnsupportedOperation)?;
                    }
                }
                Self::deposit_event(Event::NucleusUpgraded {
                    id,
                    wasm_hash: hash,
                    wasm_version: mutate.wasm_version,
                    wasm_location: to,
                });
                Ok(())
            })?;
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight((T::Weight::register(), Pays::No))]
        pub fn register(
            origin: OriginFor<T>,
            nucleus_id: T::NucleusId,
            signature: VrfSignature,
        ) -> DispatchResult {
            let submitter = ensure_signed(origin)?;
            let raw = submitter.into();
            let controller = T::Validators::lookup_active_validator(NUCLEUS_VRF_KEY_TYPE, &raw)
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

        #[pallet::call_index(3)]
        #[pallet::weight((T::Weight::submit_work(), Pays::No))]
        pub fn submit_work(origin: OriginFor<T>, nucleus_id: T::NucleusId) -> DispatchResult
        where
            <<T as pallet::Config>::Assets as frame_support::traits::fungibles::Inspect<
                <T as frame_system::Config>::AccountId,
            >>::Balance: From<u128>,
            <T as frame_system::Config>::AccountId: From<<T as pallet::Config>::NucleusId>,
        {
            ensure_signed(origin)?;

            /*   use frame_support::traits::fungibles::approvals::Mutate;
            let submitter = ensure_signed(origin)?;
            let raw = submitter.into();
            let _controller = T::Validators::lookup_active_validator(NUCLEUS_VRF_KEY_TYPE, &raw)
                .ok_or(Error::<T>::NotAuthorized)?;

            //TODO do some check

            T::Assets::approve(T::FeeAssetId::get(), &nucleus_id.clone().into(), &T::FeeCollector::get(), 1u128.into())?;
            T::Assets::transfer_from(T::FeeAssetId::get(), &nucleus_id.into(), &T::FeeCollector::get(), &T::FeeCollector::get(), 1u128.into())?;*/
            Ok(())
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

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        pub preset: Vec<T::AccountId>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            ForcedInstances::<T>::put(self.preset.clone());
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
                let forced_members = ForcedInstances::<T>::get();
                forced_members
                    .iter()
                    .take(task.requires as usize)
                    .for_each(|controller| {
                        weight = weight.saturating_add(T::DbWeight::get().writes(1));
                        Instances::<T>::mutate(&id, |peers| {
                            peers.push(controller.clone());
                        });
                        Self::deposit_event(Event::InstanceRegistered {
                            id: id.clone(),
                            controller: controller.clone(),
                            node_id: None,
                        });
                    });
                if task.requires as usize > forced_members.len() {
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
            let validators = Instances::<T>::get(nucleus_id);
            Some(NucleusInfo {
                name: eqution.name,
                manager: eqution.manager,
                wasm_hash: eqution.wasm_hash,
                wasm_version: eqution.wasm_version,
                wasm_location: eqution.wasm_location,
                // energy: eqution.energy,
                current_event: eqution.current_event,
                root_state: eqution.root_state,
                validators,
            })
        }

        pub fn get_members(nucleus_id: &T::NucleusId) -> Vec<T::AccountId> {
            Instances::<T>::get(nucleus_id)
        }
    }
}
