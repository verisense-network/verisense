use crate::{
    host_func::{
        http::{HttpCallRegister, HttpResponseWithCallback},
        timer::SchedulerAsync,
    },
    nucleus::Nucleus,
    state::NucleusState,
    Event, Gluon, NucleusResponse, Runtime, RuntimeParams, WasmInfo,
};
use codec::{Decode, Encode};
use futures::prelude::*;
use sc_client_api::{Backend, BlockBackend, BlockchainEvents, StorageProvider};
use sc_network::{service::traits::NetworkService, PeerId};
use sc_transaction_pool_api::{TransactionPool, TransactionSource};
use sp_api::{Metadata, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_keystore::KeystorePtr;
use std::collections::HashMap;
// TODO use UnboundedSender to avoid blocking
use frame_system_rpc_runtime_api::AccountNonceApi;
use std::sync::mpsc::Sender as SyncSender;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver, Sender};
use vrs_metadata::{
    codegen, config::SubstrateConfig, events, metadata, Metadata as RuntimeMetadata, METADATA_BYTES,
};
use vrs_nucleus_p2p::NucleusP2pMsg;
use vrs_nucleus_runtime_api::{NucleusApi, ValidatorApi};
use vrs_primitives::{keys, AccountId, Address, Hash, NodeId, NucleusId, NucleusInfo};

