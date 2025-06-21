use codec::{Decode, Encode};
use sc_authority_discovery::Service as AuthorityDiscover;
use sc_network::{service::traits::NetworkService, PeerId};
use sc_network_types::multiaddr::Multiaddr;
use sp_api::ProvideRuntimeApi;
use sp_authority_discovery::AuthorityId;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block;
use std::collections::HashSet;
use std::sync::Arc;
use vrs_nucleus_runtime_api::NucleusRuntimeApi;
use vrs_primitives::{AccountId, NucleusId};

#[derive(Clone, Debug, Encode, Decode)]
pub enum ForwardRequest {
    Get {
        nucleus_id: NucleusId,
        endpoint: String,
        payload: Vec<u8>,
    },
    Post {
        nucleus_id: NucleusId,
        endpoint: String,
        payload: Vec<u8>,
    },
}

impl ForwardRequest {
    pub fn nucleus_id(&self) -> &NucleusId {
        match self {
            ForwardRequest::Get { nucleus_id, .. } => nucleus_id,
            ForwardRequest::Post { nucleus_id, .. } => nucleus_id,
        }
    }
}

#[derive(Clone)]
pub struct Relayer<C, N, B> {
    pub client: Arc<C>,
    pub network: Arc<N>,
    pub authority_discover: AuthorityDiscover,
    _phantom: std::marker::PhantomData<B>,
}

impl<C, N, B> Relayer<C, N, B>
where
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + 'static,
    N: NetworkService + 'static,
    B: Block + 'static,
    C::Api: vrs_nucleus_runtime_api::NucleusRuntimeApi<B> + 'static,
{
    pub fn new(client: Arc<C>, network: Arc<N>, authority_discover: AuthorityDiscover) -> Self {
        Self {
            client,
            network,
            authority_discover,
            _phantom: std::marker::PhantomData,
        }
    }

    fn find_nucleus_validators(
        &self,
        nucleus_id: &NucleusId,
    ) -> Result<Vec<AccountId>, Box<dyn std::error::Error>> {
        let hash = self.client.info().best_hash;
        let api = self.client.runtime_api();
        let validators = api
            .get_nucleus_info(hash, nucleus_id)?
            .map(|info| info.validators)
            .unwrap_or_default();
        Ok(validators)
    }

    pub async fn forward_to(&self, req: ForwardRequest) -> Result<Vec<u8>, String> {
        let validators = self
            .find_nucleus_validators(req.nucleus_id())
            .inspect_err(|e| eprintln!("{:?}", e))
            .map_err(|_| "Unable to read runtime storage")?;
        if validators.is_empty() {
            return Err("No validators found for the given nucleus ID".to_string());
        }
        let encoded = req.encode();
        for validator in validators.iter() {
            let addresses = self.lookup_address(&validator).await;
            if addresses.is_empty() {
                continue;
            }
            for address in addresses {
                let Some(peer_id) = PeerId::try_from_multiaddr(&address) else {
                    continue;
                };
                let response = self
                    .network
                    .request(
                        peer_id,
                        sc_network::ProtocolName::Static("/gluon/1"),
                        encoded.clone(),
                        None,
                        sc_network::IfDisconnected::ImmediateError,
                    )
                    .await;
                match response {
                    Ok(rsp) => return Ok(rsp.0),
                    Err(e) => eprintln!("Failed to send request to {}: {:?}", address, e),
                }
            }
        }
        Err("No peers connected".to_string())
    }

    async fn lookup_address(&self, validator: &AccountId) -> HashSet<Multiaddr> {
        let authority_id: [u8; 32] = *validator.as_ref();
        let authority_id =
            AuthorityId::from(sp_core::crypto_bytes::CryptoBytes::from_raw(authority_id));
        // the service is just a sender
        let mut onetime = self.authority_discover.clone();
        onetime
            .get_addresses_by_authority_id(authority_id)
            .await
            .unwrap_or_default()
    }
}
