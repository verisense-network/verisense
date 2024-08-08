use crate::{
    context::Context,
    wasm_code::{WasmCodeRef, WasmInfo},
};
use anyhow::anyhow;
use codec::Decode;
use sp_runtime::traits::Member;
use thiserror::Error;
use vrs_core_sdk::AccountId;
use wasmtime::{Caller, Func, Memory, Trap, Val};
use wasmtime::{Engine, ExternRef, ExternType, Instance, Module, Rooted, Store, WasmResults};
pub struct Vm {
    space: Store<Context>,
    instance: Instance,
    __call_param_ptr: i32,
}
#[derive(Error, Debug, PartialEq)]
pub enum WasmCallError {
    #[error("Endpoint not found")]
    EndpointNotFound,

    #[error("No memory exported")]
    NoMemoryExported,

    #[error("Arguments size exceeds 64KB limit")]
    ArgumentsSizeExceeded,

    #[error("Result size exceeds 64KB limit")]
    ResultSizeExceeded,

    #[error("Result pointer error")]
    ResultPointerError,

    #[error("Memory error: {0}")]
    MemoryError(String),

    #[error("Function call error: {0}")]
    FunctionCallError(String),

    #[error("Decode error: {0}")]
    DecodeError(#[from] codec::Error),

    #[error("Wasm internal error: {0}")]
    WasmInternalError(String),
}
impl WasmCallError {
    pub fn to_error_code(&self) -> u32 {
        match self {
            WasmCallError::EndpointNotFound => 0,
            WasmCallError::NoMemoryExported => 1,
            WasmCallError::ArgumentsSizeExceeded => 2,
            WasmCallError::ResultSizeExceeded => 3,
            WasmCallError::ResultPointerError => 4,
            WasmCallError::MemoryError(_) => 5,
            WasmCallError::FunctionCallError(_) => 6,
            WasmCallError::DecodeError(_) => 7,
            WasmCallError::WasmInternalError(_) => 8,
        }
    }
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

    pub fn call_get(&mut self, func: &str, args: Vec<u8>) -> Result<Vec<u8>, WasmCallError> {
        let func = self
            .instance
            .get_func(&mut self.space, &format!("__nucleus_get_{}", func))
            .ok_or(WasmCallError::EndpointNotFound)?;

        let memory = self
            .instance
            .get_memory(&mut self.space, "memory")
            .ok_or(WasmCallError::NoMemoryExported)?;

        if args.len() > 65536 {
            return Err(WasmCallError::ArgumentsSizeExceeded);
        }

        // Write args to memory
        memory
            .write(&mut self.space, self.__call_param_ptr as usize, &args)
            .map_err(|e| WasmCallError::MemoryError(e.to_string()))?;

        let mut result = vec![Val::I32(0)];
        // Call the function
        func.call(
            &mut self.space,
            &[
                Val::I32(self.__call_param_ptr as i32),
                Val::I32(args.len() as i32),
            ],
            &mut result,
        )
        .map_err(|e| WasmCallError::FunctionCallError(e.to_string()))?;

        log::info!("results: {:?}", result);

        let result_ptr = result[0].i32().ok_or(WasmCallError::ResultPointerError)? as usize;

        // Read result length
        let mut result_len_bytes = [0u8; 4];
        memory
            .read(&mut self.space, result_ptr, &mut result_len_bytes)
            .map_err(|e| WasmCallError::MemoryError(e.to_string()))?;
        let result_len = u32::from_le_bytes(result_len_bytes);

        if result_len > 65536 {
            return Err(WasmCallError::ResultSizeExceeded);
        }

        // Read result data
        let mut result_data = vec![0u8; result_len as usize];
        memory
            .read(&mut self.space, result_ptr + 4, &mut result_data)
            .map_err(|e| WasmCallError::MemoryError(e.to_string()))?;

        log::debug!("result {:?}", result_data);

        // Decode the result
        let result = <Result<Vec<u8>, String> as Decode>::decode(&mut result_data.as_slice())?
            .map_err(|e| WasmCallError::WasmInternalError(e))?;

        Ok(result)
    }

