mod kvdb;

use std::cell::{Cell, RefCell};

use rocksdb::DB;
use std::sync::Arc;
use thiserror::Error;
use wasmtime::{Caller, Engine, Extern, Func, FuncType, Linker, Memory, Store, Trap, Val, ValType};

struct Cache {
    key: Vec<u8>,
    value: Vec<u8>,
}

pub struct Context {
    pub(crate) db: Arc<DB>,
    is_get_method: bool,
    cache: Arc<Option<Cache>>,
    // 1. we need runtime storage to read
    // 3. we need http manager
    // 4. we need timer
}
// pub enum StorageError {
//     CannotPutInGetMethod = 1,
//     MemoryAccessOutOfBounds = 2,
//     DatabaseError = 3,
//     KeyNotFound = 4,
//     UnknownError,
// }
impl Context {
    pub fn init(config: ContextConfig) -> anyhow::Result<Self> {
        Ok(Context {
            db: Arc::new(kvdb::init_rocksdb(config.db_path)?),
            is_get_method: false,
            cache: Arc::new(None),
        })
    }

    pub fn is_get_method(&self) -> bool {
        self.is_get_method
    }

    pub fn set_is_get_method(&mut self, value: bool) {
        self.is_get_method = value;
    }

    pub(crate) fn inject_host_funcs(engine: &Engine) -> Linker<Context> {
        let mut linker = Linker::new(engine);

        linker
            .func_new(
                "env",
                "storage_put",
                FuncType::new(
                    &engine,
                    [ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                    [ValType::I32],
                ),
                |caller: Caller<'_, Context>, params, results| {
                    let result = kvdb::storage_put_split(
                        caller,
                        params[0].unwrap_i32(),
                        params[1].unwrap_i32(),
                        params[2].unwrap_i32(),
                        params[3].unwrap_i32(),
                    );
                    results[0] = Val::I32(result);
                    Ok(())
                },
            )
            .unwrap();
        // for static
        linker
            .func_new(
                "env",
                "storage_get",
                FuncType::new(
                    &engine,
                    [ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                    [ValType::I32],
                ),
                |caller: Caller<'_, Context>, params, results| {
                    let result = kvdb::storage_get_split(
                        caller,
                        params[0].unwrap_i32(),
                        params[1].unwrap_i32(),
                        params[2].unwrap_i32(),
                        params[3].unwrap_i32(),
                    );
                    results[0] = Val::I32(result);
                    Ok(())
                },
            )
            .unwrap();
        //for dynamic
        linker
            .func_new(
                "env",
                "storage_get_len",
                FuncType::new(
                    &engine,
                    [ValType::I32, ValType::I32, ValType::I32],
                    [ValType::I32],
                ),
                |mut caller: Caller<'_, Context>, params, results| {
                    let result = kvdb::storage_get_split_len(
                        caller,
                        params[0].unwrap_i32(),
                        params[1].unwrap_i32(),
                        params[2].unwrap_i32(),
                    );
                    let result = match result {
                        Ok(_) => 0,
                        Err(e) => e,
                    };
                    results[0] = Val::I32(result);
                    Ok(())
                },
            )
            .unwrap();
        linker
            .func_new(
                "env",
                "storage_get_data",
                FuncType::new(
                    &engine,
                    [ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                    [ValType::I32],
                ),
                |mut caller: Caller<'_, Context>, params, results| {
                    let key_ptr = params[0].unwrap_i32();
                    let key_len = params[1].unwrap_i32();
                    let value_ptr = params[2].unwrap_i32();
                    let value_len_ptr = params[3].unwrap_i32();

                    // Get memory
                    let memory = match caller.get_export("memory").and_then(|e| e.into_memory()) {
                        Some(mem) => mem,
                        None => {
                            results[0] = Val::I32(3); // Error code for memory export not found
                            return Ok(());
                        }
                    };

                    // Read the key from WebAssembly memory
                    let mut key = vec![0u8; key_len as usize];
                    if memory
                        .read(&mut caller, key_ptr as usize, &mut key)
                        .is_err()
                    {
                        results[0] = Val::I32(4); // Error code for memory read failure
                        return Ok(());
                    }
                    // Check if the data is in cache
                    let ca = caller.data().cache.clone();
                    let status = if let Some(cache) = &*ca {
                        if cache.key == key {
                            // Use cached data
                            let value_len = cache.value.len() as u32;

                            // Write value length
                            if memory
                                .write(
                                    &mut caller,
                                    value_len_ptr as usize,
                                    &value_len.to_le_bytes(),
                                )
                                .is_err()
                            {
                                results[0] = Val::I32(5); // Error code for memory write failure
                                return Ok(());
                            }

                            // Write value data
                            if memory
                                .write(&mut caller, value_ptr as usize, &cache.value)
                                .is_err()
                            {
                                results[0] = Val::I32(5); // Error code for memory write failure
                                return Ok(());
                            }

                            0 // Success status
                        } else {
                            // Key not in cache, proceed with normal flow
                            kvdb::storage_get_split(
                                caller,
                                key_ptr,
                                key_len,
                                value_ptr,
                                value_len_ptr,
                            )
                        }
                    } else {
                        // No cache, proceed with normal flow
                        kvdb::storage_get_split(caller, key_ptr, key_len, value_ptr, value_len_ptr)
                    };

                    results[0] = Val::I32(status);
                    Ok(())
                },
            )
            .unwrap();
        linker
    }

    pub(crate) fn wasm_mem(caller: &mut Caller<'_, Context>) -> Result<Memory, Trap> {
        match caller.get_export("memory") {
            Some(Extern::Memory(mem)) => Ok(mem),
            _ => Err(Trap::HeapMisaligned),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ContextConfig {
    pub db_path: Box<std::path::Path>,
}
