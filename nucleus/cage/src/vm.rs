use crate::{
    runtime::{ComponentProvider, ContextAware, FuncRegister},
    CallerInfo, TimerEntry, WasmCodeRef, WasmInfo,
};
use codec::Decode;
use std::sync::atomic::AtomicU64;
use thiserror::Error;
use wasmtime::{Engine, ExternType, Instance, Module, Store, Val, WasmResults};

pub struct Vm<R> {
    space: Store<R>,
    instance: Instance,
    __call_param_ptr: i32,
    // __host_func_param_ptr: i32,
}

pub const MAX_PARAM_SIZE: usize = 65536;

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
use std::sync::atomic::Ordering;
static THREAD_COUNTER: AtomicU64 = AtomicU64::new(0);

thread_local! {
    static THREAD_ID: u64 = THREAD_COUNTER.fetch_add(1, Ordering::SeqCst);
}

fn get_thread_id() -> u64 {
    THREAD_ID.with(|&id| id)
}

impl<R> Vm<R>
where
    R: FuncRegister<Runtime = R> + ContextAware,
{
    pub fn new_instance(wasm: &WasmInfo, runtime: R) -> anyhow::Result<Self> {
        let engine = Engine::default();
        let module = match wasm.code {
            WasmCodeRef::Blob(ref blob) => Module::from_binary(&engine, blob)?,
            WasmCodeRef::File(ref path) => Module::from_file(&engine, path)?,
        };
        module.exports().for_each(|ty| match ty.ty() {
            ExternType::Func(func) => {
                log::info!("user wasm export: {} {}", func.to_string(), ty.name());
            }
            _ => {}
        });
        let mut store = Store::new(&engine, runtime);
        let linker = R::register_host_funcs(&engine);
        let instance = linker.instantiate(&mut store, &module).unwrap();
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

    pub fn call_inner<T: codec::Encode>(
        &mut self,
        func: &str,
        args: T,
    ) -> Result<(), WasmCallError> {
        let result = self.call_method(&func, args.encode(), false);
        result.map(|_| ())
    }

    // TODO we need to re-degisn the caller context(the previous context is now called runtime)
    pub fn call_get(&mut self, func: &str, args: Vec<u8>) -> Result<Vec<u8>, WasmCallError> {
        // self.space.data_mut().set_is_get_method(true);
        // self.space.data_mut().push_caller_info(CallerInfo {
        //     func: func.to_string(),
        //     params: args.clone(),
        //     thread_id: get_thread_id(),
        //     caller_type: crate::CallerType::Get,
        // });
        let func = format!("__nucleus_get_{}", func);
        let result = self.call_method(&func, args, true);
        // self.space.data_mut().set_is_get_method(false);
        // self.space.data_mut().pop_caller_info();
        result
    }

    pub fn call_timer(
        &mut self,
        func: &str,
        args: Vec<u8>,
        caller_infos: Vec<CallerInfo>,
    ) -> Result<(Vec<u8>, Vec<TimerEntry>), WasmCallError> {
        let func = format!("__nucleus_timer_{}", func);
        let result = self.call_method(&func, args, false)?;
        let timer = self.space.data().pop_all_pending_timer();
        Ok((result, timer))
    }

    // TODO we need to re-degisn the caller context(the previous context is now called runtime)
    pub fn call_post(&mut self, func: &str, args: Vec<u8>) -> Result<Vec<u8>, WasmCallError> {
        // self.space.data_mut().set_is_get_method(false);
        // let new_caller_infos = vec![
        //     caller_infos,
        //     vec![CallerInfo {
        //         func: func.to_string(),
        //         params: args.clone(),
        //         thread_id: get_thread_id(),
        //         caller_type: crate::CallerType::Post,
        //     }],
        // ]
        // .concat();
        // self.space.data_mut().replace_caller_infos(new_caller_infos);
        let func = format!("__nucleus_post_{}", func);
        let result = self.call_method(&func, args, false);
        // self.space.data_mut().pop_caller_info();
        return result;
    }

    fn call_method(
        &mut self,
        func_name: &str,
        args: Vec<u8>,
        is_get: bool,
    ) -> Result<Vec<u8>, WasmCallError> {
        let func = self
            .instance
            .get_func(&mut self.space, &func_name)
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

        let result_ptr = result[0].i32().ok_or(WasmCallError::ResultPointerError)? as usize;

        // Read result length
        let mut result_len_bytes = [0u8; 4];
        memory
            .read(&mut self.space, result_ptr, &mut result_len_bytes)
            .map_err(|e| WasmCallError::MemoryError(e.to_string()))?;
        // println!("{:?}", result_len_bytes);
        let result_len = u32::from_le_bytes(result_len_bytes);

        // if result_len > 65536 * 128 {
        //     return Err(WasmCallError::ResultSizeExceeded);
        // }

        // Read result data
        let mut result_data = vec![0u8; result_len as usize];
        memory
            .read(&mut self.space, result_ptr + 4, &mut result_data)
            .map_err(|e| WasmCallError::MemoryError(e.to_string()))?;

        // Decode the result
        let result = <Result<Vec<u8>, String> as Decode>::decode(&mut result_data.as_slice())?
            .map_err(|e| WasmCallError::WasmInternalError(e))?;

        Ok(result)
    }

    // TODO we need to re-degisn the caller context(the previous context is now called runtime)
    // pub fn pop_pending_timer(&mut self) -> Option<TimerEntry> {
    //     self.space.data_mut().pop_timer_entry()
    // }
}

fn decode_result(a: Vec<u8>) -> (u32, Vec<u8>) {
    let mut b = a.clone();
    let c = u32::from_ne_bytes([b[0], b[1], b[2], b[3]]);
    b.drain(0..4);
    (c, b)
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
//     pub fn test_put_get_dynamic() {
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
//         let result = vm.call_post("test_put_get", vec![], vec![]).unwrap();
//         let s = <Result<String, String> as codec::Decode>::decode(&mut result.as_slice())
//             .unwrap()
//             .unwrap();
//         assert_eq!(s, "1".repeat(65536 * 16));
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
//     pub fn test_put_get_static() {
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
//         let result = vm.call_post("test_put_get_static", vec![], vec![]).unwrap();
//         let s = <Result<String, String> as codec::Decode>::decode(&mut result.as_slice())
//             .unwrap()
//             .unwrap();
//         assert_eq!(s, "test_value");
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
