use chrono::{DateTime, Duration, Utc};
use tokio::time;

use crate::{
    context::{Context, ContextConfig},
    vm::Vm,
    wasm_code::WasmInfo,
    CallerInfo, ReplyTo, TimerEntry,
};
use std::{sync::mpsc::Receiver, thread::sleep};
use tokio::sync::mpsc::Sender;

pub(crate) struct Nucleus {
    receiver: Receiver<(u64, Gluon)>,
    scheduler_timer_sender: Sender<TimerEntry>,
    vm: Option<Vm>,
}

#[derive(Debug)]
pub enum Gluon {
    CodeUpgrade {
        // TODO
        version: u32,
    },
    PostRequest {
        endpoint: String,
        payload: Vec<u8>,
        reply_to: Option<ReplyTo>,
    },
    GetRequest {
        endpoint: String,
        payload: Vec<u8>,
        reply_to: Option<ReplyTo>,
    },
}
// define VM error type id
const VM_ERROR: i32 = 0x00000001;

impl Nucleus {
    pub(crate) fn new(
        receiver: Receiver<(u64, Gluon)>,
        scheduler_timer_sender: Sender<TimerEntry>,
        context: Context,
        code: WasmInfo,
    ) -> Self {
        // TODO
        let vm = Vm::new_instance(&code, context)
            .inspect_err(|e| {
                log::error!(
                    "fail to create vm instance for AVS {} due to: {:?}",
                    &code.name,
                    e
                )
            })
            .ok();
        Nucleus {
            receiver,
            vm,
            scheduler_timer_sender,
        }
    }

    pub(crate) fn run(&mut self) {
        while let Ok((id, msg)) = self.receiver.recv() {
            // TODO save msg with id to rocksdb
            self.accept(msg);
        }
    }

    fn accept(&mut self, msg: Gluon) {
        match msg {
            Gluon::CodeUpgrade { version } => {
                // TODO load new module from storage
                // TODO handle errors
            }
            Gluon::GetRequest {
                endpoint,
                payload,
                reply_to,
            } => match self.vm {
                Some(ref mut vm) => {
                    let vm_result = vm.call_get(&endpoint, payload).map_err(|e| {
                        log::error!("fail to call get endpoint: {} due to: {:?}", &endpoint, e);
                        (VM_ERROR << 10 + e.to_error_code(), e.to_string())
                    });
                    if let Some(reply_to) = reply_to {
                        if let Err(err) = reply_to.send(vm_result) {
                            log::error!("fail to send reply to: {:?}", err);
                        }
                    } else {
                        log::error!("reply_to not found");
                    }
                }
                None => {
                    log::error!("vm not initialized");
                }
            },

            Gluon::PostRequest {
                endpoint,
                payload,
                reply_to,
            } => {
                match self.vm {
                    Some(ref mut vm) => {
                        let vm_result = vm
                            .call_post(
                                &endpoint,
                                payload.clone(),
                                vec![CallerInfo {
                                    func: "Gluon::PostRequest".to_string(),
                                    params: payload,
                                    thread_id: 0,
                                    caller_type: crate::CallerType::Entry,
                                }],
                            )
                            .map_err(|e| {
                                log::error!(
                                    "fail to call post endpoint: {} due to: {:?}",
                                    &endpoint,
                                    e
                                );
                                (VM_ERROR << 10 + e.to_error_code(), e.to_string())
                            });
                        if let Some(reply_to) = reply_to {
                            if let Err(err) = reply_to.send(vm_result) {
                                log::error!("fail to send reply to: {:?}", err);
                            }
                        } else {
                            log::error!("reply_to not found");
                        }
                        while let Some(entry) = vm.pop_pending_timer() {
                            let entry = entry.clone();
                            let sender = self.scheduler_timer_sender.clone();
                            tokio::spawn(async move {
                                if let Err(err) = sender.send(entry).await {
                                    log::error!("fail to send timer entry: {:?}", err);
                                }
                            });
                        }
                    }
                    None => {
                        log::error!("vm not initialized");
                    }
                }
                // let vm_result = self.vm.run_func(None, &endpoint, payload);
                // vec![]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wasm_code::WasmCodeRef;

    use super::*;
    use codec::Decode;
    use temp_dir::TempDir;
    // use tokio::sync::mpsc;
    use std::sync::mpsc;
    use tokio::sync::oneshot;
    use vrs_core_sdk::AccountId;

    #[tokio::test]
    async fn test_nucleus_accept() {
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
        let vm = Vm::new_instance(&wasm, context).unwrap();

        let (sender, receiver) = mpsc::channel();
        let (sender1, receiver1) = tokio::sync::mpsc::channel(123);
        let mut nucleus = Nucleus {
            receiver,
            vm: Some(vm),
            scheduler_timer_sender: sender1,
        };

        let (tx_get, rx_get) = oneshot::channel();
        let get_msg = Gluon::GetRequest {
            endpoint: "get".to_string(),
            payload: vec![],
            reply_to: Some(tx_get),
        };
        sender.send((0, get_msg)).unwrap();

        let (tx_post, rx_post) = oneshot::channel();
        let post_msg = Gluon::PostRequest {
            endpoint: "cc".to_string(),
            payload: vec![
                40, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 40, 98, 98, 98, 98, 98, 98, 98, 98, 98,
                98,
            ],
            reply_to: Some(tx_post),
        };
        sender.send((1, post_msg)).unwrap();

        let (tx_post, rx_post1) = oneshot::channel();
        let post_msg = Gluon::PostRequest {
            endpoint: "bc".to_string(),
            payload: vec![
                40, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 40, 98, 98, 98, 98, 98, 98, 98, 98, 98,
                98,
            ],
            reply_to: Some(tx_post),
        };
        sender.send((1, post_msg)).unwrap();

        for _ in 0..3 {
            let (_, msg) = nucleus.receiver.recv().unwrap();
            nucleus.accept(msg);
        }

        let get_result = rx_get.await.unwrap().unwrap();
        let get_result = <i32 as Decode>::decode(&mut &get_result[..]).unwrap();
        assert_eq!(get_result, 5);
        let post_result = rx_post.await.unwrap().unwrap();
        let post_result =
            <Result<String, String> as Decode>::decode(&mut &post_result[..]).unwrap();
        assert_eq!(
            post_result,
            Result::<String, String>::Ok("abababababababababab".to_owned())
        );
        let post_result = rx_post1.await.unwrap();
        assert_eq!(post_result, Err((1024, "Endpoint not found".to_owned())))
    }
}
