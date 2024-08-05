use std::any::Any;

use crate::{
    context::Context,
    wasm_code::{FunctionDescriptor, WasmCodeRef, WasmDescriptor, WasmInfo},
};
use anyhow::anyhow;
use sp_runtime::traits::Member;
use vrs_core_sdk::AccountId;
use wasmtime::{Engine, ExternRef, ExternType, Instance, Module, Rooted, Store, Val, WasmResults};

pub struct Vm {
    space: Store<Context>,
    instance: Instance,
    __call_param_ptr: i32,
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
            log::debug!("export1: {}", export.name());
        });
        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or(anyhow::anyhow!("no memory exported"))?;

        let ptr = memory.data_size(&store) as i32;
        memory.grow(&mut store, 1).unwrap();
        Ok(Self {
            space: store,
            instance,
            __call_param_ptr: ptr,
        })
    }

    pub fn call_get(&mut self, func: &str, args: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let post_fn = self
            .instance
            .get_func(&mut self.space, format!("__nucleus_get_{}", func).as_str())
            .ok_or(anyhow::anyhow!("endpoint not found"))?;

        let memory = self
            .instance
            .get_memory(&mut self.space, "memory")
            .ok_or(anyhow::anyhow!("no memory exported"))?;
        let mut result = vec![Val::I32(0)];
        //check args size < 64k
        if args.len() > 65536 {
            return Err(anyhow::anyhow!("args size should be less than 64k"));
        }
        memory
            .write(&mut self.space, self.__call_param_ptr as usize, &args[..])
            .unwrap();
        post_fn.call(
            &mut self.space,
            &[
                Val::I32(self.__call_param_ptr as i32),
                Val::I32(args.len() as i32),
            ],
            &mut result,
        )?;
        log::info!("results: {:?}", result);
        let result_ptr = result[0].i32().ok_or(anyhow!("result ptr error"))? as usize;
        let mut result_len = vec![0u8; 4];
        memory.read(&self.space, result_ptr, &mut result_len)?;
        let result_len =
            u32::from_ne_bytes([result_len[0], result_len[1], result_len[2], result_len[3]]);
        if result_len > 65536 {
            return Err(anyhow::anyhow!("result size should be less than 64k"));
        }
        let mut result: Vec<u8> = vec![0u8; result_len as usize];
        memory.read(&self.space, result_ptr + 4, &mut result)?;
        log::debug!("result {:?}", result);
        let result = <Result<Vec<u8>, String> as codec::Decode>::decode(&mut result.as_slice())?
            .map_err(|e| anyhow!("wasm call error {:?}", e))?;
        return Ok(result);
    }

    pub fn call_post(&mut self, func: &str, args: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let post_fn = self
            .instance
            .get_func(&mut self.space, format!("__nucleus_post_{}", func).as_str())
            .ok_or(anyhow::anyhow!("endpoint not found"))?;

        let memory = self
            .instance
            .get_memory(&mut self.space, "memory")
            .ok_or(anyhow::anyhow!("no memory exported"))?;
        let mut result = vec![Val::I32(0)];
        //check args size < 64k
        if args.len() > 65536 {
            return Err(anyhow::anyhow!("args size should be less than 64k"));
        }
        memory
            .write(&mut self.space, self.__call_param_ptr as usize, &args[..])
            .unwrap();
        post_fn.call(
            &mut self.space,
            &[
                Val::I32(self.__call_param_ptr as i32),
                Val::I32(args.len() as i32),
            ],
            &mut result,
        )?;
        log::info!("results: {:?}", result);
        let result_ptr = result[0].i32().ok_or(anyhow!("result ptr error"))? as usize;
        let mut result_len = vec![0u8; 4];
        memory.read(&self.space, result_ptr, &mut result_len)?;
        let result_len =
            u32::from_ne_bytes([result_len[0], result_len[1], result_len[2], result_len[3]]);
        if result_len > 65536 {
            return Err(anyhow::anyhow!("result size should be less than 64k"));
        }
        let mut result: Vec<u8> = vec![0u8; result_len as usize];
        memory.read(&self.space, result_ptr + 4, &mut result)?;
        let result = <Result<Vec<u8>, String> as codec::Decode>::decode(&mut result.as_slice())?
            .map_err(|e| anyhow!("wasm call error {:?}", e))?;
        return Ok(result);
    }
}
fn decode_result(a: Vec<u8>) -> (u32, Vec<u8>) {
    let mut b = a.clone();
    let c = u32::from_ne_bytes([b[0], b[1], b[2], b[3]]);
    b.drain(0..4);
    (c, b)
}
#[cfg(test)]
mod tests {
    use codec::{Decode, Encode};

