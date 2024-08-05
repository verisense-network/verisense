use crate::{context::Context, vm::Vm, wasm_code::WasmInfo, ReplyTo};
use std::collections::HashMap;
use tokio::sync::mpsc::Receiver;

pub(crate) struct Nucleus {
    receiver: Receiver<(u64, Gluon)>,
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
    fn new(receiver: Receiver<(u64, Gluon)>, context: Context, code: WasmInfo) -> Self {
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
        Nucleus { receiver, vm }
    }

    fn accept(&mut self, msg: Gluon) {
        // TODO if token:
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
                        (VM_ERROR, e.to_string())
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
                        let vm_result = vm.call_post(&endpoint, payload).map_err(|e| {
                            log::error!("fail to call get endpoint: {} due to: {:?}", &endpoint, e);
                            (VM_ERROR, e.to_string())
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
    use tokio::sync::mpsc;
    use tokio::sync::oneshot;
    use vrs_core_sdk::AccountId;

    #[tokio::test]
    async fn test_nucleus_accept() {
        let wasm_path = "../nucleus-examples/vrs_nucleus_examples.wasm";
        let wasm = WasmInfo {
            account: AccountId::new([0u8; 32]),
            name: "avs-dev-demo".to_string(),
            version: 0,
            code: WasmCodeRef::File(wasm_path.to_string()),
        };

        let context = Context::init().unwrap();
        let vm = Vm::new_instance(&wasm, context).unwrap();

        let (sender, receiver) = mpsc::channel(100);
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
        sender.send((0, get_msg)).await.unwrap();

        let (tx_post, rx_post) = oneshot::channel();
        let post_msg = Gluon::PostRequest {
            endpoint: "cc".to_string(),
            payload: vec![
                40, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 40, 98, 98, 98, 98, 98, 98, 98, 98, 98,
                98,
            ],
            reply_to: Some(tx_post),
        };
        sender.send((1, post_msg)).await.unwrap();

        for _ in 0..2 {
            let (_, msg) = nucleus.receiver.recv().await.unwrap();
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
    }
}

// TODO spawn a task to run
// pub async fn run(mut nucleus: Nucleus) {
//     while let Some(msg) = nucleus.receiver.recv().await {
//         nucleus.accept(msg).await;
//     }
// }
