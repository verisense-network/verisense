#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub mod weights;

use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use codec::Encode;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Hash;
    use sp_std::prelude::*;

    #[derive(Clone, Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
    pub struct McpServerInfo<AccountId> {
        pub name: Vec<u8>,
        pub description: Vec<u8>,
        pub url: Vec<u8>,
        pub provider: AccountId,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
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
    }

    #[pallet::call]
    impl<T: Config> Pallet<T>
    where
        T::AccountId: From<[u8; 32]>,
        T::Hash: Into<[u8; 32]>,
    {
        #[pallet::call_index(0)]
        #[pallet::weight(T::Weight::register())]
        pub fn register(
            origin: OriginFor<T>,
            name: Vec<u8>,
            description: Vec<u8>,
            url: Vec<u8>,
        ) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            let server_id = Self::derive_server_id(&signer, &name);
            let mcp = McpServerInfo {
                name,
                description,
                url,
                provider: signer,
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
            Self::deposit_event(Event::McpServerDeregistered { id: server_id });
            Ok(())
        }
    }

    impl<T: Config> Pallet<T>
    where
        T::AccountId: From<[u8; 32]>,
        T::Hash: Into<[u8; 32]>,
    {
        pub fn derive_server_id(owner: &T::AccountId, name: &[u8]) -> T::AccountId {
            let b1 = owner.encode();
            let bytes = b1.iter().chain(name.iter()).cloned().collect::<Vec<u8>>();
            let v = T::Hashing::hash(&bytes);
            let bytes: [u8; 32] = v.into();
            T::AccountId::from(bytes)
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
                *maybe_server = Some(server_info);
                Self::deposit_event(Event::McpServerRegistered {
                    id: id.clone(),
                    provider,
                });
                Ok(())
            })?;
            Ok(())
        }

        pub fn get_all_mcp_servers() -> Vec<(T::AccountId, McpServerInfo<T::AccountId>)> {
            Servers::<T>::iter().collect::<Vec<_>>()
        }

        pub fn find_mcp_server(mcp_id: &T::AccountId) -> Option<McpServerInfo<T::AccountId>> {
            Servers::<T>::get(mcp_id)
        }
    }
}