    pub fn call_post(&mut self, func: &str, args: Vec<u8>) -> Result<Vec<u8>, WasmCallError> {
        let func = self
            .instance
            .get_func(&mut self.space, &format!("__nucleus_post_{}", func))
            .ok_or(WasmCallError::EndpointNotFound)?;

        let memory = self
            .instance
            .get_memory(&mut self.space, "memory")
            .ok_or(WasmCallError::NoMemoryExported)?;

        if args.len() > 65536 {
            return Err(WasmCallError::ArgumentsSizeExceeded);
        }

        // Write args to memory
        memory
            .write(&mut self.space, self.__call_param_ptr as usize, &args)
            .map_err(|e| WasmCallError::MemoryError(e.to_string()))?;

        let mut result = vec![Val::I32(0)];
        // Call the function
        func.call(
            &mut self.space,
            &[
                Val::I32(self.__call_param_ptr as i32),
                Val::I32(args.len() as i32),
            ],
            &mut result,
        )
        .map_err(|e| WasmCallError::FunctionCallError(e.to_string()))?;

        log::info!("results: {:?}", result);

        let result_ptr = result[0].i32().ok_or(WasmCallError::ResultPointerError)? as usize;

        // Read result length
        let mut result_len_bytes = [0u8; 4];
        memory
            .read(&mut self.space, result_ptr, &mut result_len_bytes)
            .map_err(|e| WasmCallError::MemoryError(e.to_string()))?;
        let result_len = u32::from_le_bytes(result_len_bytes);

        if result_len > 65536 {
            return Err(WasmCallError::ResultSizeExceeded);
        }

        // Read result data
        let mut result_data = vec![0u8; result_len as usize];
        memory
            .read(&mut self.space, result_ptr + 4, &mut result_data)
            .map_err(|e| WasmCallError::MemoryError(e.to_string()))?;

        log::debug!("result {:?}", result_data);

        // Decode the result
        let result = <Result<Vec<u8>, String> as Decode>::decode(&mut result_data.as_slice())?
            .map_err(|e| WasmCallError::WasmInternalError(e))?;

        Ok(result)
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

        let context = Context::init("/tmp").unwrap();
        let mut vm = Vm::new_instance(&wasm, context).unwrap();
        let input = <(String, String) as codec::Encode>::encode(&(
            "aaaaaaaaaa".to_string(),
            "bbbbbbbbbb".to_string(),
        ));
        assert_eq!(
            input,
            vec![
                40, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 40, 98, 98, 98, 98, 98, 98, 98, 98, 98,
                98
            ]
        );
        let result = vm.call_post("cc", input).unwrap();
        assert_eq!(
            result,
            vec![
                0, 80, 97, 98, 97, 98, 97, 98, 97, 98, 97, 98, 97, 98, 97, 98, 97, 98, 97, 98, 97,
                98
            ]
        );
        let result =
            <Result<String, String> as codec::Decode>::decode(&mut result.as_slice()).unwrap();
        assert_eq!(result, Ok("abababababababababab".to_string()));

        let input = <String as codec::Encode>::encode(&"aaaaaaaaaa".to_string());
        //jsonrpc: {method: AVS_{AVSNAME}_{METHODNAME}, params: [], id: 1}
        let result = vm.call_post("cc", input);
        assert_eq!(
            result,
            Err(WasmCallError::WasmInternalError(
                "Failed to decode arguments tuple".to_owned()
            ))
        );

        let input = <(String, String) as codec::Encode>::encode(&(
            "aaaaaaaaaa".to_string(),
            "bbbbbbbbbb".to_string(),
        ));
        let result = vm.call_post("cc", input).unwrap();
        assert_eq!(
            result,
            vec![
                0, 80, 97, 98, 97, 98, 97, 98, 97, 98, 97, 98, 97, 98, 97, 98, 97, 98, 97, 98, 97,
                98
            ]
        );
        let result =
            <Result<String, String> as codec::Decode>::decode(&mut result.as_slice()).unwrap();
        assert_eq!(result, Ok("abababababababababab".to_string()));

        let result = vm.call_get("get", vec![]).unwrap();
        assert_eq!(result, vec![5, 0, 0, 0]);
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

        let context = Context::init("/tmp").unwrap();
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
        let mut store = Store::new(&engine, Context::init("/tmp").unwrap());
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
            .get_func(&mut store, "__nucleus_post_cc")
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
        let mut store = Store::new(&engine, Context::init("/tmp").unwrap());
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
