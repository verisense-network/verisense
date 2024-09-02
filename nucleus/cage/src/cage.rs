use crate::{
    nucleus::Nucleus, Context, ContextConfig, Gluon, NucleusResponse, ReplyTo, WasmCodeRef,
    WasmInfo,
};
use codec::{Decode, Encode};
use futures::prelude::*;
use rocksdb::{ColumnFamilyDescriptor, Options, DB};
use sc_client_api::{Backend, BlockBackend, BlockchainEvents, StorageProvider};
use sp_api::{Metadata, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use std::collections::HashMap;
use std::sync::mpsc::Sender as SyncSender;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver};
use vrs_metadata::{
    codegen, codegen::runtime_types::pallet_nucleus::pallet::NucleusEquation,
    config::SubstrateConfig, events, metadata, storage, Metadata as RuntimeMetadata,
    METADATA_BYTES,
};
use vrs_primitives::{AccountId, Hash, NucleusId};

pub struct CageParams<B, C, BN> {
    pub nucleus_rpc_rx: Receiver<(NucleusId, Gluon)>,
    pub client: Arc<C>,
    pub controller: AccountId,
    pub nucleus_home_dir: std::path::PathBuf,
    pub _phantom: std::marker::PhantomData<(B, BN)>,
}

struct NucleusCage {
    nucleus_id: NucleusId,
    tunnel: SyncSender<(u64, Gluon)>,
    pending_requests: Vec<(String, Vec<u8>, Option<ReplyTo>)>,
    event_id: u64,
    db: Arc<DB>,
    // TODO monadring-related
    // TODO kvdb
}

impl NucleusCage {
    fn validate_token(&self) -> bool {
        true
    }

    fn pre_commit(&self, id: u64, msg: &[u8]) -> anyhow::Result<()> {
        let handle = self.db.cf_handle("seq").unwrap();
        self.db.put_cf(handle, &id.to_be_bytes(), msg)?;
        Ok(())
    }

    fn drain(&mut self, imports: Vec<(String, Vec<u8>)>) {
        for import in imports.into_iter() {
            self.pending_requests.push((import.0, import.1, None));
        }
        let pipe = self.pending_requests.drain(..).collect::<Vec<_>>();
        for gluon in pipe.into_iter() {
            self.event_id += 1;
            let event = Self::merge_payload(&gluon.0, &gluon.1);
            if let Err(e) = self.pre_commit(self.event_id, &event) {
                log::error!(
                    "couldn't save event {} of nucleus {}: {:?}",
                    self.event_id,
                    self.nucleus_id,
                    e
                );
                if let Some(reply_to) = gluon.2 {
                    let _ = reply_to.send(Err((-42000, "Event persistence failed.".to_string())));
                }
            } else {
                let _ = self.tunnel.send((
                    self.event_id,
                    Gluon::PostRequest {
                        endpoint: gluon.0,
                        payload: gluon.1,
                        reply_to: gluon.2,
                    },
                ));
            }
        }
    }

    fn merge_payload(endpoint: &String, payload: &[u8]) -> Vec<u8> {
        let mut buf = <String>::encode(endpoint);
        buf.extend(payload);
        buf
    }

