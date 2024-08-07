use crate::{nucleus::Nucleus, Context, Gluon, NucleusResponse, WasmCodeRef, WasmInfo};
use codec::{Decode, Encode};
use futures::prelude::*;
use sc_client_api::{
    notifications::StorageNotification, Backend, BlockBackend, BlockchainEvents,
    StorageNotifications, StorageProvider,
};
use sp_api::{Metadata, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use std::collections::HashMap;
use std::sync::mpsc::Sender as SyncSender;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver};
use vrs_metadata::{
    codegen, config::SubstrateConfig, events, metadata, storage, Metadata as RuntimeMetadata,
    METADATA_BYTES,
};
use vrs_primitives::{AccountId, Hash, NucleusEquation, NucleusId};

pub struct CageParameters<B, C, BN> {
    pub rx: Receiver<(NucleusId, Gluon)>,
    pub client: Arc<C>,
    pub controller: AccountId,
    pub _phantom: std::marker::PhantomData<(B, BN)>,
}

struct NucleusCage {
    tunnel: SyncSender<(u64, Gluon)>,
    pending_requests: Vec<Gluon>,
    event_id: u64,
    // TODO monadring-related
    // TODO kvdb
}

impl NucleusCage {
    fn drain(&mut self) {
        for req in self.pending_requests.drain(..) {
            self.event_id += 1;
            // TODO
            let _ = self.tunnel.send((self.event_id, req));
        }
    }

    fn forward(&mut self, gluon: Gluon) {
        match gluon {
            Gluon::PostRequest { .. } => {
                self.pending_requests.push(gluon);
            }
            Gluon::GetRequest { .. } => {
                // TODO we consumed reply_to of gluon, so we can do nothing even the sending failed
                let _ = self.tunnel.send((0, gluon));
            }
            // TODO
            _ => unreachable!(),
        }
    }
}

pub fn start_nucleus_cage<B, C, BN>(params: CageParameters<B, C, BN>) -> impl Future<Output = ()>
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
    let CageParameters {
        mut rx,
        client,
        controller,
        _phantom,
    } = params;
    async move {
        let mut nuclei: HashMap<NucleusId, NucleusCage> = HashMap::new();
        // TODO what if our node is far behind the best block?
        let current_block = client.info().best_hash;
        let metadata =
            metadata::decode_from(&METADATA_BYTES[..]).expect("failed to decode metadata.");
        let mut registry_monitor = client.every_import_notification_stream();
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
                block = registry_monitor.next() => {
                    let hash = block.expect("block importing error").hash;
                    // TODO handle deregister as well
                    if let Some(instances) = map_to_nucleus(client.clone(), hash, metadata.clone()) {
                        for (id, config) in instances {
                            start_nucleus::<B>(id, config, &mut nuclei).expect("fail to start nucleus");
                        }
                    }
                },
                req = rx.recv() => {
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
                    // TODO only drain the nucleus that token
                    // nuclei.get_mut(&token).map(|nucleus| nucleus.drain());
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
    runtime_storage: Arc<C>,
    hash: B::Hash,
    metadata: RuntimeMetadata,
) -> Option<Vec<(NucleusId, NucleusEquation<AccountId, B::Hash>)>>
where
    B: sp_runtime::traits::Block,
    D: Backend<B>,
    C: BlockBackend<B> + StorageProvider<B, D> + BlockchainEvents<B> + 'static,
{
    let storage_key = storage_key(b"System", b"Events");
    let events = runtime_storage
        .storage(hash, &storage_key)
        .ok()
        .flatten()
        .map(|v| events::decode_from::<SubstrateConfig>(v.0, metadata))?;
    let instances = events
        .find::<codegen::nucleus::events::InstanceRegistered>()
        .filter_map(|ev| {
            let id = ev.ok()?.nucleus_id.clone();
            let storage_key = blake2_128concat_storage_key(b"Nucleus", b"Nuclei", id.clone());
            runtime_storage
                .storage(hash, &storage_key)
                .ok()
                .flatten()
                .map(|v| {
                    let mut buf = v.0.as_ref();
                    let decoded = <NucleusEquation<AccountId, B::Hash>>::decode(&mut buf).ok()?;
                    Some((NucleusId::new(id.0), decoded))
                })
                .flatten()
        })
        .collect::<Vec<_>>();
    Some(instances)
}

fn blake2_128concat_storage_key<K: codec::Encode>(
    module: &[u8],
    storage: &[u8],
    key: K,
) -> sp_core::storage::StorageKey {
    let mut bytes = sp_core::twox_128(module).to_vec();
    bytes.extend(&sp_core::twox_128(storage)[..]);
    let encoded = key.encode();
    let x: &[u8] = encoded.as_slice();
    let v = sp_core::blake2_128(x)
        .iter()
        .chain(x.iter())
        .cloned()
        .collect::<Vec<_>>();
    bytes.extend(v);
    sp_core::storage::StorageKey(bytes)
}

fn storage_key(module: &[u8], storage: &[u8]) -> sp_core::storage::StorageKey {
    let mut bytes = sp_core::twox_128(module).to_vec();
    bytes.extend(&sp_core::twox_128(storage)[..]);
    sp_core::storage::StorageKey(bytes)
}

// TODO
fn start_nucleus<B>(
    id: NucleusId,
    config: NucleusEquation<AccountId, B::Hash>,
    nuclei: &mut HashMap<NucleusId, NucleusCage>,
) -> anyhow::Result<()>
where
    B: sp_runtime::traits::Block,
{
    let NucleusEquation {
        name,
        account,
        wasm_url,
        wasm_hash,
        wasm_version,
        energy,
        current_event,
        root_state,
        capacity,
    } = config;
    let name = String::from_utf8(name)?;
    let url = String::from_utf8(wasm_url)?;
    let wasm = WasmInfo {
        account,
        name,
        version: 0,
        code: WasmCodeRef::File(url),
    };
    // TODO
    let context = Context::init().unwrap();
    let (tx, rx) = std::sync::mpsc::channel();
    let mut nucleus = Nucleus::new(rx, context, wasm);
    std::thread::spawn(move || {
        nucleus.run();
    });
    nuclei.insert(
        id,
        NucleusCage {
            tunnel: tx,
            pending_requests: vec![],
            event_id: 0,
        },
    );
    Ok(())
}
