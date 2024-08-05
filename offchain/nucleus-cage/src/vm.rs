use std::any::Any;

use crate::{
    context::Context,
    wasm_code::{FunctionDescriptor, WasmCodeRef, WasmDescriptor, WasmInfo},
};
use vrs_core_sdk::AccountId;
use wasmtime::{Engine, ExternRef, ExternType, Instance, Module, Rooted, Store, Val, WasmResults};

pub struct Vm {
    space: Store<Context>,
    instance: Instance,
}

impl Vm {
    pub fn new_instance(wasm: &WasmInfo, context: Context) -> anyhow::Result<Self> {
        let engine = Engine::default();
        let module = match wasm.code {
            WasmCodeRef::Blob(ref blob) => Module::from_binary(&engine, blob)?,
            WasmCodeRef::File(ref path) => Module::from_file(&engine, path)?,
        };
        module.exports().for_each(|ty| match ty.ty() {
            ExternType::Func(func) => {
                log::info!("export: {} {}", func.to_string(), ty.name());
            }
            _ => {}
        });
        let mut store = Store::new(&engine, context);
        let injects = Context::inject_host_funcs(&mut store);
        let instance = Instance::new(&mut store, &module, &injects)?;
        instance.exports(&mut store).for_each(|export| {
            log::info!("export: {}", export.name());
        });
        Ok(Self {
            space: store,
            instance,
        })
    }

    pub fn call_post(&mut self, func: &str, args: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        // TODO handle endpoint not found
        let post_fn = self
            .instance
            .get_func(&mut self.space, func)
            .ok_or(anyhow::anyhow!("endpoint not found"))?;
        let mm = self
            .instance
            .get_export(&mut self.space, "memory")
            .ok_or(anyhow::anyhow!("no memory exported"))?
            .into_memory()
            .ok_or(anyhow::anyhow!("no memory exported"))?;
        mm.write(&mut self.space, 1, &args)?;
        let mut results = vec![Val::I32(0), Val::I32(0)];
        post_fn.call(
            &mut self.space,
            &[Val::I32(1), Val::I32(args.len() as i32)],
            &mut results,
        )?;
        log::info!("results: {:?}", results);
        Ok(vec![0u8])
    }

    // pub fn call_init(&mut self, func: &str, args: Vec<u8>)
    // pub fn call_query(&mut self, func: &str, args: Vec<u8>)
    // TODO
    // pub fn call_post(&mut self, func: &str, args: u32) -> u32 {
    //     let post_fn = self
    //         .instance
    //         .get_typed_func::<(u32,), u32>(&mut self.space, func)
    //         .unwrap();
    //     post_fn.call(&mut self.space, (args,)).unwrap()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn load_wasm_should_work() {
        env_logger::init();
        let wasm_path = "../../target/wasm32-unknown-unknown/debug/vrs_nucleus_examples.wasm";
        let wasm = WasmInfo {
            account: AccountId::new([0u8; 32]),
            name: "avs-dev-demo".to_string(),
            version: 0,
            code: WasmCodeRef::File(wasm_path.to_string()),
        };

        let context = Context::init().unwrap();
        let mut vm = Vm::new_instance(&wasm, context).unwrap();
        use codec::{Decode, Encode};
        #[derive(Debug, Encode, Decode)]
        pub struct E {
            pub a: Vec<u32>,
            pub b: i32,
            pub c: u32,
        }
        let e = E {
            a: vec![],
            b: 0,
            c: 1,
        };
        let encoded_args = e.encode();
        assert_eq!(
            vec![0u8],
            //jsonrpc: {method: AVS_{AVSNAME}_{METHODNAME}, params: [], id: 1}
            vm.call_post("__nucleus_post_post", encoded_args).unwrap()
        );
    }
}
