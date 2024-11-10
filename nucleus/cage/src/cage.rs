use crate::{
    host_func::{
        http::{HttpCallRegister, HttpResponseWithCallback},
        timer::SchedulerAsync,
    },
    nucleus::Nucleus,
    state::NucleusState,
    Event, Gluon, NucleusResponse, ReplyTo, Runtime, RuntimeParams, TimerEntry, TimersReplyTo,
    WasmCodeRef, WasmInfo,
};
use codec::Encode;
use futures::prelude::*;
use sc_client_api::{Backend, BlockBackend, BlockchainEvents, StorageProvider};
use sc_network::request_responses::IncomingRequest;
use sc_network::request_responses::OutgoingResponse;
// use sc_network::service::traits::NotificationEvent;
// use sc_network::service::traits::NotificationService;
use sc_network::service::traits::NetworkService;
use sc_network::PeerId;
use sp_api::{Metadata, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_keystore::KeystorePtr;
use std::collections::HashMap;
use stream::FuturesUnordered;
// TODO use UnboundedSender to avoid blocking
use std::sync::mpsc::Sender as SyncSender;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    oneshot,
};
use vrs_metadata::{
    codegen, config::SubstrateConfig, events, metadata, Metadata as RuntimeMetadata, METADATA_BYTES,
};
use vrs_nucleus_p2p::NucleusP2pMsg;
use vrs_nucleus_runtime_api::NucleusApi;
use vrs_primitives::{AccountId, Hash, NodeId, NucleusId, NucleusInfo};

pub struct CageParams<B, C, BN> {
    pub keystore: KeystorePtr,
    pub nucleus_rpc_rx: Receiver<(NucleusId, Gluon)>,
    pub p2p_cage_rx: Receiver<NucleusP2pMsg>,
    pub noti_sender: Sender<(Vec<u8>, Vec<PeerId>)>,
    pub net_service: Arc<dyn NetworkService>,
    // pub noti_service: Box<dyn NotificationService>,
    pub client: Arc<C>,
    pub controller: AccountId,
    pub nucleus_home_dir: std::path::PathBuf,
    pub _phantom: std::marker::PhantomData<(B, BN)>,
}

struct NucleusCage {
    nucleus_id: NucleusId,
    tunnel: SyncSender<(u64, Gluon)>,
    pending_requests: Vec<Gluon>,
    event_id: u64,
    state: Arc<NucleusState>,
    // TODO monadring-related
}

impl NucleusCage {
    fn validate_token(&self) -> bool {
        true
    }

    fn pre_commit(&self, id: u64, msg: &[u8]) -> anyhow::Result<()> {
        // let handle = self.db.cf_handle("seq").unwrap();
        // self.db.put_cf(handle, &id.to_be_bytes(), msg)?;
        Ok(())
    }

    fn drain(&mut self, imports: Vec<Event>) {
        // TODO handle imports first
        // for `TimerRegister` and `HttpRequest`, we need to check its id
        let pipe = self.pending_requests.drain(..).collect::<Vec<_>>();
        for gluon in pipe.into_iter() {
            self.event_id += 1;
            let event = Event::from(&gluon);
            if let Err(e) = self.pre_commit(self.event_id, &event.encode()) {
                log::error!(
                    "couldn't save event {} of nucleus {}: {:?}",
                    self.event_id,
                    self.nucleus_id,
                    e
                );
                // TODO only reply request from rpc
                // if let Some(reply_to) = gluon.2 {
                //     let _ = reply_to.send(Err((-42000, "Event persistence failed.".to_string())));
                // }
            } else {
                let _ = self.tunnel.send((self.event_id, gluon));
            }
        }
    }

    fn forward(&mut self, gluon: Gluon) {
        if matches!(gluon, Gluon::GetRequest { .. }) {
            let _ = self.tunnel.send((0, gluon));
        } else {
            self.pending_requests.push(gluon);
        }
    }
}

