use crate::{
    runtime::{ContextAware, FuncRegister},
    vm::Vm,
    ReplyTo, TimerEntry, TimersReplyTo, WasmInfo,
};
use codec::{Decode, Encode};
use std::sync::mpsc::Receiver;
use vrs_core_sdk::{
    http::{HttpRequest, HttpResponse},
    CallResult,
};

pub struct Nucleus<R> {
    receiver: Receiver<(u64, Gluon)>,
    vm: Option<Vm<R>>,
}
#[derive(Debug, Decode)]
pub struct CodeUpgrade {
    version: u32,
    digest: [u8; 32],
}
#[derive(Debug)]
pub enum Gluon {
    CodeUpgrade(CodeUpgrade),
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
    // TODO add task_id
    TimerRequest {
        endpoint: String,
        payload: Vec<u8>,
        reply_to: Option<ReplyTo>,
        pending_timer_queue: TimersReplyTo,
    },
    TimerInitRequest {
        pending_timer_queue: TimersReplyTo,
    },
    HttpCallback {
        request_id: u64,
        payload: CallResult<HttpResponse>,
    },
}

impl From<&Gluon> for Event {
    fn from(gluon: &Gluon) -> Self {
        match gluon {
            Gluon::CodeUpgrade(CodeUpgrade { version, digest }) => Event::CodeUpgrade {
                version: *version,
                digest: *digest,
            },
            Gluon::PostRequest {
                endpoint, payload, ..
            } => Event::PostRequest {
                endpoint: endpoint.clone(),
                payload: payload.clone(),
            },
            Gluon::GetRequest { .. } => Event::Dummy,
            Gluon::TimerRequest {
                // task_id,
                endpoint,
                payload,
                ..
            } => Event::TimerTrigger {
                task_id: 0,
                endpoint: endpoint.clone(),
                payload: payload.clone(),
            },
            Gluon::HttpCallback {
                request_id,
                payload,
            } => Event::HttpCallback {
                request_id: *request_id,
                payload: payload.clone(),
            },
            Gluon::TimerInitRequest { .. } => Event::TimerInit {},
        }
    }
}

impl Into<Event> for Gluon {
    fn into(self) -> Event {
        match self {
            Self::CodeUpgrade(CodeUpgrade { version, digest }) => {
                Event::CodeUpgrade { version, digest }
            }
            Self::PostRequest {
                endpoint, payload, ..
            } => Event::PostRequest { endpoint, payload },
            Self::GetRequest { .. } => Event::Dummy,
            Self::TimerRequest {
                // task_id,
                endpoint,
                payload,
                ..
            } => Event::TimerTrigger {
                task_id: 0,
                endpoint,
                payload,
            },
            Self::HttpCallback {
                request_id,
                payload,
            } => Event::HttpCallback {
                request_id,
                payload,
            },
            Self::TimerInitRequest {
                pending_timer_queue,
            } => Event::TimerInit {},
        }
    }
}

/// Serialized `Gluon`, i.e. received from peers or call `gluon.into()` then save it
#[derive(Debug, Encode, Decode, Clone)]
pub enum Event {
    #[codec(index = 0)]
    CodeUpgrade { version: u32, digest: [u8; 32] },
    #[codec(index = 1)]
    PostRequest { endpoint: String, payload: Vec<u8> },
    #[codec(index = 2)]
    TimerRegister {
        task_id: u64,
        delay: u64,
        endpoint: String,
        payload: Vec<u8>,
    },
    #[codec(index = 3)]
    TimerTrigger {
        task_id: u64,
        endpoint: String,
        payload: Vec<u8>,
    },
    #[codec(index = 4)]
    HttpRequest {
        request_id: u64,
        ssl_key: Vec<u8>,
        request: HttpRequest,
    },
    #[codec(index = 5)]
    HttpCallback {
        request_id: u64,
        payload: CallResult<HttpResponse>,
    },
    #[codec(index = 6)]
    TimerInit {},
    #[codec(skip)]
    Dummy,
}

// define VM error type id
const VM_ERROR: i32 = 0x00000001;

