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
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{Hash, MaybeDisplay};
    use sp_runtime::Vec;
    use vrs_primitives::NucleusEquation;

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
            + Ord
            + MaxEncodedLen;
    }

    #[pallet::storage]
    #[pallet::unbounded]
    pub type Nuclei<T: Config> = StorageMap<
        Hasher = Blake2_128Concat,
        Key = T::NucleusId,
        Value = NucleusEquation<T::AccountId, T::Hash>,
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

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        NucleusCreated {
            id: T::NucleusId,
            name: Vec<u8>,
            account: T::AccountId,
            wasm_url: Vec<u8>,
            wasm_hash: T::Hash,
            wasm_version: u32,
            energy: u128,
            capacity: u8,
        },
        InstanceRegistered {
            nucleus_id: T::NucleusId,
            controller_account: T::AccountId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::create_nucleus())]
        pub fn create_nucleus(
            origin: OriginFor<T>,
            name: Vec<u8>,
            wasm_url: Vec<u8>,
            wasm_hash: T::Hash,
            // TODO deduction from the author's account
            energy: Option<u128>,
            capacity: u8,
        ) -> DispatchResult {
            let author = ensure_signed(origin)?;
            ensure!(name.len() <= 80, "Name too long");
            ensure!(wasm_url.len() <= 512, "Wasm URL too long");
            let hash = T::Hashing::hash_of(&(author.clone(), name.clone(), wasm_url.clone()));
            let id = <T::NucleusId>::decode(&mut &hash.as_ref()[..]).expect("qed;");
            ensure!(!Nuclei::<T>::contains_key(&id), "Nucleus already exists");
            Nuclei::<T>::insert(
                &id,
                NucleusEquation {
                    name: name.clone(),
                    account: author.clone(),
                    wasm_url: wasm_url.clone(),
                    wasm_hash,
                    wasm_version: 0,
                    energy: energy.unwrap_or_default(),
                    current_event: 0,
                    root_state: T::Hash::default(),
                    capacity,
                },
            );
            Self::deposit_event(Event::NucleusCreated {
                id,
                name,
                account: author,
                wasm_url,
                wasm_hash,
                wasm_version: 0,
                energy: energy.unwrap_or_default(),
                capacity,
            });
            Ok(())
        }

        // TODO just for testing
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::mock_register())]
        pub fn mock_register(origin: OriginFor<T>, nucleus_id: T::NucleusId) -> DispatchResult {
            let controller_account = ensure_signed(origin)?;
            Instances::<T>::mutate(&nucleus_id, |cages| {
                // TODO
                cages.push(controller_account.clone());
            });
            Self::deposit_event(Event::InstanceRegistered {
                nucleus_id,
                controller_account,
            });
            Ok(())
        }
    }
}
