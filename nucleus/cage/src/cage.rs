use crate::{nucleus::Nucleus, Context, Gluon, NucleusResponse, WasmCodeRef, WasmInfo};
use codec::Decode;
use futures::prelude::*;
use sc_client_api::{Backend, BlockBackend, BlockchainEvents, StorageProvider};
use std::collections::HashMap;
use std::sync::mpsc::Sender as SyncSender;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver};
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
    C: BlockBackend<B> + StorageProvider<B, BN> + BlockchainEvents<B> + 'static,
{
    let CageParameters {
        mut rx,
        client,
        controller,
        _phantom,
    } = params;
    async move {
        let mut nuclei: HashMap<NucleusId, NucleusCage> = HashMap::new();
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
                nucleus = nucleus_instance_updates(client.clone(), controller.clone()) => {
                    println!("{:?}", nucleus);
                    // TODO diff to start & stop
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

// TODO move this to pallet-nucleus
async fn nucleus_instance_updates<B, D, C>(
    runtime_storage: Arc<C>,
    controller: AccountId,
) -> Option<Vec<(NucleusId, NucleusEquation<AccountId, Hash>)>>
where
    B: sp_runtime::traits::Block,
    D: Backend<B>,
    C: BlockBackend<B> + StorageProvider<B, D> + BlockchainEvents<B> + 'static,
{
    let storage_key = blake2_128concat_storage_key(b"Nucleus", b"Instances", controller);
    let updates = runtime_storage
        .storage_changes_notification_stream(Some(&[storage_key]), None)
        .ok()?
        .next()
        .await?;
    let storage = updates
        .changes
        .iter()
        .map(|(_, _, v)| v)
        .take(1)
        .next()?
        .clone();

    let instances = storage.and_then(|v| {
        let mut buf = v.0.as_ref();
        let decoded = <Vec<NucleusId>>::decode(&mut buf).ok()?;
        Some(decoded)
    })?;
    log::info!(
        "💡Detecting instances update at block {}: {:?}",
        updates.block,
        instances,
    );
    let instances = instances
        .into_iter()
        .filter_map(|id| {
            let storage_key = blake2_128concat_storage_key(b"Nucleus", b"Nuclei", id.clone());
            runtime_storage
                .storage(updates.block, &storage_key)
                .ok()
                .flatten()
                .map(|v| {
                    let mut buf = v.0.as_ref();
                    let decoded = <NucleusEquation<AccountId, Hash>>::decode(&mut buf).ok()?;
                    Some(decoded)
                })
                .flatten()
                .map(|v| (id, v))
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

// TODO
fn start_nucleus(
    id: NucleusId,
    config: NucleusEquation<AccountId, Hash>,
) -> anyhow::Result<SyncSender<(u64, Gluon)>> {
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
    let nucleus = Nucleus::new(rx, context, wasm);
    Ok(tx)
}
