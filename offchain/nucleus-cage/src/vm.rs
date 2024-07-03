use crate::wasm_code::{WasmCodeRef, WasmInfo};
use nucleus_core::Context;
use wasmtime::{Engine, ExternRef, Instance, Linker, Module, Rooted, Store, Val};

pub struct Vm {
    space: Store<Context>,
    module: Module,
    linker: Linker<Context>,
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
        let instance = linker.instantiate(&mut store, &module).ok()?;
        // TODO resolve descriptor
        Some(Self {
            space: store,
            module,
            linker,
            instance,
        })
    }

    // pub fn call_init(&mut self, func: &str, args: Vec<u8>)
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
