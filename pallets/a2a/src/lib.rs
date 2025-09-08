#![cfg_attr(not(feature = "std"), no_std)]

use codec::alloc;
pub use pallet::*;
mod migrations;
pub mod offchain;
pub mod weights;

use alloc::string::String;
use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use crate::migrations::v1::MigrateV0ToV1;
    use crate::offchain::VerifiedA2aPayload;
    use a2a_rs::*;
    use codec::Encode;
    use frame_support::dispatch::{GetDispatchInfo, PostDispatchInfo};
    use frame_support::pallet_prelude::*;
    use frame_system::offchain::{AppCrypto, CreateSignedTransaction, SignedPayload, SigningTypes};
    use frame_system::pallet_prelude::*;
    use log::{info, warn};
    use sp_core::crypto::{Ss58AddressFormat, Ss58Codec};
    use sp_core::U256;
    use sp_runtime::traits::{Dispatchable, Hash, IdentifyAccount};
    use sp_runtime::RuntimeAppPublic;
    use sp_std::prelude::*;
    use sp_std::vec;
    use vrs_support::{AgentRegistry, ValidatorsInterface};

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type Weight: WeightInfo;

        type AuthorityId: Member + Parameter + RuntimeAppPublic + MaybeSerializeDeserialize;

        type AppCrypto: AppCrypto<Self::Public, Self::Signature>;

        type RuntimeCall: Parameter
            + Dispatchable<RuntimeOrigin = Self::RuntimeOrigin, PostInfo = PostDispatchInfo>
            + GetDispatchInfo
            + From<frame_system::Call<Self>>;

        #[pallet::constant]
        type UnsignedPriority: Get<TransactionPriority>;

        type ValidatorsInterface: ValidatorsInterface<Self::AccountId>;
    }

    #[pallet::storage]
    #[pallet::unbounded]
    pub type Agents<T: Config> = StorageMap<
        Hasher = Blake2_128Concat,
        Key = T::AccountId,
        Value = AgentInfo<T::AccountId>,
        QueryKind = OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::unbounded]
    pub type UnverifiedAgents<T: Config> = StorageMap<
        Hasher = Blake2_128Concat,
        Key = T::AccountId,
        Value = (AgentInfo<T::AccountId>, u32),
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
        AgentDeregistered {
            id: T::AccountId,
        },
        AgentVerified {
            id: T::AccountId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        NotAuthorized,
        AgentAlreadyExists,
        AgentNotFound,
        AgentNameImmutable,
        NotValidator,
        InvalidPriceRate,
        InvalidUrl,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// this is only for registering off-chain agents
        #[pallet::call_index(0)]
        #[pallet::weight(T::Weight::register())]
        pub fn register(
            origin: OriginFor<T>,
            agent_card: AgentCard,
            price_rate: Option<u16>,
        ) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            let price_rate = price_rate.unwrap_or(100);
            ensure!(price_rate <= 1000, Error::<T>::InvalidPriceRate);
            let agent_id = Self::derive_agent_id(&signer, &agent_card);
            let agent = AgentInfo {
                owner_id: signer.clone(),
                agent_id: agent_id.clone(),
                url_verified: false,
                price_rate,
                agent_card,
            };
            //check url
            let url = url::Url::parse(agent.agent_card.url.as_str())
                .map_err(|_| Error::<T>::InvalidUrl)?;
            ensure!(url.scheme() == "https", Error::<T>::InvalidUrl);
            Self::register_agent(agent.clone())?;
            let block_number =
                Into::<U256>::into(frame_system::Pallet::<T>::block_number()).as_u32();
            UnverifiedAgents::<T>::insert(&agent_id, (agent, block_number));
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::Weight::update())]
        pub fn update(
            origin: OriginFor<T>,
            agent_id: T::AccountId,
            agent_card: AgentCard,
            price_rate: Option<u16>,
        ) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            let mut agent = Self::find_agent(&agent_id).ok_or(Error::<T>::AgentNotFound)?;
            ensure!(agent.owner_id == signer, Error::<T>::NotAuthorized);
            ensure!(
                agent.agent_card.name == agent_card.name,
                Error::<T>::AgentNameImmutable
            );
            //check url
            let url = url::Url::parse(agent.agent_card.url.as_str())
                .map_err(|_| Error::<T>::InvalidUrl)?;
            ensure!(url.scheme() == "https", Error::<T>::InvalidUrl);
            if let Some(price_rate) = price_rate {
                ensure!(price_rate <= 1000, Error::<T>::InvalidPriceRate);
                agent.price_rate = price_rate;
            }
            let old_domain = extract_domain(agent.agent_card.url.as_str());
            let new_domain = extract_domain(agent_card.url.as_str());
            agent.agent_card = agent_card;
            if old_domain != new_domain && new_domain.is_some() {
                agent.url_verified = false;
                let block_number =
                    Into::<U256>::into(frame_system::Pallet::<T>::block_number()).as_u32();
                UnverifiedAgents::<T>::insert(&agent_id, (agent.clone(), block_number));
            }
            Self::update_agent(agent)?;
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::Weight::deregister())]
        pub fn deregister(origin: OriginFor<T>, agent_id: T::AccountId) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            Agents::<T>::try_mutate(&agent_id, |maybe_agent| {
                if maybe_agent.is_none() {
                    return Err(Error::<T>::AgentNotFound);
                }
                ensure!(
                    maybe_agent.as_ref().map(|v| v.owner_id.clone()) == Some(signer),
                    Error::<T>::NotAuthorized
                );
                *maybe_agent = None;
                Ok(())
            })?;
            Self::deposit_event(Event::AgentDeregistered {
                id: agent_id.clone(),
            });
            Ok(())
        }

        #[pallet::weight(1)]
        #[pallet::call_index(3)]
        pub fn after_dns_verify(
            origin: OriginFor<T>,
            payload: VerifiedA2aPayload<T::Public, T::AccountId, BlockNumberFor<T>>,
            _signature: T::Signature,
        ) -> DispatchResultWithPostInfo {
            use vrs_primitives::keys::NUCLEUS_VRF_KEY_TYPE as KEY_TYPE;
            ensure_none(origin)?;
            let val_id =
                T::ValidatorsInterface::lookup_active_validator(KEY_TYPE, &payload.key_data);
            if val_id.is_none() {
                info!(
                    "Not a validator in current validator set, key_data: {:?}",
                    payload.key_data
                );
                return Err(Error::<T>::NotValidator.into());
            }
            let verified_agents = payload.observations;
            for verified_agent in verified_agents {
                UnverifiedAgents::<T>::remove(&verified_agent);
                let _ = Agents::<T>::try_mutate(&verified_agent.clone(), |maybe_server| {
                    if maybe_server.is_none() {
                        return Err(Error::<T>::AgentNotFound);
                    }
                    let mut server = maybe_server.clone().unwrap();
                    server.url_verified = true;
                    *maybe_server = Some(server);
                    Self::deposit_event(Event::AgentVerified { id: verified_agent });
                    Ok(())
                });
            }
            Ok(().into())
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
    where
        <T as frame_system::Config>::AccountId: Ss58Codec,
    {
        fn on_runtime_upgrade() -> Weight {
            use frame_support::traits::OnRuntimeUpgrade;
            MigrateV0ToV1::<T>::on_runtime_upgrade()
        }

        fn offchain_worker(block_number: BlockNumberFor<T>) {
            let block_now = block_number.into().as_u32();
            if block_now % 20 != 0 {
                return;
            }
            if let Some((public, key_data, _validator_id)) = Self::get_validator_id() {
                let mut verified_agents = vec![];
                for s in UnverifiedAgents::<T>::iter() {
                    if block_now - s.1 .1 > 144000 {
                        continue;
                    }
                    if let Some(domain) = extract_domain(s.1 .0.agent_card.url.as_str()) {
                        let mut v: Vec<&str> = domain.split(".").collect();
                        if v.len() > 1 {
                            let id = s.0.to_ss58check_with_version(Ss58AddressFormat::custom(
                                T::SS58Prefix::get(),
                            ));
                            v[0] = id.as_str();
                            let check_result = domain_verifier::verify_domain(v.join(".").as_str());
                            if let Ok(true) = check_result {
                                verified_agents.push(s.0);
                            }
                        }
                    }
                }
                if verified_agents.is_empty() {
                    return;
                }
                Self::submit_unsigned_transaction(block_number, public, key_data, verified_agents)
                    .unwrap();
            }
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn derive_agent_id(owner: &T::AccountId, agent: &AgentCard) -> T::AccountId {
            // Derive agent ID from owner and agent name
            let b1 = owner.encode();
            let bytes = b1
                .iter()
                .chain(agent.name.as_bytes().iter())
                .cloned()
                .collect::<Vec<u8>>();
            let v = T::Hashing::hash(&bytes);
            let bytes = v.encode();
            T::AccountId::decode(&mut &bytes[..]).unwrap()
        }

        pub fn get_all_agents() -> Vec<AgentInfo<T::AccountId>> {
            let mut agents = Vec::new();
            for agent in Agents::<T>::iter() {
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
            Agents::<T>::try_mutate(&agent_id, |maybe_agent| {
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
            Agents::<T>::try_mutate(&agent_id, |maybe_agent| {
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

        fn deregister_agent(agent_id: &T::AccountId) -> Result<(), Self::Err> {
            Agents::<T>::remove(agent_id);
            Self::deposit_event(Event::AgentDeregistered {
                id: agent_id.clone(),
            });
            Ok(())
        }

        fn find_agent(agent_id: &T::AccountId) -> Option<AgentInfo<T::AccountId>> {
            Agents::<T>::get(agent_id)
        }
    }

    impl<T: Config> Pallet<T> {
        fn get_validator_id() -> Option<(<T as SigningTypes>::Public, Vec<u8>, T::AccountId)> {
            use vrs_primitives::keys::NUCLEUS_VRF_KEY_TYPE as KEY_TYPE;
            for key in <T::AppCrypto as AppCrypto<
                <T as SigningTypes>::Public,
                <T as SigningTypes>::Signature,
            >>::RuntimeAppPublic::all()
            .into_iter()
            {
                let key_data = key.to_raw_vec();
                let val_id = T::ValidatorsInterface::lookup_active_validator(KEY_TYPE, &key_data);
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
                warn!(
                    "InvalidTransaction => current_block: {:?}, block_number: {:?}",
                    current_block, block_number
                );
                return InvalidTransaction::Future.into();
            }
            ValidTransaction::with_tag_prefix("A2a")
                .priority(T::UnsignedPriority::get())
                .and_provides(account_id)
                .longevity(5)
                .propagate(true)
                .build()
        }
    }

    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;
        fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
            if let Call::after_dns_verify {
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
}

pub fn extract_domain(x: &str) -> Option<String> {
    use sp_std::prelude::ToOwned;
    let url = url::Url::parse(x).ok()?.domain()?.to_owned();
    Some(url)
}
