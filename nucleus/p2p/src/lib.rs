// use codec::{Decode, Encode};
use futures::prelude::*;
use sc_client_api::{Backend, BlockBackend, BlockchainEvents, StorageProvider};
use sp_api::{Metadata, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
// use std::collections::HashMap;
// use std::sync::mpsc::Sender as SyncSender;
use std::sync::Arc;
use vrs_primitives::{AccountId, Hash, NucleusId};

use sc_network::request_responses::IncomingRequest;

pub struct P2pParams<B, C, BN> {
    pub reqres_receiver: async_channel::Receiver<IncomingRequest>,
    pub client: Arc<C>,
    pub p2p_cage_tx: tokio::sync::mpsc::Sender<IncomingRequest>,
    pub controller: AccountId,
    pub _phantom: std::marker::PhantomData<(B, BN)>,
}

struct NucleusP2p {}

impl NucleusP2p {}

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
        p2p_cage_tx,
        controller,
        _phantom,
    } = params;
    async move {
        log::info!("🔌 Nucleus p2p controller: {}", controller);
        loop {
            tokio::select! {
                // poll the msg from another p2p peer
                // The msg type is: sc_network::request_responses::IncomingRequest
                // pub struct IncomingRequest {
                //     pub peer: PeerId,
                //     pub payload: Vec<u8>,
                //     pub pending_response: Sender<OutgoingResponse>,
                // }
                Ok(request) = reqres_receiver.recv() => {
                    println!("IncomingRequest msg: {:?}", request);
                    // do stuff
                    // forward the request to cage
                    _ = p2p_cage_tx.send(request).await;

                    // use request.pending_response to send oneshot OutgoingResponse back

                    // API: anywhere you want to send request, use like:
                    // let network_worker = ...;
                    // let service = network_worker.service();
                    // let res: (Vec<u8>, ProtocolName) = service.request(peer_id, ProtocolName::Static("xxxx"), vec_u8_payload, None, IfDisconnected::ImmediateError).await?;
                    // here the res is the data sent above by request.pending_response, just simple as that

                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    //     use super::*;
    //     use crate::SchedulerAsync;
    //     use crate::{nucleus::Nucleus, scheduler, vm::Vm, Scheduler, WrappedSchedulerSync};
    //     use codec::Decode;
    //     use futures::channel::mpsc::SendError;
    //     use rocksdb::Options;
    //     use sp_core::hexdisplay::AsBytesRef;
    //     use std::thread;
    //     use std::{sync::Arc, time::Duration};
    //     use temp_dir::TempDir;
    //     use tokio::sync::{oneshot, RwLock};
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
    //     fn decode(data: Vec<u8>) -> i32 {
    //         let mut reply = data.as_slice();
    //         let reply = <Result<i32, String> as codec::Decode>::decode(&mut reply)
    //             .unwrap()
    //             .unwrap();
    //         reply
    //     }
    //     #[tokio::test]
    //     async fn test_scheduler_async() {
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
    //         let scheduler = SchedulerAsync::new();
    //         let scheduler = Arc::new(scheduler);
    //         let sc = scheduler.clone();
    //         let mut nucleus = Nucleus::new(receiver_cage, scheduler, context, wasm);
    //         let sender_cage_clone = sender_cage.clone();
    //         task::spawn_blocking(move || {
    //             nucleus.run();
    //         });

    //         let (sender_reply, receiver_reply) = tokio::sync::oneshot::channel();
    //         sender_cage
    //             .send((
    //                 0,
    //                 Gluon::PostRequest {
    //                     endpoint: "test_set_perfect_tree_mod_timer".to_owned(),
    //                     payload: <(i32, i32) as codec::Encode>::encode(&(1, 0)),
    //                     // reply_to: Some(sender_reply),
    //                     reply_to: Some(sender_reply),
    //                 },
    //             ))
    //             .unwrap();
    //         let reply = receiver_reply.await.unwrap().unwrap();
    //         assert_eq!(decode(reply), 1);
    //         let (sender_reply, receiver_reply) = tokio::sync::oneshot::channel();
    //         sender_cage
    //             .send((
    //                 0,
    //                 Gluon::PostRequest {
    //                     endpoint: "test_stream".to_owned(),
    //                     payload: vec![],
    //                     // reply_to: Some(sender_reply),
    //                     reply_to: Some(sender_reply),
    //                 },
    //             ))
    //             .unwrap();
    //         let reply = receiver_reply.await.unwrap().unwrap();
    //         assert_eq!(decode(reply), 555);
    //         tokio::spawn(async move {
    //             let mut last = 0;
    //             let result = [0, 0, 1, 2, 2, 3, 3, 4, 3, 4, 4, 5, 4, 5, 5, 6];
    //             while let Some(entry) = sc.pop().await {
    //                 let sender = sender_cage_clone.clone();

    //                 let (sender_reply, receiver_reply) = tokio::sync::oneshot::channel();
    //                 if let Err(err) = sender.send((
    //                     0,
    //                     Gluon::PostRequest {
    //                         endpoint: entry.func_name,
    //                         payload: entry.func_params,
    //                         reply_to: Some(sender_reply),
    //                     },
    //                 )) {
    //                     println!("fail to send timer entry: {:?}", err);
    //                 }
    //                 let reply = receiver_reply.await.unwrap().unwrap();
    //                 assert!(result[decode(reply) as usize] >= last);
    //                 last = result[last];

    //                 // task::spawn_blocking(move || async move {
    //                 //     let reply = receiver_reply.await.unwrap();
    //                 //     println!("reply: {:?}", reply);
    //                 // });
    //             }
    //         });
    //         tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    //     }
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
