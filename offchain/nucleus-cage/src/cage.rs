use crate::{nucleus::Nucleus, Gluon, NucleusResponse};
use futures::{prelude::*, select};
use sc_client_api::{Backend, BlockBackend, BlockchainEvents, StorageProvider};
use std::collections::HashMap;
use std::sync::mpsc::Sender as SyncSender;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver, Sender};
use vrs_primitives::{AccountId, NucleusEquation, NucleusId};

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
        // TODO init nucleus already registered
        // TODO mock monadring
        //////////////////////////////////////////////////////
        let (token_tx, mut token_rx) = mpsc::unbounded_channel::<String>();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                let _ = token_tx.send("".to_string());
            }
        });
        //////////////////////////////////////////////////////

        loop {
            tokio::select! {
                nucleus = monitor_nucleus_registry(client.clone(), controller.clone()) => {
                    // start_nucleus(nucleus);
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

async fn monitor_nucleus_registry<B, BN, C>(runtime_storage: Arc<C>, controller: AccountId)
where
    B: sp_runtime::traits::Block,
    BN: Backend<B>,
    C: BlockBackend<B> + StorageProvider<B, BN> + BlockchainEvents<B> + 'static,
{
    let storage_key = blake2_128concat_storage_key(b"nucleus", b"RegisteredCages", controller);
    // TODO
    let new_val = runtime_storage
        .storage_changes_notification_stream(Some(&[storage_key]), None)
        .expect("fail to get storage changes notification stream")
        // .map(|c| c.changes.iter().collect::<Vec<_>>())
        .next()
        .await;
    println!("{:?}", new_val);
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

// fn start_nucleus(nucleus: NucleusEquation) {}
