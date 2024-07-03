use crate::wasm_code::{WasmCodeRef, WasmInfo};
use nucleus_core::Context;
use wasmtime::{Engine, ExternRef, Instance, Linker, Module, Rooted, Store, Val};

#[derive(Debug)]
pub struct Vm {
    space: Store,
    module: Module,
    linker: Linker,
    instance: Instance,
    // TODO descriptor
}

impl Vm {
    pub fn new_instance(wasm: WasmInfo, context: Context) -> Option<Self> {
        let engine = Engine::default();
        let module = match wasm.code {
            WasmCodeRef::Blob(ref blob) => Module::from_binary(&engine, blob).ok()?,
            WasmCodeRef::File(ref path) => Module::from_file(&engine, path).ok()?,
        };
        let linker = Linker::new(&engine);
        let mut store = Store::new(&engine, context.clone());
        let instance = linker.instantiate(&mut store, &module)?;
        // TODO resolve descriptor
        Self {
            space: store,
            module,
            linker,
            instance,
        }
    }

    // pub fn call_init(&mut self, func: &str, args: Vec<u8>) ->
}