impl<R> Nucleus<R>
where
    R: ContextAware + FuncRegister<Runtime = R>,
{
    pub(crate) fn init(
        receiver: Receiver<(u64, Gluon)>,
        runtime: R,
        code: WasmInfo,
    ) -> anyhow::Result<Self> {
        let vm = Vm::new_instance(&code, runtime).inspect_err(|e| {
            log::error!(
                "fail to create vm instance for AVS {} due to: {:?}",
                &code.name,
                e
            )
        })?;
        Ok(Nucleus {
            receiver,
            vm: Some(vm),
        })
    }

    pub(crate) fn run(&mut self) {
        while let Ok((id, msg)) = self.receiver.recv() {
            // TODO save msg with id to state
            // only interrupted if events save failed or VM not initialized
            if let Err(e) = self.accept(msg) {
                log::error!("Nucleus interrupted due to: {:?}", e);
                break;
            }
        }
    }

    fn accept(&mut self, msg: Gluon) -> anyhow::Result<()> {
        let vm = self
            .vm
            .as_mut()
            .ok_or(anyhow::anyhow!("VM not initialized"))?;
        match msg {
            Gluon::CodeUpgrade(CodeUpgrade { version, digest }) => {
                // TODO load new module from storage
            }
            Gluon::HttpCallback {
                request_id,
                payload,
            } => {
                let _ = vm.call_inner("__nucleus_http_callback", (request_id, payload));
            }
            Gluon::GetRequest {
                endpoint,
                payload,
                reply_to,
            } => {
                let vm_result = vm.call_get(&endpoint, payload).map_err(|e| {
                    log::error!("fail to call get endpoint: {} due to: {:?}", &endpoint, e);
                    (VM_ERROR << 10 + e.to_error_code(), e.to_string())
                });
                if let Some(reply_to) = reply_to {
                    let _ = reply_to.send(vm_result);
                }
            }
            Gluon::PostRequest {
                endpoint,
                payload,
                reply_to,
            } => {
                match self.vm {
                    Some(ref mut vm) => {
                        let vm_result = vm.call_post(&endpoint, payload.clone()).map_err(|e| {
                            log::error!(
                                "fail to call post endpoint: {} due to: {:?}",
                                &endpoint,
                                e
                            );
                            (VM_ERROR << 10 + e.to_error_code(), e.to_string())
                        });
                        // TODO mark: we need to re-design the caller context
                        // while let Some(entry) = vm.pop_pending_timer() {
                        //     // println!("{:?}", entry);
                        //     self.scheduler.push(entry);
                        // }
                        if let Some(reply_to) = reply_to {
                            if let Err(err) = reply_to.send(vm_result) {
                                println!("fail to send reply to: {:?}", err);
                                log::error!("fail to send reply to: {:?}", err);
                            }
                        } else {
                            log::error!("reply_to not found");
                        }
                    }
                    None => {
                        log::error!("vm not initialized");
                    }
                }
            }
            Gluon::TimerRequest {
                endpoint,
                payload,
                reply_to,
                pending_timer_queue,
            } => match self.vm {
                Some(ref mut vm) => {
                    let vm_result = vm.call_timer(&endpoint, payload.clone());
                    match vm_result {
                        Ok((result, entries)) => {
                            if let Some(reply_to) = reply_to {
                                if let Err(err) = reply_to.send(Ok(result)) {
                                    log::error!("⏲️ Fail to send reply to: {:?}", err);
                                }
                            }
                            pending_timer_queue.send(entries).unwrap();
                        }
                        Err(e) => {
                            log::error!(
                                "fail to call timer endpoint: {} due to: {:?}",
                                &endpoint,
                                e
                            );

                            if let Some(reply_to) = reply_to {
                                if let Err(err) = reply_to
                                    .send(Err((VM_ERROR << 10 + e.to_error_code(), e.to_string())))
                                {
                                    log::error!("⏲️ Fail to send reply to: {:?}", err);
                                }
                            }
                        }
                    }
                }
                None => {
                    log::error!("vm not initialized");
                }
            },
            Gluon::TimerInitRequest {
                pending_timer_queue,
            } => match self.vm {
                Some(ref mut vm) => {
                    let vm_result = vm.call_init();
                    match vm_result {
                        Ok(entries) => {
                            pending_timer_queue.send(entries).unwrap();
                        }
                        Err(e) => {
                            log::error!(
                                "fail to call timer endpoint: {} due to: {:?}",
                                "TimerInit",
                                e
                            );
                            pending_timer_queue.send(vec![]).unwrap();
                        }
                    }
                }
                None => {
                    log::error!("vm not initialized");
                    pending_timer_queue.send(vec![]).unwrap();
                }
            },
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_suite::new_vm_with_executable, WasmCodeRef};

    use super::*;
    use codec::Decode;
    use temp_dir::TempDir;
    // use tokio::sync::mpsc;
    use std::sync::mpsc;
    use tokio::sync::oneshot;
    use vrs_core_sdk::AccountId;

    #[tokio::test]
    async fn test_nucleus_accept() {
        let wasm_path = "../../nucleus-examples/basic_macros.wasm";
        let (vm, _) = new_vm_with_executable(wasm_path.to_string());
        let (sender, receiver) = mpsc::channel();
        let mut nucleus = Nucleus {
            receiver,
            vm: Some(vm),
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
