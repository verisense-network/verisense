use crate::{
    runtime::{ContextAware, FuncRegister},
    vm::Vm,
    CallerInfo, ReplyTo, TimersReplyTo, WasmInfo,
};
use std::sync::mpsc::Receiver;
use vrs_core_sdk::{http::HttpResponse, CallResult};

pub(crate) struct Nucleus<R> {
    receiver: Receiver<(u64, Gluon)>,
    vm: Option<Vm<R>>,
}

#[derive(Debug)]
pub enum Gluon {
    CodeUpgrade {
        // TODO
        version: u32,
    },
    TimerRequest {
        endpoint: String,
        payload: Vec<u8>,
        reply_to: Option<ReplyTo>,
        pending_timer_queue: TimersReplyTo,
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
    HttpCallback {
        request_id: u64,
        payload: CallResult<HttpResponse>,
    },
}
// define VM error type id
const VM_ERROR: i32 = 0x00000001;

impl<R> Nucleus<R>
where
    R: ContextAware + FuncRegister<Runtime = R>,
{
    pub(crate) fn new(receiver: Receiver<(u64, Gluon)>, runtime: R, code: WasmInfo) -> Self {
        // TODO
        let vm = Vm::new_instance(&code, runtime)
            .inspect_err(|e| {
                log::error!(
                    "fail to create vm instance for AVS {} due to: {:?}",
                    &code.name,
                    e
                )
            })
            .ok();
        Nucleus { receiver, vm }
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
            Gluon::HttpCallback {
                request_id,
                payload,
            } => {
                self.vm.as_mut().map(|vm| {
                    vm.call_inner("__nucleus_http_callback", (request_id, payload))
                        .inspect_err(|e| {
                            log::error!("fail to http callback due to: {:?}", e);
                        })
                        .ok();
                });
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
                // let vm_result = self.vm.run_func(None, &endpoint, payload);
                // vec![]
            }
            Gluon::TimerRequest {
                endpoint,
                payload,
                reply_to,
                pending_timer_queue,
            } => match self.vm {
                Some(ref mut vm) => {
                    let vm_result = vm.call_timer(
                        &endpoint,
                        payload.clone(),
                        vec![CallerInfo {
                            func: "Gluon::TimerRequest".to_string(),
                            params: payload,
                            thread_id: 0,
                            caller_type: crate::CallerType::Entry,
                        }],
                    );
                    match vm_result {
                        Ok((result, entries)) => {
                            if let Some(reply_to) = reply_to {
                                if let Err(err) = reply_to.send(Ok(result)) {
                                    println!("fail to send reply to: {:?}", err);
                                    log::error!("fail to send reply to: {:?}", err);
                                }
                            } else {
                                log::error!("reply_to not found");
                            }
                            pending_timer_queue.send(entries);
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
                                    println!("fail to send reply to: {:?}", err);
                                    log::error!("fail to send reply to: {:?}", err);
                                }
                            } else {
                                log::error!("reply_to not found");
                            }
                        }
                    }
                }
                None => {
                    log::error!("vm not initialized");
                }
            },
        }
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
