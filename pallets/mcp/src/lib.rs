#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub mod migrations;
pub mod offchain;
pub mod weights;

use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use crate::migrations::v1::MigrateV0ToV1;
    use crate::offchain::VerifiedMCPPayload;
    use alloc::string::String;
    use codec::{alloc, Encode};
    use frame_support::dispatch::{GetDispatchInfo, PostDispatchInfo};
    use frame_support::pallet_prelude::*;
    use frame_system::offchain::{AppCrypto, CreateSignedTransaction, SignedPayload, SigningTypes};
    use frame_system::pallet_prelude::*;
    use log::{info, warn};
    use sp_core::crypto::{Ss58AddressFormat, Ss58Codec};
    use sp_core::U256;
    use sp_runtime::traits::Hash;
    use sp_runtime::traits::{Dispatchable, IdentifyAccount};
    use sp_runtime::RuntimeAppPublic;
    use sp_std::prelude::*;
    use sp_std::str::FromStr;
    use url::Url;
    use vrs_primitives::keys::NUCLEUS_VRF_KEY_TYPE as KEY_TYPE;
    use vrs_support::ValidatorsInterface;

    #[derive(Clone, Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
    pub struct McpServerInfo<AccountId> {
        pub name: Vec<u8>,
        pub description: Vec<u8>,
        pub url: Vec<u8>,
        pub url_verified: bool,
        pub provider: AccountId,
        pub price_rate: u16,
        pub logo: Option<String>,
        pub provider_website: Option<String>,
        pub provider_name: Option<String>,
    }

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
        type AuthorityId: Member + Parameter + RuntimeAppPublic + MaybeSerializeDeserialize;

        type AppCrypto: AppCrypto<Self::Public, Self::Signature>;

        type RuntimeCall: Parameter
            + Dispatchable<RuntimeOrigin = Self::RuntimeOrigin, PostInfo = PostDispatchInfo>
            + GetDispatchInfo
            + From<frame_system::Call<Self>>;

        #[pallet::constant]
        type UnsignedPriority: Get<TransactionPriority>;

        type ValidatorsInterface: ValidatorsInterface<Self::AccountId>;

        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type Weight: WeightInfo;
    }

    #[pallet::storage]
    #[pallet::unbounded]
    pub type Servers<T: Config> = StorageMap<
        Hasher = Blake2_128Concat,
        Key = T::AccountId,
        Value = McpServerInfo<T::AccountId>,
        QueryKind = OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::unbounded]
    pub type UnverifiedServers<T: Config> = StorageMap<
        Hasher = Blake2_128Concat,
        Key = T::AccountId,
        Value = (McpServerInfo<T::AccountId>, u32),
        QueryKind = OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        McpServerRegistered {
            id: T::AccountId,
            provider: T::AccountId,
        },
        McpServerDeregistered {
            id: T::AccountId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        NotAuthorized,
        McpServerAlreadyExists,
        McpServerNotFound,
        NotValidator,
        InvalidPriceRate,
        InvalidUrl,
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

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::Weight::register())]
        pub fn register(
            origin: OriginFor<T>,
            name: Vec<u8>,
            description: Vec<u8>,
            url: Vec<u8>,
            price_rate: Option<u16>,
            logo: Option<String>,
            provider_website: Option<String>,
            provider_name: Option<String>,
        ) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            let server_id = Self::derive_server_id(&signer, &name);
            let price_rate = price_rate.unwrap_or(100);
            //check url
            let url_str = String::from_utf8(url.clone());
            ensure!(url_str.is_ok(), Error::<T>::InvalidUrl);
            let url_obj =
                url::Url::parse(url_str.unwrap().as_str()).map_err(|_| Error::<T>::InvalidUrl)?;
            ensure!(url_obj.scheme() == "https", Error::<T>::InvalidUrl);
            ensure!(price_rate <= 1000, Error::<T>::InvalidPriceRate);
            let mcp = McpServerInfo {
                name,
                description,
                url,
                url_verified: true,
                provider: signer,
                price_rate,
                logo,
                provider_website,
                provider_name,
            };
            Self::register_mcp_server(server_id, mcp)?;
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::Weight::deregister())]
        pub fn deregister(origin: OriginFor<T>, server_id: T::AccountId) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            Servers::<T>::try_mutate(&server_id, |maybe_server| {
                if maybe_server.is_none() {
                    return Err(Error::<T>::McpServerNotFound);
                }
                ensure!(
                    maybe_server.as_ref().map(|v| v.provider.clone()) == Some(signer),
                    Error::<T>::NotAuthorized
                );
                *maybe_server = None;
                Ok(())
            })?;
            UnverifiedServers::<T>::remove(&server_id);
            Self::deposit_event(Event::McpServerDeregistered { id: server_id });
            Ok(())
        }

        #[pallet::weight(1)]
        #[pallet::call_index(2)]
        pub fn after_dns_verify(
            origin: OriginFor<T>,
            payload: VerifiedMCPPayload<T::Public, T::AccountId, BlockNumberFor<T>>,
            _signature: T::Signature,
        ) -> DispatchResultWithPostInfo {
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
            let verified_mcps = payload.observations;
            for verified_mcp in verified_mcps {
                UnverifiedServers::<T>::remove(&verified_mcp);
                let _ = Servers::<T>::try_mutate(&verified_mcp, |maybe_server| {
                    if maybe_server.is_none() {
                        return Err(Error::<T>::McpServerNotFound);
                    }
                    let mut server = maybe_server.clone().unwrap();
                    server.url_verified = true;
                    *maybe_server = Some(server);
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
                let mut verified_mcps = vec![];
                for s in UnverifiedServers::<T>::iter() {
                    if block_now - s.1 .1 > 144000 {
                        continue;
                    }
                    let url = String::from_utf8(s.1 .0.url.clone()).unwrap_or_default();
                    let url = Url::from_str(url.as_str());
                    if url.is_err() {
                        continue;
                    }
                    let url = url.unwrap();
                    if let Some(domain) = url.domain() {
                        let mut v: Vec<&str> = domain.split(".").collect();
                        if v.len() > 1 {
                            let txt_content = s.0.to_ss58check_with_version(
                                Ss58AddressFormat::custom(T::SS58Prefix::get()),
                            );
                            let id = hex::encode(s.0.as_ref()[0..16].to_vec());
                            v[0] = id.as_str();
                            let check_result = domain_verifier::verify_domain(
                                v.join(".").as_str(),
                                txt_content.as_str(),
                            );
                            if let Ok(true) = check_result {
                                verified_mcps.push(s.0);
                            }
                        }
                    }
                }
                if verified_mcps.is_empty() {
                    return;
                }
                Self::submit_unsigned_transaction(block_number, public, key_data, verified_mcps)
                    .unwrap();
            }
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn derive_server_id(owner: &T::AccountId, name: &[u8]) -> T::AccountId {
            let b1 = owner.encode();
            let bytes = b1.iter().chain(name.iter()).cloned().collect::<Vec<u8>>();
            let v = T::Hashing::hash(&bytes).encode();
            T::AccountId::decode(&mut &v[..]).expect("neq")
        }

        fn register_mcp_server(
            id: T::AccountId,
            server_info: McpServerInfo<T::AccountId>,
        ) -> DispatchResult {
            Servers::<T>::try_mutate(&id, |maybe_server| {
                if maybe_server.is_some() {
                    return Err(Error::<T>::McpServerAlreadyExists);
                }
                let provider = server_info.provider.clone();
                *maybe_server = Some(server_info.clone());
                Self::deposit_event(Event::McpServerRegistered {
                    id: id.clone(),
                    provider,
                });
                Ok(())
            })?;
            let block = Into::<U256>::into(frame_system::Pallet::<T>::block_number()).as_u32();
            UnverifiedServers::<T>::insert(&id, (server_info, block));
            Ok(())
        }

        pub fn get_all_mcp_servers() -> Vec<(T::AccountId, McpServerInfo<T::AccountId>)> {
            Servers::<T>::iter().collect::<Vec<_>>()
        }

        pub fn find_mcp_server(mcp_id: &T::AccountId) -> Option<McpServerInfo<T::AccountId>> {
            Servers::<T>::get(mcp_id)
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
            ValidTransaction::with_tag_prefix("Mcp")
                .priority(T::UnsignedPriority::get())
                .and_provides(account_id)
                .longevity(5)
                .propagate(true)
                .build()
        }
    }
}
