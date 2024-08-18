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
    use frame_support::{dispatch::GetDispatchInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_core::OpaquePeerId;
    use sp_std::prelude::*;

    use sp_runtime::traits::{Dispatchable, Hash, MaybeDisplay};

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Default, TypeInfo, Debug)]
    pub struct NucleusEquation<AccountId, Hash, PeerId> {
        pub name: Vec<u8>,
        pub manager: AccountId,
        pub wasm_hash: Hash,
        pub wasm_version: u32,
        pub wasm_location: Option<PeerId>,
        pub energy: u128,
        pub current_event: u64,
        pub root_state: Hash,
        pub capacity: u8,
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

        type PeerId: Parameter + Member + MaybeSerializeDeserialize + core::fmt::Debug;
    }

    #[pallet::storage]
    #[pallet::unbounded]
    pub type Nuclei<T: Config> = StorageMap<
        Hasher = Blake2_128Concat,
        Key = T::NucleusId,
        Value = NucleusEquation<T::AccountId, T::Hash, T::PeerId>,
        QueryKind = OptionQuery,
    >;

    // TODO we need to use FHE to hide the real account
    #[pallet::storage]
    #[pallet::unbounded]
    pub type Instances<T: Config> = StorageMap<
        Hasher = Blake2_128Concat,
        Key = T::NucleusId,
        Value = Vec<(T::AccountId, T::PeerId)>,
        QueryKind = ValueQuery,
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
        },
        NucleusUpgraded {
            id: T::NucleusId,
            wasm_hash: T::Hash,
            wasm_version: u32,
            wasm_location: T::PeerId,
        },
        InstanceRegistered {
            id: T::NucleusId,
            node_controller: T::AccountId,
            node_id: T::PeerId,
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
            });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::create_nucleus())]
        pub fn upload_nucleus_wasm(
            origin: OriginFor<T>,
            nucleus_id: T::NucleusId,
            to: T::PeerId,
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

        // TODO just for testing
        // #[pallet::call_index(2)]
        // #[pallet::weight(T::WeightInfo::mock_register())]
        // pub fn mock_register(origin: OriginFor<T>, nucleus_id: T::NucleusId) -> DispatchResult {
        //     let controller_account = ensure_signed(origin)?;
        //     Instances::<T>::mutate(&nucleus_id, |cages| {
        //         // TODO
        //         cages.push(controller_account.clone());
        //     });
        //     Self::deposit_event(Event::InstanceRegistered {
        //         nucleus_id,
        //         controller_account,
        //     });
        //     Ok(())
        // }
    }
}
