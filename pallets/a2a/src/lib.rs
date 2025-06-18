#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub mod weights;

use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use a2a_rs::*;
    use codec::Encode;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Hash;
    use sp_std::prelude::*;
    use vrs_support::AgentRegistry;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type Weight: WeightInfo;
    }

    #[pallet::storage]
    #[pallet::unbounded]
    pub type AgentCards<T: Config> = StorageMap<
        Hasher = Blake2_128Concat,
        Key = T::AccountId,
        Value = AgentInfo<T::AccountId>,
        QueryKind = OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AgentRegistered {
            id: T::AccountId,
            owner: T::AccountId,
        },
        AgentUpdated {
            id: T::AccountId,
            owner: T::AccountId,
        },
        AgentDeleted {
            id: T::AccountId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        NotAuthorized,
        AgentAlreadyExists,
        AgentNotFound,
        AgentNameImmutable,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T>
    where
        T::AccountId: From<[u8; 32]>,
        T::Hash: Into<[u8; 32]>,
    {
        /// this is only for registering off-chain agents
        #[pallet::call_index(0)]
        #[pallet::weight(T::Weight::register())]
        pub fn register(origin: OriginFor<T>, agent_card: AgentCard) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            let agent_id = Self::derive_agent_id(&signer, &agent_card);
            let agent = AgentInfo {
                owner_id: signer.clone(),
                agent_id: agent_id.clone(),
                agent_card,
            };
            Self::register_agent(agent)?;
            Ok(())
        }
       
        #[pallet::call_index(1)]
        #[pallet::weight(T::Weight::update())]
        pub fn update(origin: OriginFor<T>, agent_id: T::AccountId, agent_card: AgentCard) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            let mut agent = Self::find_agent(&agent_id).ok_or(Error::<T>::AgentNotFound)?;
            ensure!(agent.owner_id == signer, Error::<T>::NotAuthorized);
            ensure!(agent.agent_card.name == agent_card.name, Error::<T>::AgentNameImmutable);
            agent.agent_card = agent_card;
            Self::update_agent(agent)?;
            Ok(())
        }
        
        #[pallet::call_index(2)]
        #[pallet::weight(T::Weight::delete())]
        pub fn delete(origin: OriginFor<T>, agent_id: T::AccountId) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            let mut agent = Self::find_agent(&agent_id).ok_or(Error::<T>::AgentNotFound)?;
            ensure!(agent.owner_id == signer, Error::<T>::NotAuthorized);
            Ok(())
        }
    }

    impl<T: Config> Pallet<T>
    where
        T::AccountId: From<[u8; 32]>,
        T::Hash: Into<[u8; 32]>,
    {
        pub fn derive_agent_id(owner: &T::AccountId, agent: &AgentCard) -> T::AccountId {
            // Derive agent ID from owner and agent name
            let b1 = owner.encode();
            let bytes = b1
                .iter()
                .chain(agent.name.iter())
                .cloned()
                .collect::<Vec<u8>>();
            let v = T::Hashing::hash(&bytes);
            let bytes: [u8; 32] = v.into();
            T::AccountId::from(bytes)
        }

        pub fn get_all_agents() -> Vec<AgentInfo<T::AccountId>> {
            let mut agents = Vec::new();
            for agent in AgentCards::<T>::iter() {
                agents.push(agent.1);
            }
            agents
        }
    }

    impl<T: Config> AgentRegistry<T::AccountId> for Pallet<T> {
        type Err = Error<T>;

        /// the transaction `register` invokes this to register an off-chain agent
        /// while pallet-nucleus invokes this to register an on-chain agent
        fn register_agent(agent_info: AgentInfo<T::AccountId>) -> Result<(), Self::Err> {
            let agent_id = agent_info.agent_id.clone();
            AgentCards::<T>::try_mutate(&agent_id, |maybe_agent| {
                if maybe_agent.is_some() {
                    return Err(Error::<T>::AgentAlreadyExists);
                }
                let owner = agent_info.owner_id.clone();
                *maybe_agent = Some(agent_info);
                Self::deposit_event(Event::AgentRegistered {
                    id: agent_id.clone(),
                    owner,
                });
                Ok(())
            })?;
            Ok(())
        }

        fn update_agent(agent_info: AgentInfo<T::AccountId>) -> Result<(), Self::Err> {
            let agent_id = agent_info.agent_id.clone();
            AgentCards::<T>::try_mutate(&agent_id, |maybe_agent| {
                let owner = agent_info.owner_id.clone();
                *maybe_agent = Some(agent_info);
                Self::deposit_event(Event::AgentUpdated {
                    id: agent_id.clone(),
                    owner,
                });
                Ok(())
            })?;
            Ok(())
        }

        fn delete_agent(agent_id: &T::AccountId) -> Result<(), Self::Err> {
            AgentCards::<T>::remove(agent_id);
            Self::deposit_event(Event::AgentDeleted {id: agent_id.clone()});
            Ok(())
            
        }
        fn find_agent(agent_id: &T::AccountId) -> Option<AgentInfo<T::AccountId>> {
            AgentCards::<T>::get(agent_id)
        }
    }
}
