use codec::{Decode, Encode};
use futures::prelude::*;
use sc_client_api::{Backend, BlockBackend, BlockchainEvents, StorageProvider};
use sc_network::{request_responses::IncomingRequest, service::traits::{NotificationEvent}, PeerId, Multiaddr};
use sc_authority_discovery::Service as AuthorityDiscovery;
use sp_api::{Metadata, ProvideRuntimeApi};
use sp_application_crypto::key_types::AUTHORITY_DISCOVERY;
use sp_blockchain::HeaderBackend;
use sp_core::{sr25519, ByteArray};
use sp_keystore::KeystorePtr;
use std::{str::FromStr, sync::Arc};
use std::collections::HashSet;
use tokio::sync::{Mutex};
use async_channel::{Recv, RecvError};
use futures::channel::oneshot;
use log::log;
use sc_network::request_responses::OutgoingResponse;
use sp_authority_discovery::AuthorityId;
use tokio::time;
use tokio::time::{timeout, Timeout};
use tokio::time::error::Elapsed;
use vrs_primitives::AccountId;


pub struct P2pParams<B, C, BN> {
    pub keystore: KeystorePtr,
    pub reqres_receiver: async_channel::Receiver<IncomingRequest>,
    pub client: Arc<C>,
    pub node_key_pair: libp2p::identity::Keypair,
    pub net_service: Arc<dyn sc_network::service::traits::NetworkService>,
    pub p2p_cage_tx: tokio::sync::mpsc::Sender<(PayloadWithSignature,PeerId, oneshot::Sender<OutgoingResponse>)>,
    pub cage_p2p_rx: tokio::sync::mpsc::Receiver<(SendMessage, oneshot::Sender<String>)>,
    pub controller: AccountId,
    pub authority_discovery: Arc<Mutex<AuthorityDiscovery>>,
    pub authorities: Vec<AuthorityId>,
    pub _phantom: std::marker::PhantomData<(B, BN)>,
}

#[derive(Debug, Encode, Decode, Clone)]
pub enum RequestContent {
    SendToken(Vec<u8>),
    QueryEvents(Vec<u8>)
}

#[derive(Debug, Encode, Decode)]
pub struct PayloadWithSignature {
    pub payload: Vec<u8>,
    pub public_key: Vec<u8>,
    pub peer_id: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug)]
pub enum Destination {
    AuthorityId(AuthorityId),
    PeerId(PeerId)
}

#[derive(Debug)]
pub struct SendMessage {
    pub dest: Destination,
    pub request: RequestContent,
}

pub fn start_nucleus_p2p<B, C, BN>(params: P2pParams<B, C, BN>) -> impl Future<Output = ()>
    where
        B: sp_runtime::traits::Block,
        BN: Backend<B>,
        C: BlockBackend<B>
        + StorageProvider<B, BN>
        + BlockchainEvents<B>
        + ProvideRuntimeApi<B>
        + HeaderBackend<B>
        + 'static,
        C::Api: Metadata<B>,
{
    let P2pParams {
        keystore,
        reqres_receiver,
        client,
        node_key_pair,
        mut net_service,
        p2p_cage_tx,
        mut cage_p2p_rx,
        controller,
        mut authority_discovery,
        authorities,
        _phantom,
    } = params;

    async move {
        log::info!("🔌 Nucleus p2p controller: {}", controller);
        loop {
            tokio::select! {
                Ok(req) = reqres_receiver.recv() => {
                    let source = req.peer;
                    let payload: PayloadWithSignature = Decode::decode(&mut &req.payload[..]).unwrap();
                    log::info!("incoming p2p message: {:?}", payload);
                    let _ = p2p_cage_tx.send((payload, source, req.pending_response)).await;
                }
                Some((send_payload, resp_sender)) = cage_p2p_rx.recv() => {
                    let peers = match send_payload.dest {
                        Destination::AuthorityId(a) => {
                            let r = authority_discovery.clone().lock().await.get_addresses_by_authority_id(a).await;
                            match r {
                                None => {vec![]}
                                Some(mut ma) => {
                                    let mut v = vec![];
                                    for m in ma  {
                                        let n = m.to_string().split("/").last().unwrap().to_string();
                                        let p = PeerId::from_str(n.as_str()).unwrap();
                                        v.push(p);
                                    }
                                    v
                                }
                            }
                        },
                        Destination::PeerId(p) => {
                            vec![p.clone()]
                        }
                    };
                    let mut success = false;
                    for p in peers {
                        if let Ok(r) = send_request(
                            net_service.clone(),
                            keystore.clone(),
                            &p,
                            send_payload.request.clone(),
                        ).await {
                            success = true;
                            break;
                        }
                    }
                    let  r = if success { "OK"} else {"ERR"};
                    let _ = resp_sender.send(r.to_string());
                }
            }
        }
    }
}

pub async fn send_request(
    net_service: Arc<dyn sc_network::service::traits::NetworkService>,
    keystore: KeystorePtr,
    node_id: &PeerId,
    request: RequestContent,
) -> Result<Vec<u8>, ()> {
    let public_key = get_public_from_keystore(keystore.clone()).map_err(|_| ())?;
    let signature = sign_message(keystore.clone(), &node_id.to_bytes()).map_err(|_| ())?;
    let payload = PayloadWithSignature {
        payload: request.encode(),
        public_key: public_key.to_raw_vec(),
        peer_id: node_id.to_bytes(),
        signature: signature.to_raw_vec(),
    };
    // encode the payload to vec<u8>
    let data = payload.encode();
    let result = net_service
        .request(
            node_id.clone(),
            sc_network::types::ProtocolName::Static("/nucleus/p2p/reqres"),
            data,
            None,
            sc_network::IfDisconnected::ImmediateError,
        )
        .await;
    if let Ok((res, name)) = result {
        log::info!(
            "Response of the request is: {}: {:?}",
            name,
            std::str::from_utf8(&res).expect("not a valid ascii string.")
        );
        return Ok(res);
    } else {
        log::info!("Error on response of the request {:?}", result);
        return Err(());
    }
}

pub fn get_public_from_keystore(
    // keystore: &dyn Keystore,
    keystore: KeystorePtr,
) -> Result<sr25519::Public, Box<dyn std::error::Error>> {
    // Get all public keys
    let public_keys = keystore.sr25519_public_keys(AUTHORITY_DISCOVERY);

    // Get existing key or generate new one
    let public_id = if let Some(pk) = public_keys.first() {
        *pk
    } else {
        keystore.sr25519_generate_new(AUTHORITY_DISCOVERY, None)?
    };

    Ok(public_id)
}

// 2. Sign with specific public key
fn sign_with_public(
    // keystore: &dyn Keystore,
    keystore: KeystorePtr,
    public: &sr25519::Public,
    message: &[u8],
) -> Result<sr25519::Signature, Box<dyn std::error::Error>> {
    let signature = keystore
        .sr25519_sign(AUTHORITY_DISCOVERY, public, message)?
        .ok_or("Message signing failed")?;

    Ok(signature)
}

fn sign_message(
    // keystore: &dyn Keystore,
    keystore: KeystorePtr,
    message: &[u8],
) -> Result<sr25519::Signature, Box<dyn std::error::Error>> {
    // Get public key
    let public = keystore
        .sr25519_public_keys(AUTHORITY_DISCOVERY)
        .first()
        .copied()
        .ok_or("No public key found")?;

    // Sign the message
    let signature = keystore
        .sr25519_sign(AUTHORITY_DISCOVERY, &public, message)?
        .ok_or("Message signing failed")?;

    Ok(signature)
}