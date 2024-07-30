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
        // let mm = self
        //     .instance
        //     .get_export(&mut self.space, "memory")
        //     .ok_or(anyhow::anyhow!("no memory exported"))?
        //     .into_memory()
        //     .ok_or(anyhow::anyhow!("no memory exported"))?;
        // mm.write(&mut self.space, 1, &args)?;
        // let mut results = vec![Val::I32(0), Val::I32(0)];
        let mut results = vec![];
        post_fn.call(
            &mut self.space,
            &[Val::I32(2), Val::I32(1), Val::I32(args.len() as i32)],
            &mut results,
        )?;
        println!("results: {:?}", results);
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
            a: vec![0],
            b: 0,
            c: 1,
        };
        let encoded_args = e.encode();
        println!(
            "encoded_args: {:?}",
            vm.call_post("__nucleus_post_post", encoded_args)
        );
        // assert_eq!(
        //     vec![0u8],
        //     vm.call_post("__nucleus_post_post", encoded_args).unwrap()
        // );
    }
    #[test]
    pub fn call_post_should_work() {
        env_logger::init();
        let wasm_path = "../../target/wasm32-unknown-unknown/debug/vrs_nucleus_examples.wasm";

        let engine = Engine::default();
        let module = Module::from_file(&engine, wasm_path).unwrap();
        module.exports().for_each(|ty| match ty.ty() {
            ExternType::Func(func) => {
                println!("export: {} {}", func.to_string(), ty.name());
            }
            _ => {}
        });
        let mut store = Store::new(&engine, Context::init().unwrap());
        let injects = Context::inject_host_funcs(&mut store);
        let instance = Instance::new(&mut store, &module, &injects).unwrap();
        let memory = instance
            .get_memory(&mut store, "memory")
            .expect("Failed to get memory");
        let input = b"Hello, a world with a lot of a characters!";
        let input_len = input.len().min(255);

        let ptr = memory.data_size(&store) as i32;
        memory.grow(&mut store, 1).unwrap();
        memory
            .write(&mut store, ptr as usize, &input[..input_len])
            .unwrap();

        let add = instance
            .get_func(&mut store, "replace_a_with_b")
            .expect("function not found");
        let mut result = vec![Val::I32(0)];
        let results = add
            .call(
                &mut store,
                &[Val::I32(ptr), Val::I32(input_len as i32)],
                &mut result,
            )
            .unwrap();
        let result_ptr = result[0].unwrap_i32() as usize;
        let mut result = vec![0u8; input_len];
        memory.read(&store, result_ptr, &mut result).unwrap();
        println!("results: {:?}", String::from_utf8(result).unwrap());
    }
}
