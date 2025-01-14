use crate::{
    runtime::{ContextAware, FuncRegister},
    WasmInfo,
};
use codec::{Decode, Encode};
use thiserror::Error;
use wasmtime::{Engine, ExternType, Instance, Module, Store, Val};

pub struct Vm<R> {
    space: Store<R>,
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
    #[error("Argument decoding error")]
    DecodeError,
    #[error("Wasm internal error: {0}")]
    WasmInternalError(String),
    #[error("Wasm not initialized")]
    WasmNotInitialized,
}

const PAGE_SIZE: usize = 65536;
const MAX_PAGE_COUNT: usize = 1024;
impl Into<(i32, String)> for WasmCallError {
    fn into(self) -> (i32, String) {
        (self.to_error_code() as i32, self.to_string())
    }
}

impl WasmCallError {
    pub fn to_error_code(&self) -> i32 {
        match self {
            WasmCallError::EndpointNotFound => -5000,
            WasmCallError::NoMemoryExported => -5001,
            WasmCallError::ArgumentsSizeExceeded => -5002,
            WasmCallError::ResultSizeExceeded => -5003,
            WasmCallError::ResultPointerError => -5004,
            WasmCallError::MemoryError(_) => -5005,
            WasmCallError::FunctionCallError(_) => -5006,
            WasmCallError::DecodeError => -5007,
            WasmCallError::WasmInternalError(_) => -5008,
            WasmCallError::WasmNotInitialized => -5009,
        }
    }
}

