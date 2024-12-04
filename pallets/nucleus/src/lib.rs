#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// #[cfg(test)]
// mod mock;
// #[cfg(test)]
// mod tests;

pub mod weights;
use verisense_support::VrfInterface;
pub use weights::*;
#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use codec::{Decode, Encode};
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{Hash, LookupError, MaybeDisplay, StaticLookup};
    use sp_std::prelude::*;
    use vrs_primitives::NucleusInfo;

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

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Default, TypeInfo, Debug)]
    pub struct NucleusChallenge<AuthorityId, Hash> {
        pub submissions: Vec<(AuthorityId, Vec<u8>)>,
        pub seed: Hash,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type WeightInfo: WeightInfo;

        type NucleusId: Parameter
            + Member
            + MaybeSerializeDeserialize
            + core::fmt::Debug
            + MaybeDisplay
            + MaxEncodedLen;

        type AuthorityId: Parameter + Member + core::fmt::Debug + MaybeDisplay + MaxEncodedLen;

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
        Key = T::BlockNumber,
        Value = Vec<T::NucleusId>,
        QueryKind = ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::unbounded]
    pub type RegistrySubmissions<T: Config> = StorageMap<
        Hasher = Blake2_128Concat,
        Key = T::NucleusId,
        Value = NucleusChallenge<T::AuthorityId, T::Hash>,
        QueryKind = OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        NucleusCreated {
            id: T::NucleusId,
            name: Vec<u8>,
            manager: T::AccountId,
            wasm_hash: T::Hash,
            wasm_version: u32,
            energy: u128,
            capacity: u8,
            seed: T::Hash,
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
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::create_nucleus())]
        pub fn create_nucleus(
            origin: OriginFor<T>,
            name: Vec<u8>,
            wasm_hash: T::Hash,
            energy: Option<u128>,
            // TODO check the capacity
            capacity: u8,
        ) -> DispatchResult {
            let manager = ensure_signed(origin)?;
            ensure!(name.len() <= 80, "Name too long");
            let hash = T::Hashing::hash_of(&(manager.clone(), name.clone(), wasm_hash.clone()));
            let id = <T::NucleusId>::decode(&mut &hash.as_ref()[..]).expect("qed;");
            ensure!(
                !Nuclei::<T>::contains_key(&id),
                Error::<T>::NucleusIdAlreadyExists
            );
            let current_block = frame_system::Pallet::<T>::block_number();
            let seed = frame_system::Pallet::<T>::block_hash(current_block - 1);
            OnCreationNuclei::<T>::try_mutate(
                current_block + T::RegistryDuration::get(),
                |pendings| {
                    pendings.push(id.clone());
                    Ok(())
                },
            )?;
            RegistrySubmissions::<T>::insert(
                &id,
                NucleusChallenge {
                    submissions: vec![],
                    seed: seed.clone(),
                },
            );
            Nuclei::<T>::insert(
                &id,
                NucleusEquation {
                    name: name.clone(),
                    manager: manager.clone(),
                    wasm_hash,
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
                wasm_hash,
                wasm_version: 0,
                energy: energy.unwrap_or_default(),
                capacity,
                seed,
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
        #[pallet::weight(T::WeightInfo::register())]
        pub fn register(
            origin: OriginFor<T>,
            nucleus_id: T::NucleusId,
            proof: Vec<u8>,
        ) -> DispatchResult {
            let submitter = ensure_signed(origin)?;
            // Instances::<T>::mutate(&nucleus_id, |cages| {
            //     // TODO
            //     cages.push(controller.clone());
            // });
            // let node_id = T::ControllerLookup::lookup(controller.clone()).ok();
            // Self::deposit_event(Event::InstanceRegistered {
            //     id: nucleus_id.clone(),
            //     controller: controller.clone(),
            //     node_id,
            // });
            // T::Vrf::register_nucleus_blocknumber(
            //     nucleus_id,
            //     <system::Pallet<T>>::block_number(),
            //     validators_num,
            // )?;
            // RegisterBlockNumber::<T>::insert(
            //     &nucleus_id.clone(),
            //     <system::Pallet<T>>::block_number(),
            // );
            // Self::deposit_event(Event::VRFSeedsUpdated {
            //     nucleus_id,
            //     account_id: controller,
            //     seed,
            // });

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
