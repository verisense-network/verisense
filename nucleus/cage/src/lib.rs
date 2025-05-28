mod cage;
mod keystore;

use crate::cage::{
    MonadringToken, MonadringTokenItem, MonadringVerifyResult, NucleusCage, QueryEventsResult,
};
use codec::{Decode, Encode};
use frame_system_rpc_runtime_api::AccountNonceApi;
use futures::channel::oneshot;
use futures::prelude::*;
use sc_authority_discovery::Service as AuthorityDiscovery;
use sc_client_api::{Backend, BlockBackend, BlockchainEvents, StorageProvider};
use sc_network::request_responses::OutgoingResponse;
use sc_network::{service::traits::NetworkService, PeerId};
use sc_transaction_pool_api::{TransactionPool, TransactionSource};
use sp_api::{Core, Metadata, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_core::ByteArray;
use sp_keystore::KeystorePtr;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::Mutex;
use vrs_metadata::{
    codegen, config::SubstrateConfig, events, metadata, Metadata as RuntimeMetadata,
    METADATA_BYTES as METADATA,
};
use vrs_nucleus_executor::{
    host_func::{self, HttpCallRegister, HttpResponseWithCallback, SchedulerAsync},
    Event, Gluon, Nucleus, NucleusResponse, Runtime, RuntimeParams, WasmInfo,
};
use vrs_nucleus_p2p::{Destination, PayloadWithSignature, RequestContent, SendMessage};
use vrs_nucleus_runtime_api::{NucleusApi, ValidatorApi};
use vrs_primitives::{keys, AccountId, Hash, NodeId, NucleusId, NucleusInfo};

pub type NucleusRpcChannel = Sender<(NucleusId, Gluon)>;
pub type NucleusSignal = Receiver<(NucleusId, Gluon)>;

pub struct CageParams<P, B, C, BN> {
    pub client: Arc<C>,
    pub keystore: KeystorePtr,
    pub transaction_pool: Arc<P>,
    pub authority_discovery: Arc<Mutex<AuthorityDiscovery>>,
    pub nucleus_signal: NucleusSignal,
    pub net_service: Arc<dyn NetworkService>,
    pub tss_node: Arc<vrs_tss::NodeRuntime>,
    pub nucleus_home_dir: std::path::PathBuf,
    pub p2p_cage_rx: Receiver<(
        PayloadWithSignature,
        PeerId,
        oneshot::Sender<OutgoingResponse>,
    )>,
    pub cage_p2p_tx: Sender<(SendMessage, oneshot::Sender<Vec<u8>>)>,
    pub _phantom: std::marker::PhantomData<(B, BN)>,
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
    C::Api: Core<B> + 'static,
    C::Api: Metadata<B> + 'static,
    C::Api: ValidatorApi<B> + 'static,
    C::Api: NucleusApi<B> + 'static,
    C::Api: AccountNonceApi<B, AccountId, u32> + 'static,
{
    let CageParams {
        client,
        keystore,
        transaction_pool,
        authority_discovery,
        nucleus_signal,
        net_service,
        tss_node,
        nucleus_home_dir,
        mut p2p_cage_rx,
        cage_p2p_tx,
        _phantom,
    } = params;
    async move {
        let mut nuclei: HashMap<NucleusId, NucleusCage> = HashMap::new();
        let mut nucleus_signal = nucleus_signal;
        let nucleus_home_dir = nucleus_home_dir.into_boxed_path();
        init_nucleus_home(&nucleus_home_dir);
        log::info!("📖 Nucleus storage root: {:?}", nucleus_home_dir);
        // TODO what if our node is far behind the best block?
        // TODO use the best block hash to get the metadata
        let metadata = metadata::decode_from(&METADATA[..]).expect("failed to decode metadata.");
        let mut block_monitor = client.every_import_notification_stream();
        let timer_scheduler = Arc::new(host_func::SchedulerAsync::new());
        let (http_register, mut http_executor) = host_func::new_http_manager();
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
            .sr25519_public_keys(sp_core::crypto::key_types::BABE)
            .first()
            .copied()
            .expect("No essential session key found, please insert one");

        let hash = client.info().best_hash;
        let api = client.runtime_api();
        let controller = api
            .is_active_validator(hash, sp_core::crypto::key_types::BABE, author.to_vec())
            .expect("couldn't load runtime api");
        if controller.is_none() {
            log::warn!("Our node is not a validator!");
            return;
        }
        let self_controller = controller.unwrap();
        let chosen = get_nuclei_for_node(client.clone(), self_controller.clone(), hash);
        let (token_timeout_tx, mut token_timeout_rx) = tokio::sync::mpsc::channel(1000);
        for (id, info) in chosen {
            let nucleus_path = nucleus_home_dir.join(id.to_string());
            start_nucleus(
                id.clone(),
                info,
                nucleus_path,
                http_register.clone(),
                timer_scheduler.clone(),
                tss_node.clone(),
                &mut nuclei,
                token_timeout_tx.clone(),
            )
            .expect("fail to start nucleus");
        }
        log::info!("🔌 Nucleus cage controller: {}", self_controller);
        loop {
            tokio::select! {
                // handle monadring protocol
                Some((msg,source, resp_sender)) = p2p_cage_rx.recv() => {
                    log::info!("in cage: incoming request: {:?}", msg);
                    let Ok(req) = RequestContent::decode(&mut &msg.payload[..]) else {
                        let _ = resp_sender.send(create_outgoing("ERR".encode()));
                        continue;
                    };
                    match req {
                        RequestContent::SendToken(content) => {
                            if let Ok(token) = MonadringToken::decode(&mut &content[..]) {
                                if let Some(nucleus) = nuclei.get_mut(&token.nucleus_id){
                                    match nucleus.validate_token(&self_controller,&token) {
                                        MonadringVerifyResult::AllGood => {
                                            resp_sender.send(create_outgoing("OK".encode()));
                                            let events = token.combine_events(&self_controller);
                                            let mut token = token;
                                            if token.ring.first().is_some_and(|r|r.source == self_controller){
                                                token.ring.remove(0);
                                            }
                                            let new_events = nucleus.drain(events);
                                            let last_event_id = nucleus.event_id;
                                            let state_root = nucleus.state.get_state_root();
                                            let item = MonadringTokenItem {
                                                events:new_events,
                                                nucleus_state_root: state_root,
                                                last_event_id,
                                                source: self_controller.clone(),
                                                signature: Default::default(),
                                            };
                                            token.ring.push(item);
                                            let payload = token.encode();
                                            let mut controllers = get_nucleus_controllers(client.clone(), token.nucleus_id, client.info().best_hash);
                                            if !controllers.is_empty() {
                                                loop {
                                                    let first = controllers.remove(0);
                                                    controllers.push(first.clone());
                                                    if first == self_controller {
                                                        break;
                                                    }
                                                }
                                            }
                                            for c in controllers {
                                                 if c == self_controller {
                                                    continue;
                                                 }
                                                 let authority = sp_authority_discovery::AuthorityId::from_slice(c.as_slice()).unwrap();
                                                 let msg = SendMessage {
                                                    dest: Destination::AuthorityId(authority),
                                                    request: RequestContent::SendToken(payload.clone()),
                                                };
                                                let (resp_tx, resp_rx) = oneshot::channel::<Vec<u8>>();
                                                match cage_p2p_tx.send((msg, resp_tx)).await {
                                                    Ok(r) => {
                                                        if let Ok(s) = resp_rx.await {
                                                            if s == "OK".encode() {
                                                                break;
                                                            }
                                                        }
                                                    }
                                                    Err(_) => {

                                                    }
                                                }
                                            }
                                        }
                                        MonadringVerifyResult::Failed => {
                                            let _ = resp_sender.send(create_outgoing("ERR".encode()));
                                        }
                                    }
                                }
                            }
                        }
                        RequestContent::QueryEvents(content) => {
                            let r: Result<(NucleusId, u64), _> = Decode::decode(&mut &content[..]);
                            let Ok((nid, start_event_id)) = r else {
                                continue;
                            };
                            let events = match nuclei.get(&nid) {
                                None => {
                                    vec![]
                                }
                                Some(c) => {
                                    let mut events = vec![];
                                    for id in start_event_id+1 .. start_event_id+50 {
                                        let Ok(evt_opt) =  c.state.get_user_data(&id.to_le_bytes()) else {break;};
                                        let Some(event_encoded) = evt_opt else {break;};
                                        let Ok(ent) = Event::decode(&mut &event_encoded[..]) else {break;};
                                        events.push(ent);
                                    }
                                    events
                                }
                            };
                            let resp = QueryEventsResult {
                                events
                            };
                            let bytes = resp.encode();
                            let outgoing = OutgoingResponse {
                                result: Ok(bytes),
                                reputation_changes: vec![],
                                sent_feedback: None,
                            };
                            let _ = resp_sender.send(outgoing);
                        }
                        RequestContent::QueryCodeWasm(content) => {
                            let r: Result<(NucleusId, u32), _> = Decode::decode(&mut &content[..]);
                            let Ok((nid, wasm_version)) = r else {
                                continue;
                            };
                            let nucleus_path = nucleus_home_dir.join(nid.to_string());
                            let wasm  = match try_load_wasm(nid, nucleus_path.as_path(), wasm_version) {
                                                Ok(w) => { Ok(w.encode())},
                                                Err(_) => { Err(())}
                                            };
                            let outgoing = OutgoingResponse {result: wasm, reputation_changes: vec![],sent_feedback: None,};
                            let _ = resp_sender.send(outgoing);
                        }
                    }
                },
                nid = token_timeout_rx.recv() => {
                    let Some(timeout_nucleus) = nid else { continue; };
                    let Some(cage) = nuclei.get_mut(&timeout_nucleus) else {continue;};
                    let event_id = cage.event_id;
                    let req_content = RequestContent::QueryEvents((timeout_nucleus.clone(), event_id).encode());
                    let controllers = get_nucleus_controllers(client.clone(), timeout_nucleus, client.info().best_hash);
                    for c in controllers {
                         if c == self_controller {
                            continue;
                         }
                         let authority = sp_authority_discovery::AuthorityId::from_slice(c.as_slice()).unwrap();
                         let msg = SendMessage {
                            dest: Destination::AuthorityId(authority),
                            request: req_content.clone(),
                        };
                        let (resp_tx, resp_rx) = oneshot::channel::<Vec<u8>>();
                        let Ok(_) =  cage_p2p_tx.send((msg, resp_tx)).await else { continue;};
                        let Ok(resp_events) = resp_rx.await else {continue;};
                        let events = Decode::decode(&mut &resp_events[..]).unwrap_or_default();
                        cage.execute_outer_events(events);
                    }
                }
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
                            let tx = gen_vrf_tx(client.clone(), NucleusId::from(nucleus_id.0), keystore.clone(), public_input.as_ref(), hash, metadata.clone())
                                .inspect_err(|e| log::error!("fail to submit vrf: {:?}", e))
                                .expect("fail to submit vrf");
                            transaction_pool.submit_one(hash, TransactionSource::External, tx).await.expect("fail to submit tx");
                        } else if let Ok(Some(ev)) = event.as_ref().map(|ev| ev.as_event::<codegen::nucleus::events::NucleusUpgraded>().ok().flatten()) {
                            let nucleus_id = ev.id;
                            let digest = ev.wasm_hash;
                            let version = ev.wasm_version;
                            if nuclei.contains_key(&NucleusId::from(nucleus_id.0)) {
                                // TODO download the wasm from ev.wasm_location
                                let nucleus_path = nucleus_home_dir.join(nucleus_id.to_string());
                                if let Err(e) = upgrade_nucleus_wasm(
                                    NucleusId::from(nucleus_id.0),
                                    digest,
                                    version,
                                    nucleus_path.as_path(),
                                    &mut nuclei,
                                ) {
                                    log::error!("Upgrading nucleus {} wasm failed: {:?}", nucleus_id, e);
                                    // panic!("fail to upgrade nucleus wasm, there is nothing we can do.");
                                }
                            }
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
                                nucleus_path,
                                http_register.clone(),
                                timer_scheduler.clone(),
                                tss_node.clone(),
                                &mut nuclei,
                                token_timeout_tx.clone(),
                            ).expect("fail to start nucleus");
                        }
                    }
                },
                Some(entry) = timer_scheduler.pop() => {
                    log::info!("⏲️ Timer triggered: {:?}", entry);
                    if let Some(nucleus) = nuclei.get_mut(&entry.nucleus_id) {
                        nucleus.forward(Gluon::TimerTrigger {
                            task_id: 0,
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
                Some((module, gluon)) = nucleus_signal.recv() => {
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
                    nuclei.values_mut().for_each(|nucleus| {
                        nucleus.drain(vec![]);
                    });
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
    C::Api: NucleusApi<B> + 'static,
    T: vrs_metadata::Config,
{
    let key = vrs_metadata::codegen::storage().system().events();
    let storage_key = sp_core::storage::StorageKey(key.to_root_bytes());
    client
        .storage(hash, &storage_key)
        .inspect_err(|e| log::error!("failed to get events: {:?}", e))
        .map(|b| b.map(|v| events::decode_from::<T>(v.0, metadata)))
        .map_err(|e| e.into())
}

fn gen_vrf_tx<B, D, C>(
    client: Arc<C>,
    nucleus_id: NucleusId,
    keystore: KeystorePtr,
    public_input: &[u8],
    hash: B::Hash,
    metadata: RuntimeMetadata,
) -> Result<sp_runtime::OpaqueExtrinsic, Box<dyn std::error::Error>>
where
    B: sp_runtime::traits::Block<Extrinsic = sp_runtime::OpaqueExtrinsic> + 'static,
    D: Backend<B>,
    C: BlockBackend<B> + StorageProvider<B, D> + ProvideRuntimeApi<B> + 'static,
    C::Api: Core<B> + 'static,
    C::Api: AccountNonceApi<B, AccountId, u32> + 'static,
    C::Api: NucleusApi<B> + 'static,
{
    let (submitter, vrf) = crate::keystore::sign_to_participate(
        keystore.clone(),
        keys::NUCLEUS_VRF_KEY_TYPE,
        public_input,
    )?;
    let submitter: AccountId = submitter.into();
    // TODO for unknown reasons, the key is none so we just hard-code the key here
    let key = hex::decode(
        "26aa394eea5630e07c48ae0c9558cef7a44704b568d21667356a5a050c118746b4def25cfda6ef3a00000000",
    )
    .unwrap();
    let storage_key = sp_core::storage::StorageKey(key);
    let genesis_hash = client
        .storage(hash, &storage_key)
        .ok()
        .flatten()
        .expect("couldn't get genesis hash");
    let genesis_hash = sp_core::H256::from_slice(&genesis_hash.0);
    let api = client.runtime_api();
    let nonce = api.account_nonce(hash, submitter.clone())?;
    let version = api.version(hash)?;
    let state = vrs_metadata::tx::ClientState::<SubstrateConfig> {
        metadata,
        genesis_hash,
        runtime_version: vrs_metadata::tx::RuntimeVersion {
            spec_version: version.spec_version,
            transaction_version: version.transaction_version,
        },
    };
    let call = vrs_metadata::codegen::tx().nucleus().register(
        vrs_metadata::utils::AccountId32(nucleus_id.into()),
        vrs_metadata::codegen::runtime_types::sp_core::sr25519::vrf::VrfSignature {
            pre_output: vrf
                .pre_output
                .encode()
                .try_into()
                .expect("invalid vrf output"),
            proof: vrf.proof.encode().try_into().expect("invalid vrf proof"),
        },
    );
    let params = vrs_metadata::config::DefaultExtrinsicParamsBuilder::new()
        .nonce(nonce as u64)
        .build();
    let unsigned_tx = vrs_metadata::tx::create_partial_signed(&call, &state, params)
        .expect("couldn't create paritial transaction");
    let raw = unsigned_tx.signer_payload();
    let signature = crate::keystore::sign_tx(keystore.clone(), keys::NUCLEUS_VRF_KEY_TYPE, raw)?;
    let addr =
        vrs_metadata::utils::MultiAddress::Id(vrs_metadata::utils::AccountId32(submitter.into()));
    let signature = vrs_metadata::utils::MultiSignature::Sr25519(signature);
    let signed = unsigned_tx.sign_with_address_and_signature(&addr, &signature);
    let tx = sp_runtime::OpaqueExtrinsic::from_bytes(signed.encoded())?;
    Ok(tx)
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
    C::Api: NucleusApi<B> + 'static,
{
    let api = client.runtime_api();
    let key = codegen::storage().nucleus().instances_iter();
    let instance_key = sp_core::storage::StorageKey(key.to_root_bytes());
    let kv = match client
        .storage_pairs(hash, Option::from(&instance_key), None)
        .ok()
    {
        None => {
            vec![]
        }
        Some(pairs) => pairs
            .map(|(k, v)| {
                let mut nucleus_id_vec = [0u8; 32];
                let len = k.0.len();
                nucleus_id_vec.copy_from_slice(&k.0[len - 32..]);
                let nucleus_id = NucleusId::from(nucleus_id_vec);
                let controllers = <Vec<AccountId> as Decode>::decode(&mut &v.0[..])
                    .ok()
                    .unwrap();
                (nucleus_id, controllers)
            })
            .collect::<Vec<(NucleusId, Vec<AccountId>)>>(),
    };
    let list = kv
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

fn get_nucleus_controllers<B, D, C>(
    client: Arc<C>,
    nucleus_id: NucleusId,
    hash: B::Hash,
) -> Vec<AccountId>
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
    let x: &[u8; 32] = nucleus_id.as_ref();
    let n = vrs_metadata::utils::AccountId32::from(*x);
    let key = codegen::storage().nucleus().instances(n);
    let instance_key = sp_core::storage::StorageKey(key.to_root_bytes());
    match client.storage(hash, &instance_key).ok() {
        None => {
            vec![]
        }
        Some(controllers_opt) => match controllers_opt {
            None => {
                vec![]
            }
            Some(sto) => <Vec<AccountId> as Decode>::decode(&mut &sto.0[..]).unwrap_or_default(),
        },
    }
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
    nucleus_path: &std::path::Path,
    nuclei: &mut HashMap<NucleusId, NucleusCage>,
) -> anyhow::Result<()> {
    let wasm = try_load_wasm(id.clone(), nucleus_path, version)?;
    log::info!("🔨 Upgrading nucleus {} to version {}.", id, version,);
    if let Some(mut cage) = nuclei.get_mut(&id) {
        cage.forward(Gluon::CodeUpgrade { wasm });
    }
    Ok(())
}

fn try_load_wasm(
    id: NucleusId,
    nucleus_path: &std::path::Path,
    version: u32,
) -> anyhow::Result<WasmInfo> {
    let wasm_path = nucleus_path.join(format!("wasm/{}.wasm", version));
    // log::info!("wasm_path: {:?}", wasm_path);
    let code = std::fs::read(&wasm_path)?;
    log::info!("📦 Loaded wasm for nucleus {} version {}.", id, version);
    Ok(WasmInfo { id, version, code })
}

fn init_nucleus_home<P: AsRef<std::path::Path>>(dir: P) {
    if !std::fs::exists(dir.as_ref()).expect(
        "fail to check nucleus directory, make sure the you have access right on the directory.",
    ) {
        std::fs::create_dir_all(dir.as_ref())
            .inspect_err(|e| log::error!("fail to create nucleus home, due to {:?}", e))
            .expect("fail to create nucleus directory");
    }
}

// TODO lookup wasm before starting nucleus
// the `CodeUpgrade` might happen before `InstanceRegistered` event
fn start_nucleus(
    id: NucleusId,
    nucleus_info: NucleusInfo<AccountId, Hash, NodeId>,
    nucleus_path: std::path::PathBuf,
    http_register: Arc<HttpCallRegister>,
    timer_scheduler: Arc<SchedulerAsync>,
    tss_node: Arc<vrs_tss::NodeRuntime>,
    nuclei: &mut HashMap<NucleusId, NucleusCage>,
    token_timeout_tx: tokio::sync::mpsc::Sender<NucleusId>,
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
    let config: RuntimeParams = RuntimeParams {
        nucleus_id: id.clone(),
        nucleus_home: nucleus_path.clone().into_boxed_path(),
        http_register,
        timer_scheduler,
        tss_node,
    };
    let runtime = Runtime::init(config)?;
    let state = runtime.state();
    let tunnel = Nucleus::start(runtime, token_timeout_tx);
    log::info!("🚀 Nucleus {} started.", id);
    nuclei.insert(
        id.clone(),
        NucleusCage {
            nucleus_id: id.clone(),
            tunnel,
            pending_requests: vec![],
            event_id: current_event,
            state,
        },
    );
    // TODO if start before wasm uploaded, we should skip this, but we must ensure the wasm is downloaded to local
    if let Ok(wasm) = try_load_wasm(id.clone(), nucleus_path.as_ref(), wasm_version) {
        let cage = nuclei
            .get_mut(&id)
            .expect("just inserted nucleus, should be found;qed");
        cage.forward(Gluon::CodeUpgrade { wasm });
    }
    Ok(())
}

pub fn create_outgoing(bs: Vec<u8>) -> OutgoingResponse {
    OutgoingResponse {
        result: Ok(bs),
        reputation_changes: vec![],
        sent_feedback: None,
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::nucleus::Nucleus;
    // use crate::test_suite::new_mock_nucleus;
    // use std::sync::Arc;
    // use std::thread;
    // use stream::FuturesUnordered;
    // use tokio::task;

    // struct ResultProcessor {
    //     receiver: tokio::sync::mpsc::Receiver<NucleusResponse>,
    // }
    // impl ResultProcessor {
    //     fn new() -> tokio::sync::oneshot::Sender<NucleusResponse> {
    //         let (sender, receiver) = tokio::sync::oneshot::channel();
    //         thread::spawn(move || async move {
    //             let reply_to = receiver.await;
    //             println!("reply: {:?}", reply_to);
    //         });
    //         sender
    //     }
    // }
    // fn decode<T: Decode>(data: Vec<u8>) -> T {
    //     let mut reply = data.as_slice();
    //     println!("reply: {:?}", reply);
    //     let reply = <Result<T, String> as codec::Decode>::decode(&mut reply)
    //         .unwrap()
    //         .unwrap();
    //     reply
    // }
    // #[tokio::test]
    // async fn test_scheduler_async() {
    //     let wasm_path = "../../nucleus-examples/timer.wasm";
    //     let (out_of_runtime, sender_cage) = new_mock_nucleus(wasm_path.to_string());
    //     let sender1 = sender_cage.clone();
    //     let (tree_tx, mut tree_rx) = tokio::sync::mpsc::channel(100);
    //     tokio::spawn(async move {
    //         while let Some(entry) = out_of_runtime.scheduler.pop().await {
    //             println!("entry: {:?}", entry);
    //             if entry.func_name == "test_set_perfect_tree_mod_timer" {
    //                 let d =
    //                     <(i32, i32) as Decode>::decode(&mut entry.func_params.as_slice()).unwrap();
    //                 println!("d: {:?}", d);
    //                 tree_tx.send(d).await.unwrap();
    //             }
    //             sender1
    //                 .send((
    //                     0,
    //                     Gluon::TimerRequest {
    //                         endpoint: entry.func_name,
    //                         payload: entry.func_params,
    //                     },
    //                 ))
    //                 .unwrap();
    //         }
    //     });
    //     let (sender_reply, receiver_reply) = tokio::sync::oneshot::channel();
    //     sender_cage
    //         .send((
    //             0,
    //             Gluon::PostRequest {
    //                 endpoint: "test_set_timer".to_owned(),
    //                 payload: <(i32, i32) as codec::Encode>::encode(&(1, 0)),
    //                 // reply_to: Some(sender_reply),
    //                 reply_to: Some(sender_reply),
    //             },
    //         ))
    //         .unwrap();
    //     let (sender_reply, receiver_reply) = tokio::sync::oneshot::channel();
    //     sender_cage
    //         .send((
    //             0,
    //             Gluon::GetRequest {
    //                 endpoint: "test_get_timer".to_owned(),
    //                 payload: vec![],
    //                 reply_to: Some(sender_reply),
    //             },
    //         ))
    //         .unwrap();
    //     let reply = receiver_reply.await.unwrap().unwrap();
    //     let mut reply = reply.as_slice();
    //     let reply = <Result<String, String> as codec::Decode>::decode(&mut reply)
    //         .unwrap()
    //         .unwrap();
    //     assert_eq!(reply, "init");
    //     tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    //     let (sender_reply, receiver_reply) = tokio::sync::oneshot::channel();
    //     sender_cage
    //         .send((
    //             0,
    //             Gluon::GetRequest {
    //                 endpoint: "test_get_timer".to_owned(),
    //                 payload: vec![],
    //                 reply_to: Some(sender_reply),
    //             },
    //         ))
    //         .unwrap();
    //     let reply = receiver_reply.await.unwrap().unwrap();
    //     assert_eq!(decode::<String>(reply), "delay_complete abc 123");
    //     let mut last_d = (0, 0);
    //     tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    //     while let Some(d) = tree_rx.recv().await {
    //         assert!(d.1 >= last_d.1);
    //         last_d = d;
    //         if d.0 == 15 {
    //             break;
    //         }
    //     }
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
    // }
}