impl<R> Vm<R>
where
    R: FuncRegister<Runtime = R> + ContextAware + Clone,
{
    pub fn new_instance(wasm: &WasmInfo, runtime: &R) -> anyhow::Result<Self> {
        let engine = Engine::default();
        let module = Module::from_binary(&engine, &wasm.code)?;
        module.exports().for_each(|ty| {
            if let ExternType::Func(func) = ty.ty() {
                log::debug!("user wasm export: {} {}", func.to_string(), ty.name());
            }
        });
        let mut store = Store::new(&engine, runtime.clone());
        let linker = R::register_host_funcs(&engine);
        let instance = linker.instantiate(&mut store, &module).unwrap();
        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or(anyhow::anyhow!("Invalid wasm code: no memory exported"))?;
        let ptr = memory.data_size(&store) as i32;
        memory.grow(&mut store, MAX_PAGE_COUNT as u64).unwrap();
        Ok(Self {
            space: store,
            instance,
            __call_param_ptr: ptr,
        })
    }

    pub fn call_http_callback<T: Encode>(&mut self, args: T) -> Result<(), WasmCallError> {
        self.space.data_mut().set_read_only(false);
        self.call("__nucleus_http_callback", args.encode())
            .inspect_err(|e| {
                log::warn!("fail to invoke inner method, {:?}", e);
                self.space.data().rollback_all_pending_timer();
            })
            .inspect(|_| {
                self.space.data().enqueue_all_pending_timer();
            })?;
        Ok(())
    }

    pub fn call_timer_trigger(&mut self, func: &str, args: Vec<u8>) -> Result<(), WasmCallError> {
        self.space.data_mut().set_read_only(false);
        let func = format!("__nucleus_timer_{}", func);
        self.call(&func, args)
            .inspect(|_| {
                self.space.data().enqueue_all_pending_timer();
            })
            .inspect_err(|e| {
                log::warn!("fail to invoke timer method, {:?}", e);
                self.space.data().rollback_all_pending_timer();
            })?;
        Ok(())
    }

    pub fn call_init(&mut self) {
        self.space.data_mut().set_read_only(false);
        let _ = self
            .call("__nucleus_init", ().encode())
            .inspect(|_| {
                self.space.data().enqueue_all_pending_timer();
            })
            .inspect_err(|e| {
                log::warn!("fail to invoke init method, {:?}", e);
                eprint!("fail to invoke init method, {:?}", e);
                self.space.data().rollback_all_pending_timer();
            });
    }

    pub fn call_get(&mut self, func: &str, args: Vec<u8>) -> Result<Vec<u8>, WasmCallError> {
        self.space.data_mut().set_read_only(true);
        let func = format!("__nucleus_get_{}", func);
        let result = self.call(&func, args);
        result
    }

    pub fn call_post(&mut self, func: &str, args: Vec<u8>) -> Result<Vec<u8>, WasmCallError> {
        self.space.data_mut().set_read_only(false);
        let func = format!("__nucleus_post_{}", func);
        self.call(&func, args)
            .inspect(|_| {
                self.space.data().enqueue_all_pending_timer();
            })
            .inspect_err(|e| {
                log::warn!("fail to invoke post method, {:?}", e);
                self.space.data().rollback_all_pending_timer();
            })
    }

    fn call(&mut self, func_name: &str, args: Vec<u8>) -> Result<Vec<u8>, WasmCallError> {
        let func = self
            .instance
            .get_func(&mut self.space, &func_name)
            .ok_or(WasmCallError::EndpointNotFound)?;
        let memory = self
            .instance
            .get_memory(&mut self.space, "memory")
            .ok_or(WasmCallError::NoMemoryExported)?;

        // TODO move this check on RPC
        if args.len() > PAGE_SIZE * MAX_PAGE_COUNT {
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

        let result_ptr = result[0].i32().ok_or(WasmCallError::ResultPointerError)? as usize;

        // Read result length
        let mut result_len_bytes = [0u8; 4];
        memory
            .read(&mut self.space, result_ptr, &mut result_len_bytes)
            .map_err(|e| WasmCallError::MemoryError(e.to_string()))?;
        let result_len = u32::from_le_bytes(result_len_bytes);

        // Read result data
        let mut result_data = vec![0u8; result_len as usize];
        memory
            .read(&mut self.space, result_ptr + 4, &mut result_data)
            .map_err(|e| WasmCallError::MemoryError(e.to_string()))?;

        // Decode the result, `None` represents that the raw_params couldn't be decoded
        let result = <Option<Vec<u8>> as Decode>::decode(&mut result_data.as_slice())
            .map_err(|_| WasmCallError::DecodeError)?
            .ok_or(WasmCallError::DecodeError)?;

        Ok(result)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use chrono::Duration;
//     use codec::{Decode, Encode};
//     use std::thread::sleep;
//     use temp_dir::TempDir;
//     #[test]
//     pub fn load_wasm_should_work() {
//         let wasm_path = "../../nucleus-examples/vrs_nucleus_examples.wasm";
//         let wasm = WasmInfo {
//             account: AccountId::new([0u8; 32]),
//             name: "avs-dev-demo".to_string(),
//             version: 0,
//             code: WasmCodeRef::File(wasm_path.to_string()),
//         };

//         let tmp_dir = TempDir::new().unwrap();
//         let context = Context::init(ContextConfig {
//             db_path: tmp_dir.child("1").into_boxed_path(),
//         })
//         .unwrap();
//         let mut vm = Vm::new_instance(&wasm, context).unwrap();
//         let input = <(String, String) as codec::Encode>::encode(&(
//             "aaaaaaaaaa".to_string(),
//             "bbbbbbbbbb".to_string(),
//         ));
//         assert_eq!(
//             input,
//             vec![
//                 40, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 40, 98, 98, 98, 98, 98, 98, 98, 98, 98,
//                 98
//             ]
//         );
//         let result = vm.call_post("cc", input, vec![]).unwrap();
//         assert_eq!(
//             result,
//             vec![
//                 0, 80, 97, 98, 97, 98, 97, 98, 97, 98, 97, 98, 97, 98, 97, 98, 97, 98, 97, 98, 97,
//                 98
//             ]
//         );
//         let result =
//             <Result<String, String> as codec::Decode>::decode(&mut result.as_slice()).unwrap();
//         assert_eq!(result, Ok("abababababababababab".to_string()));

//         let input = <String as codec::Encode>::encode(&"aaaaaaaaaa".to_string());
//         //jsonrpc: {method: AVS_{AVSNAME}_{METHODNAME}, params: [], id: 1}
//         let result = vm.call_post("cc", input, vec![]);
//         assert_eq!(
//             result,
//             Err(WasmCallError::WasmInternalError(
//                 "Failed to decode arguments tuple".to_owned()
//             ))
//         );

//         let input = <(String, String) as codec::Encode>::encode(&(
//             "aaaaaaaaaa".to_string(),
//             "bbbbbbbbbb".to_string(),
//         ));
//         let result = vm.call_post("cc", input, vec![]).unwrap();
//         assert_eq!(
//             result,
//             vec![
//                 0, 80, 97, 98, 97, 98, 97, 98, 97, 98, 97, 98, 97, 98, 97, 98, 97, 98, 97, 98, 97,
//                 98
//             ]
//         );
//         let result =
//             <Result<String, String> as codec::Decode>::decode(&mut result.as_slice()).unwrap();
//         assert_eq!(result, Ok("abababababababababab".to_string()));

//         let result = vm.call_get("get", vec![]).unwrap();
//         assert_eq!(result, vec![5, 0, 0, 0]);
//         // assert_eq!(
//         //     vec![0u8],
//         //     vm.call_post("__nucleus_post_post", encoded_args).unwrap()
//         // );
//     }
//     #[test]
//     pub fn test_set_time_delay() {
//         let wasm_path = "../../nucleus-examples/vrs_nucleus_examples.wasm";
//         let wasm = WasmInfo {
//             account: AccountId::new([0u8; 32]),
//             name: "avs-dev-demo".to_string(),
//             version: 0,
//             code: WasmCodeRef::File(wasm_path.to_string()),
//         };

//         let tmp_dir = TempDir::new().unwrap();
//         let context = Context::init(ContextConfig {
//             db_path: tmp_dir.child("1").into_boxed_path(),
//         })
//         .unwrap();
//         let mut vm = Vm::new_instance(&wasm, context).unwrap();

//         let result = vm.call_post("test_set_timer", vec![], vec![]).unwrap();
//         let result = vm
//             .call_get(
//                 "get_data",
//                 <String as codec::Encode>::encode(&"delay".to_string()),
//             )
//             .unwrap();
//         let r = <Result<String, String> as codec::Decode>::decode(&mut result.as_slice()).unwrap();
//         assert_eq!(r.unwrap(), "init");
//         while let Some(timer_entry) = vm.pop_pending_timer() {
//             let mut now = chrono::Utc::now();
//             if timer_entry.timestamp > now {
//                 let duration: Duration = timer_entry.timestamp - now;
//                 sleep(duration.to_std().unwrap());
//             }
//             let vm_result = vm
//                 .call_post(&timer_entry.func_name, timer_entry.func_params, vec![])
//                 .map_err(|e| {
//                     log::error!(
//                         "fail to call post endpoint: {} due to: {:?}",
//                         &timer_entry.func_name,
//                         e
//                     );
//                     (1000000 << 10 + e.to_error_code(), e.to_string())
//                 });
//             let result = vm
//                 .call_get(
//                     "get_data",
//                     <String as codec::Encode>::encode(&"delay".to_string()),
//                 )
//                 .unwrap();
//             let r =
//                 <Result<String, String> as codec::Decode>::decode(&mut result.as_slice()).unwrap();

//             assert_eq!(r.unwrap(), "delay_complete abc 123");
//         }
//     }

//     #[test]
//     pub fn test_should_not_call_put() {
//         let wasm_path = "../../nucleus-examples/vrs_nucleus_examples.wasm";
//         let wasm = WasmInfo {
//             account: AccountId::new([0u8; 32]),
//             name: "avs-dev-demo".to_string(),
//             version: 0,
//             code: WasmCodeRef::File(wasm_path.to_string()),
//         };

//         let tmp_dir = TempDir::new().unwrap();
//         let context = Context::init(ContextConfig {
//             db_path: tmp_dir.child("1").into_boxed_path(),
//         })
//         .unwrap();
//         let mut vm = Vm::new_instance(&wasm, context).unwrap();
//         println!("__call_ptr={}", vm.__call_param_ptr);
//         let result = vm.call_get("should_not_call_put", vec![]).unwrap();
//         let result = <Result<(), String> as codec::Decode>::decode(&mut result.as_slice()).unwrap();
//         println!("{:?}", result);
//         assert_eq!(
//             result,
//             Err("Operation not allowed in GET method".to_string())
//         );
//         let result = vm.call_post("should_call_put", vec![], vec![]).unwrap();
//         let result = <Result<(), String> as codec::Decode>::decode(&mut result.as_slice()).unwrap();
//         assert_eq!(result, Ok(()));
//     }

//     #[test]
//     pub fn test_encode() {
//         let a = ();
//         let b = ("123".to_string(),);
//         let c = ("123".to_string(), 123.to_string());
//         let wasm_path = "../../nucleus-examples/vrs_nucleus_examples.wasm";
//         let wasm = WasmInfo {
//             account: AccountId::new([0u8; 32]),
//             name: "avs-dev-demo".to_string(),
//             version: 0,
//             code: WasmCodeRef::File(wasm_path.to_string()),
//         };

//         let tmp_dir = TempDir::new().unwrap();
//         let context = Context::init(ContextConfig {
//             db_path: tmp_dir.child("2").into_boxed_path(),
//         })
//         .unwrap();
//         let mut vm = Vm::new_instance(&wasm, context).unwrap();

//         let result = vm.call_post("i0o0", vec![], vec![]).unwrap();
//         assert_eq!(result, a.encode());
//         let result = vm.call_post("i1o0", b.encode(), vec![]).unwrap();
//         assert_eq!(result, a.encode());
//         let result = vm.call_post("i1o1", b.encode(), vec![]).unwrap();

//         assert_eq!(result, b.encode());
//     }

//     #[test]
//     pub fn call_post_should_work_for_general() {
//         let wasm_path = "../../nucleus-examples/vrs_nucleus_examples.wasm";

//         let engine = Engine::default();
//         let module = Module::from_file(&engine, wasm_path).unwrap();
//         module.exports().for_each(|ty| match ty.ty() {
//             ExternType::Func(func) => {
//                 log::info!("export: {} {}", func.to_string(), ty.name());
//             }
//             _ => {}
//         });

//         let tmp_dir = TempDir::new().unwrap();
//         let context = Context::init(ContextConfig {
//             db_path: tmp_dir.child("3").into_boxed_path(),
//         })
//         .unwrap();
//         let mut store = Store::new(&engine, context);
//         let linker = Context::inject_host_funcs(&engine);
//         let instance = linker.instantiate(&mut store, &module).unwrap();
//         let memory = instance
//             .get_memory(&mut store, "memory")
//             .expect("Failed to get memory");

//         let input = <(String, String) as codec::Encode>::encode(&(
//             "aaaaaaaaaa".to_string(),
//             "bbbbbbbbbb".to_string(),
//         ));
//         let input_len = input.len();
//         let ptr = memory.data_size(&store) as i32;
//         memory.grow(&mut store, 1).unwrap();
//         memory.write(&mut store, ptr as usize, &input[..]).unwrap();

//         let call_example: wasmtime::Func = instance
//             .get_func(&mut store, "__nucleus_post_cc")
//             .expect("function not found");
//         let mut result = vec![Val::I32(0)];
//         call_example
//             .call(
//                 &mut store,
//                 &[Val::I32(ptr), Val::I32(input_len as i32)],
//                 &mut result,
//             )
//             .unwrap();
//         let result_ptr = result[0].unwrap_i32() as usize;
//         let mut result_len = vec![0u8; 50];
//         memory.read(&store, result_ptr, &mut result_len).unwrap();
//         log::info!("result_len: {:?}", result_len);
//         let mut result_len = vec![0u8; 4];
//         memory.read(&store, result_ptr, &mut result_len).unwrap();
//         log::info!("result_len: {:?}", result_len);
//         //convert result_len to u32
//         let result_len =
//             u32::from_ne_bytes([result_len[0], result_len[1], result_len[2], result_len[3]]);
//         log::info!("result_len: {}", result_len);
//         let mut result = vec![0u8; result_len as usize];
//         memory.read(&store, result_ptr + 4, &mut result).unwrap();
//         log::info!("results: {:?}", result);
//         let s = <Result<Vec<u8>, String> as codec::Decode>::decode(&mut result.as_slice())
//             .unwrap()
//             .unwrap();
//         let result = <Result<String, String> as codec::Decode>::decode(&mut s.as_slice()).unwrap();
//         assert_eq!(result, Ok("abababababababababab".to_string()));
//     }

//     #[test]
//     pub fn call_post_should_fail_for_general() {
//         let wasm_path = "../../nucleus-examples/vrs_nucleus_examples.wasm";

//         let engine = Engine::default();
//         let module = Module::from_file(&engine, wasm_path).unwrap();
//         module.exports().for_each(|ty| match ty.ty() {
//             ExternType::Func(func) => {
//                 log::info!("export: {} {}", func.to_string(), ty.name());
//             }
//             _ => {}
//         });

//         let tmp_dir = TempDir::new().unwrap();
//         let context = Context::init(ContextConfig {
//             db_path: tmp_dir.child("4").into_boxed_path(),
//         })
//         .unwrap();
//         let mut store = Store::new(&engine, context);
//         let linker = Context::inject_host_funcs(&engine);
//         let instance = linker.instantiate(&mut store, &module).unwrap();
//         let memory = instance
//             .get_memory(&mut store, "memory")
//             .expect("Failed to get memory");

//         let input = <String as codec::Encode>::encode(&"aaaaaaaaaa".to_string());
//         let input_len = input.len();
//         let ptr = memory.data_size(&store) as i32;
//         memory.grow(&mut store, 1).unwrap();
//         memory.write(&mut store, ptr as usize, &input[..]).unwrap();

//         let call_example: wasmtime::Func = instance
//             .get_func(&mut store, "__nucleus_post_cc")
//             .expect("function not found");
//         let mut result = vec![Val::I32(0)];
//         call_example
//             .call(
//                 &mut store,
//                 &[Val::I32(ptr), Val::I32(input_len as i32)],
//                 &mut result,
//             )
//             .unwrap();
//         let result_ptr = result[0].unwrap_i32() as usize;
//         let mut result_len = vec![0u8; 50];
//         memory.read(&store, result_ptr, &mut result_len).unwrap();
//         log::info!("result_len: {:?}", result_len);
//         let mut result_len = vec![0u8; 4];
//         memory.read(&store, result_ptr, &mut result_len).unwrap();
//         log::info!("result_len: {:?}", result_len);
//         //convert result_len to u32
//         let result_len =
//             u32::from_ne_bytes([result_len[0], result_len[1], result_len[2], result_len[3]]);
//         log::info!("result_len: {}", result_len);
//         let mut result = vec![0u8; result_len as usize];
//         memory.read(&store, result_ptr + 4, &mut result).unwrap();
//         log::info!("results: {:?}", result);
//         let s = <Result<Vec<u8>, String> as codec::Decode>::decode(&mut result.as_slice()).unwrap();
//         assert_eq!(s, Err("Failed to decode arguments tuple".to_string()));
//     }
//     #[test]
//     pub fn test_not_found() {
//         let wasm_path = "../../nucleus-examples/vrs_nucleus_examples.wasm";
//         let wasm = WasmInfo {
//             account: AccountId::new([0u8; 32]),
//             name: "avs-dev-demo".to_string(),
//             version: 0,
//             code: WasmCodeRef::File(wasm_path.to_string()),
//         };

//         let tmp_dir = TempDir::new().unwrap();
//         let context = Context::init(ContextConfig {
//             db_path: tmp_dir.child("2").into_boxed_path(),
//         })
//         .unwrap();
//         let mut vm = Vm::new_instance(&wasm, context).unwrap();
//         let result = vm.call_post("test_get_not_found", vec![], vec![]).unwrap();
//         let s = <Result<String, String> as codec::Decode>::decode(&mut result.as_slice())
//             .unwrap()
//             .unwrap();
//         assert_eq!(s, "");
//     }
//     #[test]
//     pub fn test_multiple_set_time_delay() {
//         let wasm_path = "../../nucleus-examples/vrs_nucleus_examples.wasm";
//         let wasm = WasmInfo {
//             account: AccountId::new([0u8; 32]),
//             name: "avs-dev-demo".to_string(),
//             version: 0,
//             code: WasmCodeRef::File(wasm_path.to_string()),
//         };

//         let tmp_dir = TempDir::new().unwrap();
//         let context = Context::init(ContextConfig {
//             db_path: tmp_dir.child("1").into_boxed_path(),
//         })
//         .unwrap();
//         let mut vm = Vm::new_instance(&wasm, context).unwrap();

//         let result = vm
//             .call_post("test_set_tree_mod_timer", vec![], vec![])
//             .unwrap();
//         let result = vm
//             .call_get(
//                 "get_data",
//                 <String as codec::Encode>::encode(&"delay".to_string()),
//             )
//             .unwrap();
//         let r = <Result<String, String> as codec::Decode>::decode(&mut result.as_slice()).unwrap();
//         assert_eq!(r.unwrap(), "delay_complete init 0");
//         let mut ord = 0;
//         while let Some(timer_entry) = vm.pop_pending_timer() {
//             ord += 1;
//             // println!("{:?}", timer_entry);
//             let mut now = chrono::Utc::now();
//             if timer_entry.timestamp > now {
//                 let duration: Duration = timer_entry.timestamp - now;
//                 sleep(duration.to_std().unwrap());
//             }
//             let vm_result = vm
//                 .call_post(&timer_entry.func_name, timer_entry.func_params, vec![])
//                 .map_err(|e| {
//                     log::error!(
//                         "fail to call post endpoint: {} due to: {:?}",
//                         &timer_entry.func_name,
//                         e
//                     );
//                     (1000000 << 10 + e.to_error_code(), e.to_string())
//                 });
//             let result = vm
//                 .call_get(
//                     "get_data",
//                     <String as codec::Encode>::encode(&"delay".to_string()),
//                 )
//                 .unwrap();
//             let r = <Result<String, String> as codec::Decode>::decode(&mut result.as_slice())
//                 .unwrap()
//                 .unwrap();
//             assert_eq!(r, format!("delay_complete abc {}", ord));
//         }
//     }
//     #[test]
//     pub fn test_tree_set_time_delay() {
//         let wasm_path = "../../nucleus-examples/vrs_nucleus_examples.wasm";
//         let wasm = WasmInfo {
//             account: AccountId::new([0u8; 32]),
//             name: "avs-dev-demo".to_string(),
//             version: 0,
//             code: WasmCodeRef::File(wasm_path.to_string()),
//         };

//         let tmp_dir = TempDir::new().unwrap();
//         let context = Context::init(ContextConfig {
//             db_path: tmp_dir.child("1").into_boxed_path(),
//         })
//         .unwrap();
//         let mut vm = Vm::new_instance(&wasm, context).unwrap();
//         let input = <(i32, i32) as codec::Encode>::encode(&(1, 0));
//         let result = vm
//             .call_post(
//                 "test_set_perfect_tree_mod_timer",
//                 input,
//                 vec![CallerInfo {
//                     func: "test_tree_set_time_delay".to_string(),
//                     thread_id: 0,
//                     params: vec![],
//                     caller_type: crate::CallerType::Entry,
//                 }],
//             )
//             .unwrap();
//         let result = vm
//             .call_get(
//                 "get_data",
//                 <String as codec::Encode>::encode(&"delay".to_string()),
//             )
//             .unwrap();
//         let r = <Result<String, String> as codec::Decode>::decode(&mut result.as_slice()).unwrap();

//         let mut s = format!("{}", r.unwrap());
//         let mut ord = 0;
//         while let Some(timer_entry) = vm.pop_pending_timer() {
//             ord += 1;

//             // println!("{:#?}", timer_entry);
//             let mut now = chrono::Utc::now();
//             if timer_entry.timestamp > now {
//                 let duration: Duration = timer_entry.timestamp - now;
//                 sleep(duration.to_std().unwrap());
//             }

//             let vm_result = vm
//                 .call_post(
//                     &timer_entry.func_name,
//                     timer_entry.func_params,
//                     timer_entry.caller_infos,
//                 )
//                 .map_err(|e| {
//                     log::error!(
//                         "fail to call post endpoint: {} due to: {:?}",
//                         &timer_entry.func_name,
//                         e
//                     );
//                     (1000000 << 10 + e.to_error_code(), e.to_string())
//                 });

//             let result = vm
//                 .call_get(
//                     "get_data",
//                     <String as codec::Encode>::encode(&"delay".to_string()),
//                 )
//                 .unwrap();
//             let r = <Result<String, String> as codec::Decode>::decode(&mut result.as_slice())
//                 .unwrap()
//                 .unwrap();
//             s = s + &format!("\n{}", r);
//         }
//         let mut nodes: Vec<(u32, u32)> = vec![]; // Start with (1, 0) for "init"

//         nodes.extend(
//             s.lines() // Skip the "init" line
//                 .filter_map(|line| {
//                     let parts: Vec<&str> = line.split_whitespace().collect();
//                     if parts.len() == 5 {
//                         Some((parts[1].parse().ok()?, parts[4].parse().ok()?))
//                     } else {
//                         None
//                     }
//                 }),
//         );
//         let mut right = vec![0; 100];
//         let mut last = 0;
//         for data in nodes {
//             assert!(data.1 >= last);
//             last = data.1;
//             right[data.0 as usize] = data.1
//         }
//         assert!(right[1] == 0);
//         assert!(right[2] == 1);
//         assert!(right[3] == 2);
//         assert!(right[4] == 2);
//         assert!(right[5] == 3);
//         assert!(right[6] == 3);
//         assert!(right[7] == 4);
//         assert!(right[8] == 3);
//         assert!(right[9] == 4);
//         assert!(right[10] == 4);
//         assert!(right[11] == 5);
//         assert!(right[12] == 4);
//         assert!(right[13] == 5);
//         assert!(right[14] == 5);
//         assert!(right[15] == 6);
//     }
// }