pub fn start_nucleus_cage<B, C, BN>(params: CageParams<B, C, BN>) -> impl Future<Output = ()>
where
    B: sp_runtime::traits::Block,
    BN: Backend<B>,
    C: BlockBackend<B>
        + StorageProvider<B, BN>
        + BlockchainEvents<B>
        + ProvideRuntimeApi<B>
        + HeaderBackend<B>
        + 'static,
    C::Api: Metadata<B> + 'static,
    C::Api: NucleusApi<B> + 'static,
{
    let CageParams {
        keystore,
        mut nucleus_rpc_rx,
        mut p2p_cage_rx,
        mut noti_sender,
        mut net_service,
        client,
        controller,
        nucleus_home_dir,
        _phantom,
    } = params;
    async move {
        let mut nuclei: HashMap<NucleusId, NucleusCage> = HashMap::new();
        log::info!("📖 Nucleus storage root: {:?}", nucleus_home_dir);
        let nucleus_home_dir = nucleus_home_dir.into_boxed_path();
        // TODO what if our node is far behind the best block?
        let metadata =
            metadata::decode_from(&METADATA_BYTES[..]).expect("failed to decode metadata.");
        let mut registry_monitor = client.every_import_notification_stream();
        let timer_scheduler = Arc::new(crate::host_func::timer::SchedulerAsync::new());
        let mut timers_receivers: FuturesUnordered<oneshot::Receiver<Vec<TimerEntry>>> =
            FuturesUnordered::new();

        let (http_register, mut http_executor) = crate::host_func::http::new_http_manager();
        let http_register = Arc::new(http_register);
        // TODO mock monadring
        //////////////////////////////////////////////////////
        let (token_tx, mut token_rx) = mpsc::unbounded_channel::<NucleusId>();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                match token_tx.send(NucleusId::from([0u8; 32])) {
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
        });
        //////////////////////////////////////////////////////
        log::info!("🔌 Nucleus cage controller: {}", controller);
        loop {
            tokio::select! {
                Some(msg) = p2p_cage_rx.recv() => {
                    // req type is IncomingRequest, process it.
                    match msg {
                        NucleusP2pMsg::ReqRes(req) => {
                            log::info!("in cage: incoming request: {:?}", req);

                            // API: anywhere you want to send request, use like:
                            // let result = vrs_nucleus_p2p::send_request(net_service, keystore, peer_id, payload).await;

                            // let payload = req.payload;
                            // decode the payload to verify the signature
                            // let payload_with_sig: vrs_nucleus_p2p::PayloadWithSignature = payload.decode();
                            // let signature = sp_core::sr25519::Signature::from_raw(payload_with_sig.signature.try_into().unwrap());
                            // let public_key = sp_core::sr25519::Public::from_raw(payload_with_sig.signature.try_into().unwrap());
                            // let msg = payload_with_sig.peer_id;
                            // verify the signature
                            // let verify_result = sp_core::sr25519::Pair::verify(&signature, &msg, &public_key);
                            // match verify_result {
                            //     true => {
                                    // do stuff
                                    // as a response role, use req.pending_response to send oneshot OutgoingResponse back
                                    // let outgoing_msg = OutgoingResponse {
                                    //     result: Ok(b"response from cage req/res handle.".to_vec()),
                                    //     reputation_changes: vec![],
                                    //     sent_feedback: None
                                    // };
                                    // _ = req.pending_response.send(outgoing_msg);
                                // }
                                // false => {
                                    // verified failed, do some report

                            //     }
                            // }
                        }
                        NucleusP2pMsg::Noti(noti) => {
                            log::info!("in cage: incoming notification: {:?}", noti);
                            // process notification here
                            // let payload = noti.notification;
                            // let payload_with_sig: vrs_nucleus_p2p::PayloadWithSignature = payload.decode();
                            // let signature = sp_core::sr25519::Signature::from_raw(payload_with_sig.signature.try_into().unwrap());
                            // let public_key = sp_core::sr25519::Public::from_raw(payload_with_sig.signature.try_into().unwrap());
                            // let msg = payload_with_sig.peer_id;
                            // verify the signature
                            // let verify_result = sp_core::sr25519::Pair::verify(&signature, &msg, &public_key);
                            // match verify_result {
                            //     true => {
                                    // do stuff
                                // }
                                // false => {
                                    // verified failed, do some report
                            //     }
                            // }

                            // API: anywhere you want to send a notification, use like:
                            // the first param is the raw payload without signature, the second one Vec<> is the peer list
                            // _ = noti_sender.send((Vec<u8>, Vec<PeerId>)).await;

                        }
                    }
                },
                block = registry_monitor.next() => {
                    let hash = block.expect("block importing error").hash;
                    // TODO handle deregister as well
                    if let Some(instances) = map_to_nucleus(client.clone(), hash, metadata.clone()) {
                        for (id, config) in instances {
                            let nucleus_path = nucleus_home_dir.join(id.to_string());
                            start_nucleus::<B>(id.clone(), http_register.clone(), timer_scheduler.clone(), config, nucleus_path, &mut nuclei).expect("fail to start nucleus");

                            if let Some(nucleus) = nuclei.get_mut(&id) {
                                let (timer_sender, timer_receiver) = oneshot::channel();
                                timers_receivers.push(timer_receiver);
                                nucleus.forward(Gluon::TimerInitRequest {
                                    pending_timer_queue: timer_sender,
                                });
                            }

                        }
                    }
                },
                Some(timer_entries) = timers_receivers.next()=>{
                    log::info!("⏲️ Timer entries received: {:?}", timer_entries);
                    for entry in timer_entries.expect("Failed to receive timer entries").into_iter(){
                        timer_scheduler.push(entry);
                    }
                },
                timer_entry = timer_scheduler.pop() => {
                    log::info!("⏲️ Timer gluon sent: {:?}", timer_entry);
                    if let Some(entry) = timer_entry {
                        if let Some(nucleus) = nuclei.get_mut(&entry.nucleus_id) {
                            let (timer_sender, timer_receiver) = oneshot::channel();
                            timers_receivers.push(timer_receiver);
                            nucleus.forward(Gluon::TimerRequest {
                                endpoint: entry.func_name,
                                payload: entry.func_params,
                                reply_to: None,
                                pending_timer_queue: timer_sender,
                            });
                        }
                    }
                },
                http_reply = http_executor.recv_response()=>{
                    log::info!("🌐 Http reply: {:?}", http_reply);

                    if let Some(HttpResponseWithCallback {
                        nucleus_id,
                        req_id,
                        response,
                    }) = http_reply{
                        if let Some(nucleus) = nuclei.get_mut(&nucleus_id) {
                            nucleus.forward(Gluon::HttpCallback {
                                request_id: req_id,
                                payload: response,
                            });
                        }

                    }else{
                        log::error!("🌐 No http reply");
                    }
                },
                // http_reply = http_executor.poll() => {
                //     println!("ooooooo");
                //     log::error!("couldn't fork a coroutine to execute http, {:?}", http_reply);
                //     if http_reply.is_err() {
                //         log::error!("couldn't fork a coroutine to execute http, {:?}", http_reply.err());
                //         continue;
                //     }
                //     let HttpResponseWithCallback {
                //         nucleus_id,
                //         req_id,
                //         response,
                //     } = http_reply.expect("already checked").expect("HttpCallRegister must exists;qed");
                //     if let Some(nucleus) = nuclei.get_mut(&nucleus_id) {
                //         nucleus.forward(Gluon::HttpCallback {
                //             request_id: req_id,
                //             payload: response,
                //         });
                //     }
                // },
                req = nucleus_rpc_rx.recv() => {
                    let (module, gluon) = req.expect("fail to receive nucleus request");
                    if let Some(nucleus) = nuclei.get_mut(&module) {
                        nucleus.forward(gluon);
                    } else {
                        reply_directly(gluon, Err((-40004, "Nucleus not found.".to_string())));
                    }
                },
                // TODO replace this with token received
                token = token_rx.recv() => {
                    log::info!("mocking monadring: token {} received.", token.expect("sender closed"));
                    // TODO only drain the associated nucleus
                    nuclei.values_mut().for_each(|nucleus| nucleus.drain(vec![]));
                }
            }
        }
    }
}

