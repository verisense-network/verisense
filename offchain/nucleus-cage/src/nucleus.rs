use crate::{vm::Vm, wasm_code::WasmInfo};
use nucleus_core::Context;
use std::collections::HashMap;
use tokio::sync::mpsc::Receiver;

pub(crate) struct Nucleus {
    receiver: Receiver<Gluon>,
    context: Context,
    vm: Option<Vm>,
}

#[derive(Clone, Debug)]
pub(crate) enum Gluon {
    CodeUpgrade {
        id: u64,
        version: u32,
    },
    PostRequest {
        id: u64,
        endpoint: String,
        payload: Vec<u8>,
        reply: Option<u64>,
    },
    GetRequest {
        endpoint: String,
        payload: Vec<u8>,
        reply: u64,
    },
}

impl Nucleus {
    fn new(receiver: Receiver<Gluon>, context: Context, code: WasmInfo) -> Self {
        // TODO
        let vm = Vm::new_instance(code, context.clone());
        Nucleus {
            receiver,
            context,
            vm,
        }
    }

    async fn accept(&mut self, msg: Gluon) {
        match msg {
            Gluon::CodeUpgrade { id, version } => {
                // TODO load new module from storage
                // TODO handle errors
            }
            Gluon::GetRequest {
                endpoint,
                payload,
                reply,
            } => {
                // TODO resolve parameters
                // let vm_result = self.vm.run_func(None, &endpoint, vec![]);
            }
            Gluon::PostRequest {
                id,
                endpoint,
                payload,
                reply,
            } => {
                // let vm_result = self.vm.run_func(None, &endpoint, vec![]);
            }
        }
    }
}

// TODO spawn a task to run
pub async fn run(mut nucleus: Nucleus) {
    while let Some(msg) = nucleus.receiver.recv().await {
        nucleus.accept(msg).await;
    }
}