pub struct CageParams<P, B, C, BN> {
    pub keystore: KeystorePtr,
    pub transaction_pool: Arc<P>,
    pub nucleus_rpc_rx: Receiver<(NucleusId, Gluon)>,
    pub p2p_cage_rx: Receiver<NucleusP2pMsg>,
    pub noti_sender: Sender<(Vec<u8>, Vec<PeerId>)>,
    pub net_service: Arc<dyn NetworkService>,
    // pub noti_service: Box<dyn NotificationService>,
    pub client: Arc<C>,
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

pub fn start_nucleus_cage<P, B, C, BN>(params: CageParams<P, B, C, BN>) -> impl Future<Output = ()>
where
    B: sp_runtime::traits::Block<Extrinsic = sp_runtime::OpaqueExtrinsic>,
    BN: Backend<B>,
    P: TransactionPool<Block = B, Hash = B::Hash> + 'static,
    C: BlockBackend<B>
        + StorageProvider<B, BN>
        + BlockchainEvents<B>
        + ProvideRuntimeApi<B>
        + HeaderBackend<B>
        + 'static,
    C::Api: Metadata<B> + 'static,
    C::Api: ValidatorApi<B> + 'static,
    C::Api: NucleusApi<B, Address, vrs_runtime::RuntimeCall, vrs_runtime::SignedExtra> + 'static,
    C::Api: AccountNonceApi<B, AccountId, u32> + 'static,
{
    let CageParams {
        keystore,
        transaction_pool,
        mut nucleus_rpc_rx,
        mut p2p_cage_rx,
        mut noti_sender,
        mut net_service,
        client,
        nucleus_home_dir,
        _phantom,
    } = params;
    async move {
        let mut nuclei: HashMap<NucleusId, NucleusCage> = HashMap::new();
        let nucleus_home_dir = nucleus_home_dir.into_boxed_path();
        log::info!("📖 Nucleus storage root: {:?}", nucleus_home_dir);

        // TODO what if our node is far behind the best block?
        let metadata =
            metadata::decode_from(&METADATA_BYTES[..]).expect("failed to decode metadata.");

        let mut block_monitor = client.every_import_notification_stream();
        let timer_scheduler = Arc::new(crate::host_func::timer::SchedulerAsync::new());
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
        let author = keystore
            .sr25519_public_keys(sp_core::crypto::key_types::AURA)
            .first()
            .copied()
            .expect("No essential session key found, please insert one");

        let hash = client.info().best_hash;
        let api = client.runtime_api();
        let controller = api
            .is_active_validator(hash, sp_core::crypto::key_types::AURA, author.to_vec())
            .expect("couldn't load runtime api");
        if controller.is_none() {
            log::warn!("Our node is not a validator!");
            return;
        }
        let controller = controller.unwrap();
        let chosen = get_nuclei_for_node(client.clone(), controller.clone(), hash);
        for (id, info) in chosen {
            let nucleus_path = nucleus_home_dir.join(id.to_string());
            start_nucleus(
                id.clone(),
                info,
                nucleus_path.as_path(),
                http_register.clone(),
                timer_scheduler.clone(),
                &mut nuclei,
            )
            .expect("fail to start nucleus");
        }
        log::info!("🔌 Nucleus cage controller: {}", controller);
        loop {
            tokio::select! {
                // handle monadring protocol
                Some(msg) = p2p_cage_rx.recv() => {
                    match msg {
                        NucleusP2pMsg::ReqRes(req) => {
                            log::info!("in cage: incoming request: {:?}", req);
                        }
                        NucleusP2pMsg::Noti(noti) => {
                            log::info!("in cage: incoming notification: {:?}", noti);
                        }
                    }
                },
                // handle hostnet events
                block = block_monitor.next() => {
                    let hash = block.expect("block importing error").hash;
                    let events = extract_events::<_, _, _, SubstrateConfig>(client.clone(), hash, metadata.clone())
                        .expect("fail to extract events");
                    if events.is_none() {
                        continue;
                    }
                    let events = events.unwrap();
                    for event in events.iter() {
                        if let Ok(Some(ev)) = event.as_ref().map(|ev| ev.as_event::<codegen::nucleus::events::NucleusCreated>().ok().flatten()) {
                            let nucleus_id = ev.id;
                            let public_input = ev.public_input;
                            submit_vrf(transaction_pool.clone(), client.clone(), NucleusId::from(nucleus_id.0), keystore.clone(), public_input.as_ref(), hash)
                                .await
                                .inspect_err(|e| log::error!("fail to submit vrf: {:?}", e))
                                .expect("fail to submit vrf");
                        } else if let Ok(Some(ev)) = event.as_ref().map(|ev| ev.as_event::<codegen::nucleus::events::NucleusUpgraded>().ok().flatten()) {
                            let nucleus_id = ev.id;
                            let digest = ev.wasm_hash;
                            let version = ev.wasm_version;
                            // TODO download the wasm from ev.wasm_location
                            let nucleus_path = nucleus_home_dir.join(nucleus_id.to_string());
                            upgrade_nucleus_wasm(
                                NucleusId::from(nucleus_id.0),
                                digest,
                                version,
                                nucleus_path.as_path(),
                                &mut nuclei,
                            ).expect("fail to upgrade nucleus wasm");
                        } else if let Ok(Some(ev)) = event.as_ref().map(|ev| ev.as_event::<codegen::nucleus::events::InstanceRegistered>().ok().flatten()) {
                            let nucleus_id = ev.id;
                            let api = client.runtime_api();
                            let info = api
                                .get_nucleus_info(hash, NucleusId::from(nucleus_id.0))
                                .inspect_err(|e| log::error!("fail to get nucleus info while receiving nucleus created event: {:?}", e))
                                .ok()
                                .flatten()
                                .expect("fail to get nucleus info while receiving nucleus created event");
                            let nucleus_path = nucleus_home_dir.join(nucleus_id.to_string());
                            start_nucleus(
                                NucleusId::from(nucleus_id.0),
                                info,
                                nucleus_path.as_path(),
                                http_register.clone(),
                                timer_scheduler.clone(),
                                &mut nuclei,
                            ).expect("fail to start nucleus");
                        }
                    }
                },
                Some(entry) = timer_scheduler.pop() => {
                    log::info!("⏲️ Timer gluon sent: {:?}", entry);
                    if let Some(nucleus) = nuclei.get_mut(&entry.nucleus_id) {
                        nucleus.forward(Gluon::TimerRequest {
                            endpoint: entry.func_name,
                            payload: entry.func_params,
                        });
                    }
                },
                Some(http_reply) = http_executor.recv_response() => {
                    log::info!("🌐 Http reply: {:?}", http_reply);
                    let HttpResponseWithCallback {
                        nucleus_id,
                        req_id,
                        response,
                    } = http_reply;
                    if let Some(nucleus) = nuclei.get_mut(&nucleus_id) {
                        nucleus.forward(Gluon::HttpCallback {
                            request_id: req_id,
                            payload: response,
                        });
                    }
                },
                Some((module, gluon)) = nucleus_rpc_rx.recv() => {
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

fn extract_events<B, D, C, T>(
    client: Arc<C>,
    hash: B::Hash,
    metadata: RuntimeMetadata,
) -> Result<Option<events::Events<T>>, Box<dyn std::error::Error>>
where
    B: sp_runtime::traits::Block,
    D: Backend<B>,
    C: BlockBackend<B>
        + StorageProvider<B, D>
        + BlockchainEvents<B>
        + ProvideRuntimeApi<B>
        + 'static,
    C::Api: NucleusApi<B, Address, vrs_runtime::RuntimeCall, vrs_runtime::SignedExtra> + 'static,
    T: vrs_metadata::Config,
{
    // TODO change to use the storage key from metadata
    let storage_key = storage_key(b"System", b"Events");
    client
        .storage(hash, &storage_key)
        .inspect_err(|e| log::error!("failed to get events: {:?}", e))
        .map(|b| b.map(|v| events::decode_from::<T>(v.0, metadata)))
        .map_err(|e| e.into())
}

async fn submit_vrf<P, B, D, C>(
    pool: Arc<P>,
    client: Arc<C>,
    nucleus_id: NucleusId,
    keystore: KeystorePtr,
    public_input: &[u8],
    hash: B::Hash,
) -> Result<(), Box<dyn std::error::Error>>
where
    B: sp_runtime::traits::Block<Extrinsic = sp_runtime::OpaqueExtrinsic> + 'static,
    P: TransactionPool<Block = B, Hash = B::Hash> + 'static,
    D: Backend<B>,
    C: BlockBackend<B>
        + StorageProvider<B, D>
        + BlockchainEvents<B>
        + ProvideRuntimeApi<B>
        + 'static,
    C::Api: AccountNonceApi<B, AccountId, u32> + 'static,
    C::Api: NucleusApi<B, Address, vrs_runtime::RuntimeCall, vrs_runtime::SignedExtra> + 'static,
{
    let (submitter, vrf) = crate::keystore::sign_to_participate(
        keystore.clone(),
        keys::NUCLEUS_VRF_KEY_TYPE,
        public_input,
    )?;
    let submitter: AccountId = submitter.into();
    let api = client.runtime_api();
    let nonce = api.account_nonce(hash, submitter.clone())?;
    // TODO
    let (addr, call, extra) = api.compose_vrf_tx(hash, nucleus_id, submitter, nonce, vrf)?;
    let signature = crate::keystore::sign_tx(
        keystore.clone(),
        keys::NUCLEUS_VRF_KEY_TYPE,
        addr.clone(),
        call.clone(),
        extra.clone(),
    )?;
    let tx = vrs_runtime::UncheckedExtrinsic::new_signed(call, addr, signature, extra);
    let xt = sp_runtime::OpaqueExtrinsic::from(tx);
    pool.submit_one(hash, TransactionSource::External, xt)
        .await?;
    Ok(())
}

fn get_nuclei_for_node<B, D, C>(
    client: Arc<C>,
    id: AccountId,
    hash: B::Hash,
) -> Vec<(NucleusId, NucleusInfo<AccountId, Hash, NodeId>)>
where
    B: sp_runtime::traits::Block,
    D: Backend<B>,
    C: BlockBackend<B>
        + StorageProvider<B, D>
        + BlockchainEvents<B>
        + ProvideRuntimeApi<B>
        + 'static,
    C::Api: NucleusApi<B, Address, vrs_runtime::RuntimeCall, vrs_runtime::SignedExtra> + 'static,
{
    let api = client.runtime_api();
    let key = codegen::storage().nucleus().instances_iter();
    let instance_key = sp_core::storage::StorageKey(key.to_root_bytes());
    let list = client
        .storage(hash, &instance_key)
        .ok()
        .flatten()
        .map(|data| <Vec<(NucleusId, Vec<AccountId>)> as Decode>::decode(&mut &data.0[..]).ok())
        .flatten()
        .unwrap_or_default()
        .into_iter()
        .filter(|(_, instances)| instances.iter().find(|&ist| *ist == id).is_some())
        .filter_map(|(nucleus_id, _)| {
            api.get_nucleus_info(hash, nucleus_id.clone())
                .ok()?
                .map(|info| (nucleus_id, info))
        })
        .collect::<Vec<_>>();
    list
}

fn storage_key(module: &[u8], storage: &[u8]) -> sp_core::storage::StorageKey {
    let mut bytes = sp_core::twox_128(module).to_vec();
    bytes.extend(&sp_core::twox_128(storage)[..]);
    sp_core::storage::StorageKey(bytes)
}

fn upgrade_nucleus_wasm(
    id: NucleusId,
    digest: Hash,
    version: u32,
    nucleus_root_path: &std::path::Path,
    nuclei: &mut HashMap<NucleusId, NucleusCage>,
) -> anyhow::Result<()> {
    let wasm_path = nucleus_root_path.join(format!("wasm/{}.wasm", version));
    let code = std::fs::read(&wasm_path)?;
    let wasm = WasmInfo {
        id: id.clone(),
        version,
        code,
    };
    let mut cage = nuclei
        .get_mut(&id)
        .ok_or(anyhow::anyhow!("Nucleus not found"))?;
    cage.forward(Gluon::CodeUpgrade {
        version,
        digest: wasm.digest(),
        wasm: Some(wasm),
    });
    Ok(())
}

fn start_nucleus(
    id: NucleusId,
    nucleus_info: NucleusInfo<AccountId, Hash, NodeId>,
    nucleus_root_path: &std::path::Path,
    http_register: Arc<HttpCallRegister>,
    timer_scheduler: Arc<SchedulerAsync>,
    nuclei: &mut HashMap<NucleusId, NucleusCage>,
) -> anyhow::Result<()> {
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
    let config = RuntimeParams {
        nucleus_id: id.clone(),
        db_path: nucleus_root_path.join("db").into_boxed_path(),
        http_register,
        timer_scheduler,
    };
    let runtime = Runtime::init(config)?;
    let state = runtime.state.clone();
    let (tx, rx) = std::sync::mpsc::channel();
    let mut nucleus = Nucleus::init(rx, runtime);
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
    fn decode<T: Decode>(data: Vec<u8>) -> T {
        let mut reply = data.as_slice();
        println!("reply: {:?}", reply);
        let reply = <Result<T, String> as codec::Decode>::decode(&mut reply)
            .unwrap()
            .unwrap();
        reply
    }
    #[tokio::test]
    async fn test_scheduler_async() {
        let wasm_path = "../../nucleus-examples/timer.wasm";
        let (out_of_runtime, sender_cage) = new_mock_nucleus(wasm_path.to_string());
        let sender1 = sender_cage.clone();
        let (tree_tx, mut tree_rx) = tokio::sync::mpsc::channel(100);
        tokio::spawn(async move {
            while let Some(entry) = out_of_runtime.scheduler.pop().await {
                println!("entry: {:?}", entry);
                if entry.func_name == "test_set_perfect_tree_mod_timer" {
                    let d =
                        <(i32, i32) as Decode>::decode(&mut entry.func_params.as_slice()).unwrap();
                    println!("d: {:?}", d);
                    tree_tx.send(d).await.unwrap();
                }
                sender1
                    .send((
                        0,
                        Gluon::TimerRequest {
                            endpoint: entry.func_name,
                            payload: entry.func_params,
                        },
                    ))
                    .unwrap();
            }
        });
        let (sender_reply, receiver_reply) = tokio::sync::oneshot::channel();
        sender_cage
            .send((
                0,
                Gluon::PostRequest {
                    endpoint: "test_set_timer".to_owned(),
                    payload: <(i32, i32) as codec::Encode>::encode(&(1, 0)),
                    // reply_to: Some(sender_reply),
                    reply_to: Some(sender_reply),
                },
            ))
            .unwrap();
        let (sender_reply, receiver_reply) = tokio::sync::oneshot::channel();
        sender_cage
            .send((
                0,
                Gluon::GetRequest {
                    endpoint: "test_get_timer".to_owned(),
                    payload: vec![],
                    reply_to: Some(sender_reply),
                },
            ))
            .unwrap();
        let reply = receiver_reply.await.unwrap().unwrap();
        let mut reply = reply.as_slice();
        let reply = <Result<String, String> as codec::Decode>::decode(&mut reply)
            .unwrap()
            .unwrap();
        assert_eq!(reply, "init");
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        let (sender_reply, receiver_reply) = tokio::sync::oneshot::channel();
        sender_cage
            .send((
                0,
                Gluon::GetRequest {
                    endpoint: "test_get_timer".to_owned(),
                    payload: vec![],
                    reply_to: Some(sender_reply),
                },
            ))
            .unwrap();
        let reply = receiver_reply.await.unwrap().unwrap();
        assert_eq!(decode::<String>(reply), "delay_complete abc 123");
        let mut last_d = (0, 0);
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        while let Some(d) = tree_rx.recv().await {
            assert!(d.1 >= last_d.1);
            last_d = d;
            if d.0 == 15 {
                break;
            }
        }
        // println!("{:?}", reply.1);
        // assert_eq!(decode(reply.0), 555);
        // let mut last = 0;
        // let result = [0, 0, 1, 2, 2, 3, 3, 4, 3, 4, 4, 5, 4, 5, 5, 6];
        // // println!("aaa");
        // let mut len = 2;
        // while let Some(entry) = sc.pop().await {
        //     let sender = sender_cage_clone.clone();
        //     let (sender_timer_reply, receiver_timer_reply) = tokio::sync::oneshot::channel();
        //     receivers.push(receiver_timer_reply);
        //     let (sender_reply, receiver_reply) = tokio::sync::oneshot::channel();

        //     if let Err(err) = sender.send((
        //         0,
        //         Gluon::TimerRequest {
        //             endpoint: entry.func_name,
        //             payload: entry.func_params,
        //             // reply_to: Some(sender_reply),
        //             // pending_timer_queue: sender_timer_reply,
        //         },
        //     )) {
        //         println!("fail to send timer entry: {:?}", err);
        //     }
        //     let reply = receiver_reply.await.unwrap().unwrap();
        //     println!("reply: {:?}", reply);
        //     assert!(result[decode(reply.clone()) as usize] >= last);
        //     last = result[decode(reply) as usize];
        //     let es = receivers.next().await.unwrap().unwrap();
        //     for e in es.into_iter() {
        //         sc.push(e);
        //     }
        //     len += 1;
        //     if len == result.len() {
        //         break;
        //     }
        //     // task::spawn_blocking(move || async move {
        //     //     let reply = receiver_reply.await.unwrap();
        //     //     println!("reply: {:?}", reply);
        //     // });
        // }
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
    // #[tokio::test]
    // async fn test_forum() {
    //     let wasm_path = "../../../veforum/veavs.wasm";
    //     let (sender_cage, receiver_cage) = std::sync::mpsc::channel();
    //     let (mut nucleus, mut out_of_runtime): (Nucleus<Runtime>, crate::test_suite::OutOfRuntime) =
    //         new_mock_nucleus(receiver_cage, wasm_path.to_string());
    //     let sender_cage_clone = sender_cage.clone();
    //     let sc = Arc::new(crate::host_func::timer::SchedulerAsync::new());
    //     task::spawn_blocking(move || {
    //         nucleus.run();
    //     });
    //     let mut receivers = FuturesUnordered::new();
    //     let (sender_timer_reply, receiver_timer_reply) = tokio::sync::oneshot::channel();
    //     receivers.push(receiver_timer_reply);
    //     // sender_cage
    //     //     .send((
    //     //         0,
    //     //         Gluon::TimerInitRequest {
    //     //             pending_timer_queue: sender_timer_reply,
    //     //         },
    //     //     ))
    //     //     .unwrap();
    //     // let reply = receiver_reply.await.unwrap().unwrap();
    //     let es = receivers.next().await.unwrap().unwrap();
    //     println!("{}", es.len());
    //     for e in es.into_iter() {
    //         sc.push(e);
    //     }

    //     tokio::spawn(async move {
    //         while let Some(entry) = sc.pop().await {
    //             let sender = sender_cage_clone.clone();
    //             // let (sender_timer_reply, receiver_timer_reply) = tokio::sync::oneshot::channel();
    //             // receivers.push(receiver_timer_reply);
    //             let sender = sender_cage_clone.clone();
    //             let (sender_timer_reply, receiver_timer_reply) = tokio::sync::oneshot::channel();
    //             receivers.push(receiver_timer_reply);
    //             let (sender_reply, receiver_reply) = tokio::sync::oneshot::channel();

    //             if let Err(err) = sender.send((
    //                 0,
    //                 Gluon::TimerRequest {
    //                     endpoint: entry.func_name,
    //                     payload: entry.func_params,
    //                 },
    //             )) {
    //                 println!("fail to send timer entry: {:?}", err);
    //             }
    //             let reply = receiver_reply.await.unwrap().unwrap();
    //             let es = receivers.next().await.unwrap().unwrap();
    //             for e in es.into_iter() {
    //                 sc.push(e);
    //             }
    //         }
    //     });

    //     let sender = sender_cage.clone();
    //     tokio::spawn(async move {
    //         loop {
    //             let article = VeArticle {
    //                 id: 0,
    //                 title: "title".to_string(),
    //                 content: "Today is a good day".to_string(),
    //                 author_id: 0,
    //                 author_nickname: "AI Assistant".to_string(),
    //                 subspace_id: 0,
    //                 ext_link: "ext_link".to_string(),
    //                 status: 0,
    //                 weight: 0,
    //                 created_time: chrono::Utc::now().timestamp(),
    //                 updated_time: 0,
    //             };
    //             let (article_tx, article_rx) = oneshot::channel();
    //             let article_post = Gluon::PostRequest {
    //                 endpoint: "add_article".to_string(),
    //                 payload: article.encode(),
    //                 reply_to: Some(article_tx),
    //             };
    //             sender.clone().send((0, article_post)).unwrap();
    //             tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    //         }
    //     });
    //     tokio::spawn(async move {
    //         loop {
    //             let http_reply = out_of_runtime.http_executor.recv_response().await.unwrap();
    //             let HttpResponseWithCallback {
    //                 nucleus_id,
    //                 req_id,
    //                 response,
    //             } = http_reply;
    //             // let http_reply = out_of_runtime.http_executor.poll().await;
    //             // let HttpResponseWithCallback {
    //             //     nucleus_id,
    //             //     req_id,
    //             //     response,
    //             // } = http_reply
    //             //     .expect("already checked")
    //             //     .expect("HttpCallRegister must exists;qed");
    //             sender_cage
    //                 .clone()
    //                 .send((
    //                     0,
    //                     Gluon::HttpCallback {
    //                         request_id: req_id,
    //                         payload: response,
    //                     },
    //                 ))
    //                 .unwrap();
    //         }
    //     });
    //     tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
    // }
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