fn reply_directly(gluon: Gluon, msg: NucleusResponse) {
    match gluon {
        Gluon::PostRequest { reply_to, .. } | Gluon::GetRequest { reply_to, .. } => {
            // TODO not sure the code upgrade will be handled here in the future
            let _ = reply_to.expect("").send(msg);
        }
        _ => {}
    }
}

fn map_to_nucleus<B, D, C>(
    client: Arc<C>,
    hash: B::Hash,
    metadata: RuntimeMetadata,
) -> Option<Vec<(NucleusId, NucleusInfo<AccountId, Hash, NodeId>)>>
where
    B: sp_runtime::traits::Block,
    D: Backend<B>,
    C: BlockBackend<B>
        + StorageProvider<B, D>
        + BlockchainEvents<B>
        + ProvideRuntimeApi<B>
        + 'static,
    C::Api: NucleusApi<B> + 'static,
{
    let storage_key = storage_key(b"System", b"Events");
    let events = client
        .storage(hash, &storage_key)
        .ok()
        .flatten()
        .map(|v| events::decode_from::<SubstrateConfig>(v.0, metadata))?;

    let api = client.runtime_api();
    let instances = events
        .find::<codegen::nucleus::events::InstanceRegistered>()
        .filter_map(|ev| {
            let id = ev.ok()?.id.0;
            api.get_nucleus_info(hash, NucleusId::from(id))
                .ok()?
                .map(|info| (NucleusId::from(id), info))
        })
        .collect::<Vec<_>>();
    Some(instances)
}