    use super::*;

    #[test]
    pub fn load_wasm_should_work() {
        env_logger::init();
        let wasm_path = "../nucleus-examples/vrs_nucleus_examples.wasm";
        let wasm = WasmInfo {
            account: AccountId::new([0u8; 32]),
            name: "avs-dev-demo".to_string(),
            version: 0,
            code: WasmCodeRef::File(wasm_path.to_string()),
        };

        let context = Context::init().unwrap();
        let mut vm = Vm::new_instance(&wasm, context).unwrap();
        let input = <(String, String) as codec::Encode>::encode(&(
            "aaaaaaaaaa".to_string(),
            "bbbbbbbbbb".to_string(),
        ));
        println!("input: {:?}", input);
        let result = vm.call_post("cc", input).unwrap();
        println!("encoded_result: {:?}", result);
        let result =
            <Result<String, String> as codec::Decode>::decode(&mut result.as_slice()).unwrap();
        assert_eq!(result, Ok("abababababababababab".to_string()));

        let input = <String as codec::Encode>::encode(&"aaaaaaaaaa".to_string());
        //jsonrpc: {method: AVS_{AVSNAME}_{METHODNAME}, params: [], id: 1}
        let result = vm.call_post("cc", input);
        println!("result: {:?}", result);

        let input = <(String, String) as codec::Encode>::encode(&(
            "aaaaaaaaaa".to_string(),
            "bbbbbbbbbb".to_string(),
        ));
        let result = vm.call_post("cc", input).unwrap();
        println!("encoded_result: {:?}", result);
        let result =
            <Result<String, String> as codec::Decode>::decode(&mut result.as_slice()).unwrap();
        assert_eq!(result, Ok("abababababababababab".to_string()));

        let result = vm.call_get("get", vec![]).unwrap();
        println!("encoded_result: {:?}", result);
        // assert_eq!(
        //     vec![0u8],
        //     vm.call_post("__nucleus_post_post", encoded_args).unwrap()
        // );
    }
    #[test]
    pub fn test_encode() {
        let a = ();
        let b = ("123".to_string(),);
        let c = ("123".to_string(), 123.to_string());
        env_logger::init();
        let wasm_path = "../nucleus-examples/vrs_nucleus_examples.wasm";
        let wasm = WasmInfo {
            account: AccountId::new([0u8; 32]),
            name: "avs-dev-demo".to_string(),
            version: 0,
            code: WasmCodeRef::File(wasm_path.to_string()),
        };

        let context = Context::init().unwrap();
        let mut vm = Vm::new_instance(&wasm, context).unwrap();

        let result = vm.call_post("i0o0", vec![]).unwrap();
        assert_eq!(result, a.encode());
        let result = vm.call_post("i1o0", b.encode()).unwrap();
        assert_eq!(result, a.encode());
        let result = vm.call_post("i1o1", b.encode()).unwrap();
        assert_eq!(result, b.encode());
    }
    #[test]
    pub fn call_post_should_work_for_general() {
        env_logger::init();
        let wasm_path = "../nucleus-examples/vrs_nucleus_examples.wasm";

        let engine = Engine::default();
        let module = Module::from_file(&engine, wasm_path).unwrap();
        module.exports().for_each(|ty| match ty.ty() {
            ExternType::Func(func) => {
                log::info!("export: {} {}", func.to_string(), ty.name());
            }
            _ => {}
        });
        let mut store = Store::new(&engine, Context::init().unwrap());
        let injects = Context::inject_host_funcs(&mut store);
        let instance = Instance::new(&mut store, &module, &injects).unwrap();
        let memory = instance
            .get_memory(&mut store, "memory")
            .expect("Failed to get memory");

        let input = <(String, String) as codec::Encode>::encode(&(
            "aaaaaaaaaa".to_string(),
            "bbbbbbbbbb".to_string(),
        ));
        let input_len = input.len();
        let ptr = memory.data_size(&store) as i32;
        memory.grow(&mut store, 1).unwrap();
        memory.write(&mut store, ptr as usize, &input[..]).unwrap();

        let call_example: wasmtime::Func = instance
            .get_func(&mut store, "__nucleus_decoded_cc")
            .expect("function not found");
        let mut result = vec![Val::I32(0)];
        call_example
            .call(
                &mut store,
                &[Val::I32(ptr), Val::I32(input_len as i32)],
                &mut result,
            )
            .unwrap();
        let result_ptr = result[0].unwrap_i32() as usize;
        let mut result_len = vec![0u8; 50];
        memory.read(&store, result_ptr, &mut result_len).unwrap();
        log::info!("result_len: {:?}", result_len);
        let mut result_len = vec![0u8; 4];
        memory.read(&store, result_ptr, &mut result_len).unwrap();
        log::info!("result_len: {:?}", result_len);
        //convert result_len to u32
        let result_len =
            u32::from_ne_bytes([result_len[0], result_len[1], result_len[2], result_len[3]]);
        log::info!("result_len: {}", result_len);
        let mut result = vec![0u8; result_len as usize];
        memory.read(&store, result_ptr + 4, &mut result).unwrap();
        log::info!("results: {:?}", result);
        let s = <Result<Vec<u8>, String> as codec::Decode>::decode(&mut result.as_slice())
            .unwrap()
            .unwrap();
        let result = <Result<String, String> as codec::Decode>::decode(&mut s.as_slice()).unwrap();
        assert_eq!(result, Ok("abababababababababab".to_string()));
    }
    #[test]
    pub fn call_post_should_fail_for_general() {
        env_logger::init();
        let wasm_path = "../../target/wasm32-unknown-unknown/debug/vrs_nucleus_examples.wasm";

        let engine = Engine::default();
        let module = Module::from_file(&engine, wasm_path).unwrap();
        module.exports().for_each(|ty| match ty.ty() {
            ExternType::Func(func) => {
                log::info!("export: {} {}", func.to_string(), ty.name());
            }
            _ => {}
        });
        let mut store = Store::new(&engine, Context::init().unwrap());
        let injects = Context::inject_host_funcs(&mut store);
        let instance = Instance::new(&mut store, &module, &injects).unwrap();
        let memory = instance
            .get_memory(&mut store, "memory")
            .expect("Failed to get memory");

        let input = <String as codec::Encode>::encode(&"aaaaaaaaaa".to_string());
        let input_len = input.len();
        let ptr = memory.data_size(&store) as i32;
        memory.grow(&mut store, 1).unwrap();
        memory.write(&mut store, ptr as usize, &input[..]).unwrap();

        let call_example: wasmtime::Func = instance
            .get_func(&mut store, "__nucleus_decoded_cc")
            .expect("function not found");
        let mut result = vec![Val::I32(0)];
        call_example
            .call(
                &mut store,
                &[Val::I32(ptr), Val::I32(input_len as i32)],
                &mut result,
            )
            .unwrap();
        let result_ptr = result[0].unwrap_i32() as usize;
        let mut result_len = vec![0u8; 50];
        memory.read(&store, result_ptr, &mut result_len).unwrap();
        log::info!("result_len: {:?}", result_len);
        let mut result_len = vec![0u8; 4];
        memory.read(&store, result_ptr, &mut result_len).unwrap();
        log::info!("result_len: {:?}", result_len);
        //convert result_len to u32
        let result_len =
            u32::from_ne_bytes([result_len[0], result_len[1], result_len[2], result_len[3]]);
        log::info!("result_len: {}", result_len);
        let mut result = vec![0u8; result_len as usize];
        memory.read(&store, result_ptr + 4, &mut result).unwrap();
        log::info!("results: {:?}", result);
        let s = <Result<Vec<u8>, String> as codec::Decode>::decode(&mut result.as_slice()).unwrap();
        assert_eq!(s, Err("Failed to decode arguments tuple".to_string()));
    }
}
