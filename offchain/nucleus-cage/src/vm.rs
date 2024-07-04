use crate::{
    context::Context,
    wasm_code::{WasmCodeRef, WasmInfo},
};
use nucleus_core::AccountId;
use wasmtime::{Engine, ExternRef, Instance, Module, Rooted, Store, Val, WasmResults};

pub struct Vm {
    space: Store<Context>,
    module: Module,
    // linker: Linker<Context>,
    instance: Instance,
    // TODO descriptor
}

impl Vm {
    pub fn new_instance(wasm: WasmInfo, context: Context) -> Option<Self> {
        let engine = Engine::default();
        let module = match wasm.code {
            WasmCodeRef::Blob(ref blob) => Module::from_binary(&engine, blob)
                .inspect_err(|e| log::error!("{:?}", e))
                .ok()?,
            WasmCodeRef::File(ref path) => Module::from_file(&engine, path)
                .inspect_err(|e| log::error!("{:?}", e))
                .ok()?,
        };
        let mut store = Store::new(&engine, context);
        let injects = Context::inject_host_funcs(&mut store);
        let instance = Instance::new(&mut store, &module, &injects)
            .inspect_err(|e| {
                log::error!("crate module instance for {} failed: {:?}", &wasm.name, e)
            })
            .ok()?;
        // TODO resolve descriptor
        Some(Self {
            space: store,
            module,
            // linker,
            instance,
        })
    }

    // pub fn call_init(&mut self, func: &str, args: Vec<u8>)
    // pub fn call_query(&mut self, func: &str, args: Vec<u8>)
    // TODO
    pub fn call_post(&mut self, func: &str, args: u32) -> u32 {
        let post_fn = self
            .instance
            .get_typed_func::<(u32,), u32>(&mut self.space, func)
            .unwrap();
        post_fn.call(&mut self.space, (args,)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn load_wasm_should_work() {
        env_logger::init();
        let wasm_path = "../../target/wasm32-unknown-unknown/debug/nucleus_examples.wasm";
        let wasm = WasmInfo {
            account: AccountId::new([0u8; 32]),
            name: "avs-dev-demo".to_string(),
            version: 0,
            code: WasmCodeRef::File(wasm_path.to_string()),
        };

        let context = Context::init().unwrap();
        let mut vm = Vm::new_instance(wasm, context).unwrap();
        assert_eq!(11, vm.call_post("post", 10));
    }
}
