use codec::{Decode, Encode};
use futures::prelude::*;
use sc_client_api::{Backend, BlockBackend, BlockchainEvents, StorageProvider};
use sc_network::{
    request_responses::IncomingRequest,
    service::traits::{NotificationEvent, NotificationService},
    PeerId,
};
use sp_api::{Metadata, ProvideRuntimeApi};
use sp_application_crypto::key_types::AUTHORITY_DISCOVERY;
use sp_blockchain::HeaderBackend;
use sp_core::{sr25519, ByteArray, Pair};
use sp_keystore::KeystorePtr;
use std::{str::FromStr, sync::Arc};
use vrs_primitives::AccountId;

pub struct P2pParams<B, C, BN> {
    pub keystore: KeystorePtr,
    pub reqres_receiver: async_channel::Receiver<IncomingRequest>,
    pub client: Arc<C>,
    pub net_service: Arc<dyn sc_network::service::traits::NetworkService>,
    pub test_receiver: tokio::sync::mpsc::UnboundedReceiver<Vec<PeerId>>,
    pub p2p_cage_tx: tokio::sync::mpsc::Sender<NucleusP2pMsg>,
    pub noti_receiver: tokio::sync::mpsc::Receiver<(Vec<u8>, Vec<PeerId>)>,
    pub noti_service: Box<dyn NotificationService>,
    pub controller: AccountId,
    pub _phantom: std::marker::PhantomData<(B, BN)>,
}

#[derive(Debug)]
pub struct P2pNotification {
    peer: PeerId,
    notification: Vec<u8>,
}

#[derive(Debug)]
pub enum NucleusP2pMsg {
    ReqRes(IncomingRequest),
    Noti(P2pNotification),
}

#[derive(Debug, Encode, Decode)]
pub struct PayloadWithSignature {
    payload: Vec<u8>,
    public_key: Vec<u8>,
    peer_id: Vec<u8>,
    signature: Vec<u8>,
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
        mut net_service,
        mut test_receiver,
        p2p_cage_tx,
        mut noti_receiver,
        mut noti_service,
        controller,
        _phantom,
    } = params;
    async move {
        log::info!("🔌 Nucleus p2p controller: {}", controller);

        loop {
            tokio::select! {
                Some(nodes) = test_receiver.recv() => {
                    for node_id in nodes {
                        let test_data: Vec<u8> = format!("hello, notification").as_bytes().to_vec();
                        log::info!(" ==> going to send noti: {:?} {:?}", node_id, test_data);
                        // let msg_sink = noti_service
                        //     .message_sink(&node_id)
                        //     .expect("error when get msg_sink");
                        // msg_sink.send_sync_notification(test_data);
                        noti_service.send_sync_notification(&node_id, test_data);
                        // _ = noti_service2
                        //     .send_async_notification(&node_id, test_data.clone())
                        //     .await;
                    }
                },
                Some(noti) = noti_receiver.recv() => {

                    let (noti, mut nodes) = noti;
                    // process nodes.is_empty(), if it is, query the whole set of peers
                    if nodes.is_empty() {
                        let mut peer_ids = Vec::new();

                        // Get the network state
                        if let Ok(state) = net_service.network_state().await {
                            // Extract connected peers
                            for (name, _) in state.connected_peers {
                                let peer_id = PeerId::from_str(&name).expect("failed to convert string to PeerId");
                                peer_ids.push(peer_id);
                            }
                        }
                        nodes = peer_ids;
                    }

                    for node_id in nodes {
                        // sign the node_id, get a signature
                        let public_key = get_public_from_keystore(keystore.clone()).map_err(|_| ()).expect("get public from keystore error.");
                        let signature = sign_message(keystore.clone(), &node_id.to_bytes()).map_err(|_| ()).expect("sign msg error");
                        let payload = PayloadWithSignature {
                            payload: noti.clone(),
                            public_key: public_key.to_raw_vec(),
                            peer_id: node_id.to_bytes(),
                            signature: signature.to_raw_vec(),
                        };
                        // encode the payload to vec<u8>
                        let data = payload.encode();
                        // log::info!(" ==> going to send noti: {:?} {:?}", node_id, noti.clone());
                        // _ = noti_service
                        //     .send_async_notification(&node_id, noti)
                        //     .await;
                        noti_service.send_sync_notification(&node_id, data);
                    }
                },
                Ok(request) = reqres_receiver.recv() => {
                    log::debug!("Incoming p2p request msg: {:?}", request);
                    // do stuff
                    // forward the request to cage
                    let msg = NucleusP2pMsg::ReqRes(request);
                    _ = p2p_cage_tx.send(msg).await;

                },
                Some(event) = noti_service.next_event() => {
                    match event {
                        NotificationEvent::NotificationReceived {
                            peer,
                            notification
                        } => {
                            log::info!("p2p notification received: peer: {:?}  noti: {:?}", peer, notification);
                            let noti = P2pNotification {peer, notification};
                            let msg = NucleusP2pMsg::Noti(noti);
                            _ = p2p_cage_tx.send(msg).await;

                        }
                        NotificationEvent::ValidateInboundSubstream {
                            peer,
                            handshake,
                            result_tx,
                        } => {
                            log::info!("notification ValidateInboundSubstream received: {:?}", peer);
                            log::info!("notification ValidateInboundSubstream received: {:?}", handshake);
                            // to build the notification substream
                            // _ = result_tx.send(ValidationResult::Accept);
                        }
                        NotificationEvent::NotificationStreamOpened {
                            peer,
                            direction,
                            handshake,
                            negotiated_fallback,
                        } => {
                            log::info!("notification NotificationStreamOpened received: {:?}", peer);
                            log::info!("notification NotificationStreamOpened received: {:?}", direction);
                            log::info!("notification NotificationStreamOpened received: {:?}", handshake);
                            log::info!("notification NotificationStreamOpened received: {:?}", negotiated_fallback);
                        }
                        NotificationEvent::NotificationStreamClosed { peer } => {
                            log::info!("notification NotificationStreamClosed received: {:?}", peer);
                        }
                    }
                }
            }
        }
    }
}

pub async fn send_request(
    net_service: Arc<dyn sc_network::service::traits::NetworkService>,
    keystore: KeystorePtr,
    node_id: &PeerId,
    data: Vec<u8>,
) -> Result<Vec<u8>, ()> {
    let public_key = get_public_from_keystore(keystore.clone()).map_err(|_| ())?;
    let signature = sign_message(keystore.clone(), &node_id.to_bytes()).map_err(|_| ())?;
    let payload = PayloadWithSignature {
        payload: data,
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
