mod cage;
mod keystore;

use crate::cage::NucleusCage;
use codec::{Decode, Encode};
use frame_system_rpc_runtime_api::AccountNonceApi;
use futures::channel::oneshot;
use futures::prelude::*;
use sc_authority_discovery::Service as AuthorityDiscovery;
use sc_client_api::{Backend, BlockBackend, BlockchainEvents, StorageProvider};
use sc_network::{
    request_responses::{IncomingRequest, OutgoingResponse},
    service::traits::NetworkService,
    PeerId,
};
use sc_transaction_pool_api::{TransactionPool, TransactionSource};
use sp_api::{Core, Metadata, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_keystore::KeystorePtr;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::{self, Receiver, Sender};
use vrs_gluon_relayer::ForwardRequest;
use vrs_metadata::{
    codegen, config::SubstrateConfig, events, metadata, Metadata as RuntimeMetadata,
    METADATA_BYTES as METADATA,
};
use vrs_nucleus_executor::{
    host_func::{self, HttpCallRegister, HttpResponseWithCallback, SchedulerAsync},
    Gluon, Nucleus, NucleusError, NucleusResponse, Runtime, RuntimeParams, WasmInfo,
};
use vrs_nucleus_network::{PayloadWithSignature, SendMessage};
use vrs_nucleus_runtime_api::NucleusRuntimeApi;
use vrs_primitives::{keys, AccountId, Hash, NodeId, NucleusId, NucleusInfo};
use vrs_validator_runtime_api::ValidatorRuntimeApi;

pub type NucleusRpcChannel = Sender<(NucleusId, Gluon)>;
pub type NucleusSignal = Receiver<(NucleusId, Gluon)>;

pub struct CageParams<P, B, C, BN> {
    pub client: Arc<C>,
    pub authority_id: AccountId,
    pub keystore: KeystorePtr,
    pub transaction_pool: Arc<P>,
    pub authority_discovery: AuthorityDiscovery,
    pub nucleus_signal: NucleusSignal,
    pub net_service: Arc<dyn NetworkService>,
    pub tss_node: Arc<vrs_tss::NodeRuntime>,
    pub nucleus_home_dir: std::path::PathBuf,
    pub gluon_relay_rx: async_channel::Receiver<IncomingRequest>,
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
    C::Api: ValidatorRuntimeApi<B> + 'static,
    C::Api: NucleusRuntimeApi<B> + 'static,
    C::Api: AccountNonceApi<B, AccountId, u32> + 'static,
{
    let CageParams {
        client,
        authority_id,
        keystore,
        transaction_pool,
        authority_discovery,
        nucleus_signal,
        net_service,
        tss_node,
        nucleus_home_dir,
        gluon_relay_rx,
        p2p_cage_rx,
        cage_p2p_tx,
        _phantom,
    } = params;
    async move {
        let mut nuclei: HashMap<NucleusId, NucleusCage> = HashMap::new();
        let mut nucleus_signal = nucleus_signal;
        let nucleus_home_dir = nucleus_home_dir.into_boxed_path();
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
        let hash = client.info().best_hash;
        let chosen = get_nuclei_for_node(client.clone(), authority_id.clone(), hash);
        let (token_timeout_tx, token_timeout_rx) = tokio::sync::mpsc::channel(1000);
        for (id, info) in chosen {
            let nucleus_path = nucleus_home_dir.join(id.to_string());
            if let Err(e) = start_nucleus(
                id.clone(),
                info,
                nucleus_path,
                http_register.clone(),
                timer_scheduler.clone(),
                tss_node.clone(),
                &mut nuclei,
                token_timeout_tx.clone(),
            ) {
                log::error!("Failed to start nucleus {}: {:?}", id, e);
            }
        }
        loop {
            tokio::select! {
                Ok(req) = gluon_relay_rx.recv() => {
                    let (tx, rx) = tokio::sync::oneshot::channel::<NucleusResponse>();
                    tokio::spawn(async move {
                        // rsp is encoded as NucleusResponse
                        let Ok(rsp) = rx.await else { return };
                        let _ = req.pending_response.send(OutgoingResponse {
                            result: Ok(rsp.encode()),
                            reputation_changes: vec![],
                            sent_feedback: None,
                        });
                    });
                    match ForwardRequest::decode(&mut &req.payload[..]) {
                        Ok(fwd) => handle_relayer_message(&mut nuclei, nucleus_home_dir.clone(), fwd, tx).await,
                        Err(_) => {
                            let _ = tx.send(Err(NucleusError::params("unable to decode ForwardReq")));
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
                            let tx = gen_vrf_tx(client.clone(), NucleusId::from(nucleus_id.0), keystore.clone(), public_input.as_ref(), hash, metadata.clone())
                                .inspect_err(|e| log::error!("fail to submit vrf: {:?}", e))
                                .expect("fail to submit vrf");
                            if let Err(e) = transaction_pool.submit_one(hash, TransactionSource::External, tx).await {
                                log::error!("fail to submit vrf transaction: {:?}", e);
                            }
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
                                }
                            }
                        } else if let Ok(Some(ev)) = event.as_ref().map(|ev| ev.as_event::<codegen::nucleus::events::InstanceRegistered>().ok().flatten()) {
                            let nucleus_id = ev.id;
                            let controller = ev.controller;
                            if AccountId::from(controller.0) == authority_id {
                                let api = client.runtime_api();
                                let info = api
                                    .get_nucleus_info(hash, &NucleusId::from(nucleus_id.0))
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
                    match nuclei.get_mut(&module) {
                        Some(nucleus) => nucleus.forward(gluon),
                        None => reply_directly(gluon, Err(NucleusError::nucleus_not_found())),
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
        Gluon::PostRequest { reply_to, .. }
        | Gluon::GetRequest { reply_to, .. }
        | Gluon::AbiRequest { reply_to } => {
            reply_to.and_then(|tx| tx.send(msg).ok());
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
    C::Api: NucleusRuntimeApi<B> + 'static,
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
    C::Api: NucleusRuntimeApi<B> + 'static,
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
    C::Api: NucleusRuntimeApi<B> + 'static,
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
            api.get_nucleus_info(hash, &nucleus_id)
                .ok()?
                .map(|info| (nucleus_id, info))
        })
        .collect::<Vec<_>>();
    list
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
    if let Some(cage) = nuclei.get_mut(&id) {
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
    let code = std::fs::read(&wasm_path)?;
    log::info!("📦 Loaded wasm for nucleus {} version {}.", id, version);
    Ok(WasmInfo { id, version, code })
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
        validators,
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

async fn handle_relayer_message(
    nuclei: &mut HashMap<NucleusId, NucleusCage>,
    nucleus_home_dir: Box<std::path::Path>,
    fwd: ForwardRequest,
    tx: tokio::sync::oneshot::Sender<NucleusResponse>,
) {
    match fwd {
        ForwardRequest::Get {
            nucleus_id,
            endpoint,
            payload,
        } => {
            let gluon = Gluon::GetRequest {
                endpoint,
                payload,
                reply_to: Some(tx),
            };
            match nuclei.get_mut(&nucleus_id) {
                Some(nucleus) => nucleus.forward(gluon),
                None => reply_directly(gluon, Err(NucleusError::nucleus_not_found())),
            }
        }
        ForwardRequest::Post {
            nucleus_id,
            endpoint,
            payload,
        } => {
            let gluon = Gluon::PostRequest {
                endpoint,
                payload,
                reply_to: Some(tx),
            };
            match nuclei.get_mut(&nucleus_id) {
                Some(nucleus) => nucleus.forward(gluon),
                None => reply_directly(gluon, Err(NucleusError::nucleus_not_found())),
            }
        }
        ForwardRequest::Abi { nucleus_id } => {
            let gluon = Gluon::AbiRequest { reply_to: Some(tx) };
            match nuclei.get_mut(&nucleus_id) {
                Some(nucleus) => nucleus.forward(gluon),
                None => reply_directly(gluon, Err(NucleusError::nucleus_not_found())),
            }
        }
        ForwardRequest::Install {
            nucleus_id,
            version,
            payload,
        } => {
            let path = nucleus_home_dir
                .join(nucleus_id.to_string())
                .join(format!("wasm/{}.wasm", version));
            match File::create(&path).await {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(&payload).await {
                        log::error!("Failed to write wasm file: {:?}", e);
                        let _ = tx.send(Err(NucleusError::node(
                            "The validator rejected the wasm file.",
                        )));
                        return;
                    }
                    log::info!("Wasm file {}:{} written to {:?}", nucleus_id, version, path);
                    let _ = tx.send(Ok(vec![]));
                }
                Err(e) => {
                    log::error!("Failed to create wasm file: {:?}", e);
                    let _ = tx.send(Err(NucleusError::node(
                        "The validator rejected the wasm file.",
                    )));
                }
            }
        }
    }
}
