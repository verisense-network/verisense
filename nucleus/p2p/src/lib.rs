// use codec::{Decode, Encode};
use futures::prelude::*;
use sc_client_api::{Backend, BlockBackend, BlockchainEvents, StorageProvider};
use sp_api::{Metadata, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
// use std::collections::HashMap;
// use std::sync::mpsc::Sender as SyncSender;
use std::str::FromStr;
use std::sync::Arc;
use vrs_primitives::{AccountId, Hash, NucleusId};

use sc_network::request_responses::IncomingRequest;
use sc_network::service::traits::NotificationEvent;
use sc_network::service::traits::NotificationService;
use sc_network::service::traits::ValidationResult;
use sc_network::PeerId;

pub struct P2pParams<B, C, BN> {
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

struct NucleusP2p {}

impl NucleusP2p {}

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
            // println!(" ---> noti protocol name: {:?}", noti_service.protocol());
            tokio::select! {
                Some(nodes) = test_receiver.recv() => {
                    println!(" ==> get nodes info: {:?}", nodes);
                    for node_id in nodes {
                        let test_data: Vec<u8> = format!("hello, notification").as_bytes().to_vec();
                        println!(" ==> going to send noti: {:?} {:?}", node_id, test_data);
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
                    println!(" ==> get noti info: {:?}", noti);

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
                        println!("nodes id: {:?}", peer_ids);
                        nodes = peer_ids;
                    }

                    for node_id in nodes {
                        // println!(" ==> going to send noti: {:?} {:?}", node_id, noti.clone());
                        // _ = noti_service
                        //     .send_async_notification(&node_id, noti)
                        //     .await;
                        noti_service.send_sync_notification(&node_id, noti.clone());
                    }
                },
                Ok(request) = reqres_receiver.recv() => {
                    println!("IncomingRequest msg: {:?}", request);
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
                            println!("notification received: peer: {:?}  noti: {:?}", peer, notification);
                            let noti = P2pNotification {peer, notification};
                            let msg = NucleusP2pMsg::Noti(noti);
                            _ = p2p_cage_tx.send(msg).await;

                        }
                        NotificationEvent::ValidateInboundSubstream {
                            peer,
                            handshake,
                            result_tx,
                        } => {
                            println!("notification ValidateInboundSubstream received: {:?}", peer);
                            println!("notification ValidateInboundSubstream received: {:?}", handshake);
                            // to build the notification substream
                            // _ = result_tx.send(ValidationResult::Accept);
                        }
                        NotificationEvent::NotificationStreamOpened {
                            peer,
                            direction,
                            handshake,
                            negotiated_fallback,
                        } => {
                            println!("notification NotificationStreamOpened received: {:?}", peer);
                            println!("notification NotificationStreamOpened received: {:?}", direction);
                            println!("notification NotificationStreamOpened received: {:?}", handshake);
                            println!("notification NotificationStreamOpened received: {:?}", negotiated_fallback);
                        }
                        NotificationEvent::NotificationStreamClosed { peer } => {
                            println!("notification NotificationStreamClosed received: {:?}", peer);
                        }
                    }
                }
            }
        }
    }
}

pub async fn send_request(
    net_service: Arc<dyn sc_network::service::traits::NetworkService>,
    node_id: &PeerId,
    data: Vec<u8>,
) -> Result<Vec<u8>, ()> {
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
        println!(
            "Response of the request is: {}: {:?}",
            name,
            std::str::from_utf8(&res).expect("not a valid ascii string.")
        );
        return Ok(res);
    } else {
        println!("Error on response of the request {:?}", result);
        return Err(());
    }
}
