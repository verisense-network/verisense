#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub mod weights;

use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use a2a_rs::*;
    use codec::{Decode, Encode};
    use frame_support::traits::fungibles;
    use frame_support::{pallet_prelude::*, traits::OneSessionHandler};
    use frame_system::pallet_prelude::*;
    use sp_runtime::{
        traits::{Hash, LookupError, MaybeDisplay, One, StaticLookup},
        RuntimeAppPublic,
    };
    use sp_std::prelude::*;
    use vrs_support::AgentRegistry;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type Weight: WeightInfo;

        type AgentId: Parameter
            + Member
            + MaybeSerializeDeserialize
            + core::fmt::Debug
            + MaybeDisplay
            + MaxEncodedLen;
    }

    #[pallet::storage]
    #[pallet::unbounded]
    pub type AgentCards<T: Config> = StorageMap<
        Hasher = Blake2_128Concat,
        Key = T::AgentId,
        Value = (T::AccountId, AgentCard),
        QueryKind = OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AgentRegistered { id: T::AgentId, owner: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T> {
        NotAuthorized,
        AgentAlreadyExists,
        AgentNotFound,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T>
    where
        T::AgentId: From<[u8; 32]>,
    {
        #[pallet::call_index(0)]
        #[pallet::weight(T::Weight::register())]
        pub fn register(origin: OriginFor<T>, agent_card: AgentCard) -> DispatchResult {
            let manager = ensure_signed(origin)?;
            let agent_id = Self::derive_agent_id(&manager, &agent_card);
            Self::register_agent(manager.clone(), agent_id.clone(), agent_card)?;
            Self::deposit_event(Event::AgentRegistered {
                id: agent_id,
                owner: manager,
            });
            Ok(())
        }
    }

    impl<T: Config> Pallet<T>
    where
        T::AgentId: From<[u8; 32]>,
    {
        pub fn derive_agent_id(owner: &T::AccountId, agent: &AgentCard) -> T::AgentId {
            // Derive agent ID from owner and agent name
            let b1 = owner.encode();
            let bytes = b1
                .iter()
                .chain(agent.name.iter())
                .cloned()
                .collect::<Vec<u8>>();
            let v = T::Hashing::hash(&bytes);
            let bytes: [u8; 32] = bytes
                .try_into()
                .expect("Agent ID must be 32 bytes long; qed");
            T::AgentId::from(bytes)
        }
    }

    impl<T: Config> AgentRegistry<T::AccountId, T::AgentId> for Pallet<T> {
        type Err = Error<T>;

        fn register_agent(
            owner: T::AccountId,
            agent_id: T::AgentId,
            agent_card: AgentCard,
        ) -> Result<(), Self::Err> {
            AgentCards::<T>::try_mutate(&agent_id, |maybe_card| {
                if maybe_card.is_some() {
                    return Err(Error::<T>::AgentAlreadyExists);
                }
                *maybe_card = Some((owner.clone(), agent_card));
                Self::deposit_event(Event::AgentRegistered {
                    id: agent_id.clone(),
                    owner,
                });
                Ok(())
            })?;
            Ok(())
        }

        fn find_agent(agent_id: &T::AgentId) -> Result<(T::AccountId, AgentCard), Self::Err> {
            AgentCards::<T>::get(agent_id).ok_or(Error::<T>::AgentNotFound)
        }
    }
}
