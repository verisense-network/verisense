use crate::{
    runtime::{ContextAware, FuncRegister},
    NucleusError, WasmInfo,
};
use codec::{Decode, Encode};
use thiserror::Error;
use wasmtime::{Engine, ExternType, Instance, Module, Store, Val};

pub struct Vm<R> {
    space: Store<R>,
    instance: Instance,
    __call_param_ptr: i32,
    wasm_info: WasmInfo,
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

impl Into<NucleusError> for WasmCallError {
    fn into(self) -> NucleusError {
        NucleusError {
            code: self.to_error_code(),
            message: self.to_string(),
        }
    }
}

impl WasmCallError {
    pub fn to_error_code(&self) -> i32 {
        match self {
            WasmCallError::EndpointNotFound => -32070,
            WasmCallError::NoMemoryExported => -32071,
            WasmCallError::ArgumentsSizeExceeded => -32072,
            WasmCallError::ResultSizeExceeded => -32073,
            WasmCallError::ResultPointerError => -32074,
            WasmCallError::MemoryError(_) => -32075,
            WasmCallError::FunctionCallError(_) => -32076,
            WasmCallError::DecodeError => -32077,
            WasmCallError::WasmInternalError(_) => -32078,
            WasmCallError::WasmNotInitialized => -32079,
        }
    }
}

// deprecated, this function always returns Ok(())
pub fn validate_wasm_abi(blob: &[u8], spec: &[serde_json::Value]) -> Result<(), NucleusError> {
    // let engine = Engine::default();
    // let module = Module::from_binary(&engine, blob)
    //     .map_err(|e| NucleusError::abi(format!("Invalid WASM code blob: {}", e)))?;
    // let exports = module
    //     .exports()
    //     .filter_map(|e| {
    //         if let ExternType::Func(_func) = e.ty() {
    //             Some(e.name().to_string())
    //         } else {
    //             None
    //         }
    //     })
    //     .map(|name| (name, ()))
    //     .collect::<std::collections::HashMap<String, ()>>();
    // for item in spec {
    //     let func = item.as_object().ok_or({
    //         NucleusError::abi(format!(
    //             "Invalid ABI specification: expected an object, found {}",
    //             item
    //         ))
    //     })?;
    //     let ty = func.get("type").and_then(|t| t.as_str()).ok_or({
    //         NucleusError::abi(format!(
    //             "Invalid ABI specification: missing `type` field near {}",
    //             item
    //         ))
    //     })?;
    //     if ty == "fn" {
    //         let method = func.get("method").and_then(|m| m.as_str()).ok_or({
    //             NucleusError::abi(format!(
    //                 "Invalid ABI specification: missing `method` field near {}",
    //                 item
    //             ))
    //         })?;
    //         let name = func.get("name").and_then(|m| m.as_str()).ok_or({
    //             NucleusError::abi(format!(
    //                 "Invalid ABI specification: missing `name` field near {}",
    //                 item
    //             ))
    //         })?;
    //         let export_name = format!("__nucleus_{}_{}", method, name);
    //         if !exports.contains_key(&export_name) {
    //             return Err(NucleusError::abi(format!(
    //                 "Missing export: {} in WASM code",
    //                 export_name
    //             )));
    //         }
    //     }
    // }
    Ok(())
}

impl<R> Vm<R>
where
    R: FuncRegister<Runtime = R> + Clone + ContextAware,
{
    pub fn new_instance(wasm: &WasmInfo, runtime: &R) -> anyhow::Result<Self> {
        let engine = Engine::default();
        let module = Module::from_binary(&engine, &wasm.code.clone())?;
        module.exports().for_each(|ty| {
            if let ExternType::Func(func) = ty.ty() {
                log::debug!("user wasm export: {} {}", func.to_string(), ty.name());
            }
        });
        let mut store = Store::new(&engine, runtime.clone());
        let linker = R::register_host_funcs(&engine);
        let instance = linker.instantiate(&mut store, &module)?;
        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or(anyhow::anyhow!("Invalid wasm code: no memory exported"))?;
        let ptr = memory.data_size(&store) as i32;
        memory.grow(&mut store, MAX_PAGE_COUNT as u64)?;
        Ok(Self {
            space: store,
            instance,
            __call_param_ptr: ptr,
            wasm_info: wasm.clone(),
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

    pub fn call_abi(&mut self) -> Result<Vec<u8>, WasmCallError> {
        self.space.data_mut().set_read_only(true);
        let result = self.call("__nucleus_abi", Vec::new())?;
        Ok(result)
    }

    pub fn call_init(&mut self) {
        log::info!(
            "call init for nucleus id: {:?}, version: {:?}",
            self.wasm_info.id,
            self.wasm_info.version
        );
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
        if !args.is_empty() {
            memory
                .write(&mut self.space, self.__call_param_ptr as usize, &args)
                .map_err(|e| WasmCallError::MemoryError(e.to_string()))?;
        }
        let params = if args.is_empty() {
            vec![]
        } else {
            vec![
                Val::I32(self.__call_param_ptr as i32),
                Val::I32(args.len() as i32),
            ]
        };
        let mut result = vec![Val::I32(0)];
        // Call the function
        func.call(&mut self.space, &params, &mut result)
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