    fn forward(&mut self, gluon: Gluon) {
        match gluon {
            Gluon::PostRequest {
                endpoint,
                payload,
                reply_to,
            } => {
                self.pending_requests.push((endpoint, payload, reply_to));
            }
            Gluon::GetRequest { .. } => {
                let _ = self.tunnel.send((0, gluon));
            }
            // TODO
            _ => unreachable!(),
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
    C::Api: Metadata<B>,
{
    let CageParams {
        mut nucleus_rpc_rx,
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
                // block = registry_monitor.next() => {
                //     let hash = block.expect("block importing error").hash;
                //     // TODO handle deregister as well
                //     if let Some(instances) = map_to_nucleus(client.clone(), hash, metadata.clone()) {
                //         for (id, config) in instances {
                //             let db_path = nucleus_home_dir.join(id.to_string());
                //             start_nucleus::<B>(id, config, db_path, &mut nuclei).expect("fail to start nucleus");
                //         }
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
                    // TODO only drain the nucleus that token
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SchedulerAsync;
    use crate::{nucleus::Nucleus, scheduler, vm::Vm, Scheduler, WrappedSchedulerSync};
    use codec::Decode;
    use futures::channel::mpsc::SendError;
    use rocksdb::Options;
    use sp_core::hexdisplay::AsBytesRef;
    use std::thread;
    use std::{sync::Arc, time::Duration};
    use temp_dir::TempDir;
    use tokio::sync::{oneshot, RwLock};
    use tokio::{sync::mpsc, task, time};
    use vrs_core_sdk::AccountId;
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
        let wasm_path = "../../nucleus-examples/vrs_nucleus_examples.wasm";
        let wasm = WasmInfo {
            account: AccountId::new([0u8; 32]),
            name: "avs-dev-demo".to_string(),
            version: 0,
            code: WasmCodeRef::File(wasm_path.to_string()),
        };

        let tmp_dir = TempDir::new().unwrap();
        let context = Context::init(ContextConfig {
            db_path: tmp_dir.child("0").into_boxed_path(),
        })
        .unwrap();
        let (sender_cage, receiver_cage) = std::sync::mpsc::channel();
        let scheduler = SchedulerAsync::new();
        let scheduler = Arc::new(scheduler);
        let sc = scheduler.clone();
        let mut nucleus = Nucleus::new(receiver_cage, scheduler, context, wasm);
        let sender_cage_clone = sender_cage.clone();
        task::spawn_blocking(move || {
            nucleus.run();
        });

        let (sender_reply, receiver_reply) = tokio::sync::oneshot::channel();
        sender_cage
            .send((
                0,
                Gluon::PostRequest {
                    endpoint: "test_set_perfect_tree_mod_timer".to_owned(),
                    payload: <(i32, i32) as codec::Encode>::encode(&(1, 0)),
                    // reply_to: Some(sender_reply),
                    reply_to: Some(sender_reply),
                },
            ))
            .unwrap();
        let reply = receiver_reply.await.unwrap().unwrap();
        assert_eq!(decode(reply), 1);
        let (sender_reply, receiver_reply) = tokio::sync::oneshot::channel();
        sender_cage
            .send((
                0,
                Gluon::PostRequest {
                    endpoint: "test_stream".to_owned(),
                    payload: vec![],
                    // reply_to: Some(sender_reply),
                    reply_to: Some(sender_reply),
                },
            ))
            .unwrap();
        let reply = receiver_reply.await.unwrap().unwrap();
        assert_eq!(decode(reply), 555);
        tokio::spawn(async move {
            let mut last = 0;
            let result = [0, 0, 1, 2, 2, 3, 3, 4, 3, 4, 4, 5, 4, 5, 5, 6];
            while let Some(entry) = sc.pop().await {
                let sender = sender_cage_clone.clone();

                let (sender_reply, receiver_reply) = tokio::sync::oneshot::channel();
                if let Err(err) = sender.send((
                    0,
                    Gluon::PostRequest {
                        endpoint: entry.func_name,
                        payload: entry.func_params,
                        reply_to: Some(sender_reply),
                    },
                )) {
                    println!("fail to send timer entry: {:?}", err);
                }
                let reply = receiver_reply.await.unwrap().unwrap();
                assert!(result[decode(reply) as usize] >= last);
                last = result[last];

                // task::spawn_blocking(move || async move {
                //     let reply = receiver_reply.await.unwrap();
                //     println!("reply: {:?}", reply);
                // });
            }
        });
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
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

// fn map_to_nucleus<B, D, C>(
//     runtime_storage: Arc<C>,
//     hash: B::Hash,
//     metadata: RuntimeMetadata,
// ) -> Option<Vec<(NucleusId, NucleusEquation<AccountId, B::Hash>)>>
// where
//     B: sp_runtime::traits::Block,
//     D: Backend<B>,
//     C: BlockBackend<B> + StorageProvider<B, D> + BlockchainEvents<B> + 'static,
// {
//     let storage_key = storage_key(b"System", b"Events");
//     let events = runtime_storage
//         .storage(hash, &storage_key)
//         .ok()
//         .flatten()
//         .map(|v| events::decode_from::<SubstrateConfig>(v.0, metadata))?;
//     let instances = events
//         .find::<codegen::nucleus::events::InstanceRegistered>()
//         .filter_map(|ev| {
//             let id = ev.ok()?.id.clone();
//             let storage_key = blake2_128concat_storage_key(b"Nucleus", b"Nuclei", id.clone());
//             runtime_storage
//                 .storage(hash, &storage_key)
//                 .ok()
//                 .flatten()
//                 .map(|v| {
//                     let mut buf = v.0.as_ref();
//                     let decoded =
//                         <NucleusEquation<AccountId, B::Hash, NodeAddress>>::decode(&mut buf)
//                             .ok()?;
//                     Some((NucleusId::new(id.0), decoded))
//                 })
//                 .flatten()
//         })
//         .collect::<Vec<_>>();
//     Some(instances)
// }

// fn blake2_128concat_storage_key<K: codec::Encode>(
//     module: &[u8],
//     storage: &[u8],
//     key: K,
// ) -> sp_core::storage::StorageKey {
//     let mut bytes = sp_core::twox_128(module).to_vec();
//     bytes.extend(&sp_core::twox_128(storage)[..]);
//     let encoded = key.encode();
//     let x: &[u8] = encoded.as_slice();
//     let v = sp_core::blake2_128(x)
//         .iter()
//         .chain(x.iter())
//         .cloned()
//         .collect::<Vec<_>>();
//     bytes.extend(v);
//     sp_core::storage::StorageKey(bytes)
// }

// fn storage_key(module: &[u8], storage: &[u8]) -> sp_core::storage::StorageKey {
//     let mut bytes = sp_core::twox_128(module).to_vec();
//     bytes.extend(&sp_core::twox_128(storage)[..]);
//     sp_core::storage::StorageKey(bytes)
// }

// TODO
// fn start_nucleus<B>(
//     id: NucleusId,
//     equation: NucleusEquation<AccountId, B::Hash>,
//     db_path: std::path::PathBuf,
//     nuclei: &mut HashMap<NucleusId, NucleusCage>,
// ) -> anyhow::Result<()>
// where
//     B: sp_runtime::traits::Block,
// {
//     let NucleusEquation {
//         name,
//         account,
//         wasm_url,
//         wasm_hash,
//         wasm_version,
//         energy,
//         current_event,
//         root_state,
//         capacity,
//     } = equation;
//     let name = String::from_utf8(name)?;
//     let url = String::from_utf8(wasm_url)?;
//     let wasm = WasmInfo {
//         account,
//         name,
//         version: 0,
//         code: WasmCodeRef::File(url),
//     };
//     let config = ContextConfig {
//         db_path: db_path.into_boxed_path(),
//     };
//     // TODO
//     let context = Context::init(config)?;
//     let db = context.db.clone();
//     let (tx, rx) = std::sync::mpsc::channel();
//     let mut nucleus = Nucleus::new(rx, context, wasm);
//     std::thread::spawn(move || {
//         nucleus.run();
//     });
//     nuclei.insert(
//         id.clone(),
//         NucleusCage {
//             nucleus_id: id,
//             tunnel: tx,
//             pending_requests: vec![],
//             event_id: 0,
//             db,
//         },
//     );
//     Ok(())
// }
