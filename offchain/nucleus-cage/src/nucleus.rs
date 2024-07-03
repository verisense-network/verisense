use crate::vm::Vm;
use nucleus_core::Context;
use std::collections::HashMap;
use tokio::sync::mpsc::Receiver;
use wasmtime::{Engine, ExternRef, Instance, Linker, Module, Rooted, Store, Val};

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
    fn new(receiver: Receiver<Gluon>, context: Context) -> Self {
        let vm = Vm::new_instance();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn load_wasm_should_work() {
        let wasm_path = "../../target/wasm32-unknown-unknown/debug/nucleus_examples.wasm";
        let engine = Engine::default();
        let module = Module::from_file(&engine, wasm_path).unwrap();
        assert!(module.get_export("init").is_some());
        let linker = Linker::new(&engine);
        // let wasi = WasiCtxBuilder::new().build();
        let context = Context {};
        let mut store = Store::new(&engine, context);
        let instance = linker.instantiate(&mut store, &module).unwrap();
        assert!(instance.exports(&mut store).len() > 0);
        assert!(instance.get_export(&mut store, "init").is_some());
        let init_fn = instance
            .get_typed_func::<(), ()>(&mut store, "init")
            .unwrap();
        println!("{:?}", init_fn.call(&mut store, ()).unwrap());
        let post_fn = instance
            .get_typed_func::<(u32,), u32>(&mut store, "post1")
            .unwrap();
        // let ctx: Rooted<ExternRef> = ExternRef::new(&mut store, &mut context).unwrap();
        println!("{}", post_fn.call(&mut store, (32,)).unwrap());
        assert!(false);
    }
}