fn storage_key(module: &[u8], storage: &[u8]) -> sp_core::storage::StorageKey {
    let mut bytes = sp_core::twox_128(module).to_vec();
    bytes.extend(&sp_core::twox_128(storage)[..]);
    sp_core::storage::StorageKey(bytes)
}

// TODO
fn start_nucleus<B>(
    id: NucleusId,
    http_register: Arc<HttpCallRegister>,
    timer_scheduler: Arc<SchedulerAsync>,
    nucleus_info: NucleusInfo<AccountId, Hash, NodeId>,
    nucleus_path: std::path::PathBuf,
    nuclei: &mut HashMap<NucleusId, NucleusCage>,
) -> anyhow::Result<()>
where
    B: sp_runtime::traits::Block,
{
    let NucleusInfo {
        name,
        manager,
        wasm_location,
        wasm_hash,
        wasm_version,
        current_event,
        root_state,
        peers,
    } = nucleus_info;
    let name = String::from_utf8(name)?;

    let wasm = WasmInfo {
        account: manager,
        name,
        version: wasm_version,
        code: WasmCodeRef::File(nucleus_path.join("code.wasm")),
    };
    let config = RuntimeParams {
        nucleus_id: id.clone(),
        db_path: nucleus_path.join("db").into_boxed_path(),
        http_register,
        timer_scheduler,
    };
    let runtime = Runtime::init(config)?;
    let state = runtime.state.clone();
    let (tx, rx) = std::sync::mpsc::channel();
    if let Ok(mut nucleus) = Nucleus::init(rx, runtime, wasm) {
        // FIXME how to notify the user that his code is invalid?
        std::thread::spawn(move || {
            nucleus.run();
        });
        nuclei.insert(
            id.clone(),
            NucleusCage {
                nucleus_id: id,
                tunnel: tx,
                pending_requests: vec![],
                event_id: current_event,
                state,
            },
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nucleus::Nucleus;
    use crate::test_suite::new_mock_nucleus;
    use std::sync::Arc;
    use std::thread;
    use stream::FuturesUnordered;
    use tokio::task;

    struct ResultProcessor {
        receiver: tokio::sync::mpsc::Receiver<NucleusResponse>,
    }
    impl ResultProcessor {
        fn new() -> tokio::sync::oneshot::Sender<NucleusResponse> {
            let (sender, receiver) = tokio::sync::oneshot::channel();
            thread::spawn(move || async move {
                let reply_to = receiver.await;
                println!("reply: {:?}", reply_to);
            });
            sender
        }
    }
    fn decode(data: Vec<u8>) -> i32 {
        let mut reply = data.as_slice();
        let reply = <Result<i32, String> as codec::Decode>::decode(&mut reply)
            .unwrap()
            .unwrap();
        reply
    }
    #[tokio::test]
    async fn test_scheduler_async() {
        let wasm_path = "../../nucleus-examples/timer.wasm";
        let (sender_cage, receiver_cage) = std::sync::mpsc::channel();
        let (mut nucleus, _): (Nucleus<Runtime>, crate::test_suite::OutOfRuntime) =
            new_mock_nucleus(receiver_cage, wasm_path.to_string());
        let sender_cage_clone = sender_cage.clone();
        let sc = Arc::new(crate::host_func::timer::SchedulerAsync::new());
        task::spawn_blocking(move || {
            nucleus.run();
        });
        let mut receivers = FuturesUnordered::new();
        let (sender_timer_reply, receiver_timer_reply) = tokio::sync::oneshot::channel();
        receivers.push(receiver_timer_reply);
        sender_cage
            .send((
                0,
                Gluon::TimerInitRequest {
                    pending_timer_queue: sender_timer_reply,
                },
            ))
            .unwrap();
        let es = receivers.next().await.unwrap().unwrap();
        println!("{}", es.len());
        for e in es.into_iter() {
            println!("{:?}", e);
            sc.push(e);
        }
        // let (sender_reply, receiver_reply) = tokio::sync::oneshot::channel();
        // sender_cage
        //     .send((
        //         0,
        //         Gluon::TimerRequest {
        //             endpoint: "test_stream".to_owned(),
        //             payload: <(i32, i32) as codec::Encode>::encode(&(1, 0)),
        //             // reply_to: Some(sender_reply),
        //             reply_to: Some(sender_reply),
        //         },
        //     ))
        //     .unwrap();
        // println!("{:?}", reply.1);
        // assert_eq!(decode(reply.0), 555);
        let mut last = 0;
        let result = [0, 0, 1, 2, 2, 3, 3, 4, 3, 4, 4, 5, 4, 5, 5, 6];
        // println!("aaa");
        let mut len = 2;
        while let Some(entry) = sc.pop().await {
            let sender = sender_cage_clone.clone();
            let (sender_timer_reply, receiver_timer_reply) = tokio::sync::oneshot::channel();
            receivers.push(receiver_timer_reply);
            let (sender_reply, receiver_reply) = tokio::sync::oneshot::channel();

            if let Err(err) = sender.send((
                0,
                Gluon::TimerRequest {
                    endpoint: entry.func_name,
                    payload: entry.func_params,
                    // reply_to: Some(sender_reply),
                    // pending_timer_queue: sender_timer_reply,
                },
            )) {
                println!("fail to send timer entry: {:?}", err);
            }
            let reply = receiver_reply.await.unwrap().unwrap();
            println!("reply: {:?}", reply);
            assert!(result[decode(reply.clone()) as usize] >= last);
            last = result[decode(reply) as usize];
            let es = receivers.next().await.unwrap().unwrap();
            for e in es.into_iter() {
                sc.push(e);
            }
            len += 1;
            if len == result.len() {
                break;
            }
            // task::spawn_blocking(move || async move {
            //     let reply = receiver_reply.await.unwrap();
            //     println!("reply: {:?}", reply);
            // });
        }
    }
}

#[cfg(test)]
mod forum_tests {
    use super::*;
    use crate::nucleus::Nucleus;
    use crate::test_suite::new_mock_nucleus;
    use codec::Decode;
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;
    use std::thread;
    use stream::FuturesUnordered;
    use tokio::task;

    struct ResultProcessor {
        receiver: tokio::sync::mpsc::Receiver<NucleusResponse>,
    }
    impl ResultProcessor {
        fn new() -> tokio::sync::oneshot::Sender<NucleusResponse> {
            let (sender, receiver) = tokio::sync::oneshot::channel();
            thread::spawn(move || async move {
                let reply_to = receiver.await;
                println!("reply: {:?}", reply_to);
            });
            sender
        }
    }
    fn decode(data: Vec<u8>) -> i32 {
        let mut reply = data.as_slice();
        let reply = <Result<i32, String> as codec::Decode>::decode(&mut reply)
            .unwrap()
            .unwrap();
        reply
    }
    #[derive(Debug, Decode, Encode, Deserialize, Serialize)]
    pub struct VeArticle {
        pub id: u64,
        pub title: String,
        pub content: String,
        pub author_id: u64,
        pub author_nickname: String,
        pub subspace_id: u64,
        pub ext_link: String,
        pub status: i16,
        pub weight: i16,
        pub created_time: i64,
        pub updated_time: i64,
    }
    #[tokio::test]
    async fn test_forum() {
        let wasm_path = "../../../veforum/veavs.wasm";
        let (sender_cage, receiver_cage) = std::sync::mpsc::channel();
        let (mut nucleus, mut out_of_runtime): (Nucleus<Runtime>, crate::test_suite::OutOfRuntime) =
            new_mock_nucleus(receiver_cage, wasm_path.to_string());
        let sender_cage_clone = sender_cage.clone();
        let sc = Arc::new(crate::host_func::timer::SchedulerAsync::new());
        task::spawn_blocking(move || {
            nucleus.run();
        });
        let mut receivers = FuturesUnordered::new();
        let (sender_timer_reply, receiver_timer_reply) = tokio::sync::oneshot::channel();
        receivers.push(receiver_timer_reply);
        sender_cage
            .send((
                0,
                Gluon::TimerInitRequest {
                    pending_timer_queue: sender_timer_reply,
                },
            ))
            .unwrap();
        // let reply = receiver_reply.await.unwrap().unwrap();
        let es = receivers.next().await.unwrap().unwrap();
        println!("{}", es.len());
        for e in es.into_iter() {
            sc.push(e);
        }

        tokio::spawn(async move {
            while let Some(entry) = sc.pop().await {
                let sender = sender_cage_clone.clone();
                // let (sender_timer_reply, receiver_timer_reply) = tokio::sync::oneshot::channel();
                // receivers.push(receiver_timer_reply);
                let sender = sender_cage_clone.clone();
                let (sender_timer_reply, receiver_timer_reply) = tokio::sync::oneshot::channel();
                receivers.push(receiver_timer_reply);
                let (sender_reply, receiver_reply) = tokio::sync::oneshot::channel();

                if let Err(err) = sender.send((
                    0,
                    Gluon::TimerRequest {
                        endpoint: entry.func_name,
                        payload: entry.func_params,
                        reply_to: Some(sender_reply),
                        pending_timer_queue: sender_timer_reply,
                    },
                )) {
                    println!("fail to send timer entry: {:?}", err);
                }
                let reply = receiver_reply.await.unwrap().unwrap();
                let es = receivers.next().await.unwrap().unwrap();
                for e in es.into_iter() {
                    sc.push(e);
                }
            }
        });

        let sender = sender_cage.clone();
        tokio::spawn(async move {
            loop {
                let article = VeArticle {
                    id: 0,
                    title: "title".to_string(),
                    content: "Today is a good day".to_string(),
                    author_id: 0,
                    author_nickname: "AI Assistant".to_string(),
                    subspace_id: 0,
                    ext_link: "ext_link".to_string(),
                    status: 0,
                    weight: 0,
                    created_time: chrono::Utc::now().timestamp(),
                    updated_time: 0,
                };
                let (article_tx, article_rx) = oneshot::channel();
                let article_post = Gluon::PostRequest {
                    endpoint: "add_article".to_string(),
                    payload: article.encode(),
                    reply_to: Some(article_tx),
                };
                sender.clone().send((0, article_post)).unwrap();
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        });
        tokio::spawn(async move {
            loop {
                let http_reply = out_of_runtime.http_executor.recv_response().await.unwrap();
                let HttpResponseWithCallback {
                    nucleus_id,
                    req_id,
                    response,
                } = http_reply;
                // let http_reply = out_of_runtime.http_executor.poll().await;
                // let HttpResponseWithCallback {
                //     nucleus_id,
                //     req_id,
                //     response,
                // } = http_reply
                //     .expect("already checked")
                //     .expect("HttpCallRegister must exists;qed");
                sender_cage
                    .clone()
                    .send((
                        0,
                        Gluon::HttpCallback {
                            request_id: req_id,
                            payload: response,
                        },
                    ))
                    .unwrap();
            }
        });
        tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{nucleus::Nucleus, scheduler, vm::Vm, Scheduler, WrappedSchedulerSync};
//     use codec::Decode;
//     use futures::channel::mpsc::SendError;
//     use rocksdb::Options;
//     use std::thread;
//     use std::{sync::Arc, time::Duration};
//     use temp_dir::TempDir;
//     use tokio::sync::oneshot;
//     use tokio::{sync::mpsc, task, time};
//     use vrs_core_sdk::AccountId;
//     struct ResultProcessor {
//         receiver: tokio::sync::mpsc::Receiver<NucleusResponse>,
//     }
//     impl ResultProcessor {
//         fn new() -> tokio::sync::oneshot::Sender<NucleusResponse> {
//             let (sender, receiver) = tokio::sync::oneshot::channel();
//             thread::spawn(move || async move {
//                 let reply_to = receiver.await;
//                 println!("reply: {:?}", reply_to);
//             });
//             sender
//         }
//     }
//     #[test]
//     fn test_scheduler() {
//         let wasm_path = "../../nucleus-examples/vrs_nucleus_examples.wasm";
//         let wasm = WasmInfo {
//             account: AccountId::new([0u8; 32]),
//             name: "avs-dev-demo".to_string(),
//             version: 0,
//             code: WasmCodeRef::File(wasm_path.to_string()),
//         };

//         let tmp_dir = TempDir::new().unwrap();
//         let context = Context::init(ContextConfig {
//             db_path: tmp_dir.child("0").into_boxed_path(),
//         })
//         .unwrap();
//         let (sender_cage, receiver_cage) = std::sync::mpsc::channel();
//         let (scheduler, receiver_entry_cage) = WrappedSchedulerSync::new();
//         let scheduler = Arc::new(scheduler);
//         let mut nucleus = Nucleus::new(receiver_cage, scheduler, context, wasm);
//         let sender_cage_clone = sender_cage.clone();
//         thread::spawn(move || {
//             nucleus.run();
//         });

//         thread::spawn(move || {
//             let (sender_reply, receiver_reply) = tokio::sync::oneshot::channel();
//             let send_r = sender_cage.send((
//                 0,
//                 Gluon::PostRequest {
//                     endpoint: "test_set_perfect_tree_mod_timer".to_owned(),
//                     payload: <(i32, i32) as codec::Encode>::encode(&(1, 0)),
//                     // reply_to: Some(sender_reply),
//                     reply_to: Some(sender_reply),
//                 },
//             ));
//             println!("reply to original: {:?}", send_r);
//             thread::spawn(move || {
//                 let reply = receiver_reply.blocking_recv().unwrap();
//                 println!("reply: {:?}", reply);
//             });
//         });
//         thread::spawn(move || {
//             while let Ok(entry) = receiver_entry_cage.recv() {
//                 println!("{:?}", entry);
//                 let sender = sender_cage_clone.clone();

//                 thread::spawn(move || {
//                     let (sender_reply, receiver_reply) = tokio::sync::oneshot::channel();
//                     if let Err(err) = sender.send((
//                         0,
//                         Gluon::PostRequest {
//                             endpoint: entry.1.func_name,
//                             payload: entry.1.func_params,
//                             reply_to: Some(sender_reply),
//                         },
//                     )) {
//                         println!("fail to send timer entry: {:?}", err);
//                     }
//                     thread::spawn(move || {
//                         let reply = receiver_reply.blocking_recv().unwrap();
//                         println!("reply: {:?}", reply);
//                     });

//                     // task::spawn_blocking(move || async move {
//                     //     let reply = receiver_reply.await.unwrap();
//                     //     println!("reply: {:?}", reply);
//                     // });
//                 });
//             }
//         });
//         std::thread::sleep(Duration::from_secs(12));
//     }
// }
