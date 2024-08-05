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

    async fn accept(&mut self, msg: Gluon) {
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
            } => {
                // TODO resolve parameters
                // let vm_result = self.vm.run_func(None, &endpoint, vec![]);
            }
            Gluon::PostRequest {
                endpoint,
                payload,
                reply_to,
            } => {
                // let vm_result = self.vm.run_func(None, &endpoint, payload);
                // vec![]
            }
        }
    }
}

// TODO spawn a task to run
// pub async fn run(mut nucleus: Nucleus) {
//     while let Some(msg) = nucleus.receiver.recv().await {
//         nucleus.accept(msg).await;
//     }
// }
