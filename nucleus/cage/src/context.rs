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
    //todo: cache
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
                |mut caller: Caller<'_, Context>, params, results| {
                    results[0] = Val::I32(0);
                    if caller.data().is_get_method() {
                        results[0] = Val::I32(1);
                        return Ok(());
                    }
                    let mem = match Context::wasm_mem(&mut caller) {
                        Ok(mem) => mem,
                        Err(_) => {
                            results[0] = Val::I32(2);
                            return Ok(());
                        } // Error code for memory access failure
                    };
                    let k_ptr = params[0].unwrap_i32();
                    let k_len = params[1].unwrap_i32();
                    let v_ptr = params[2].unwrap_i32();
                    let v_len = params[3].unwrap_i32();

                    // Boundary check
                    if (k_ptr as u64 + k_len as u64) > mem.data_size(&caller) as u64
                        || (v_ptr as u64 + v_len as u64) > mem.data_size(&caller) as u64
                    {
                        results[0] = Val::I32(2);
                        return Ok(());
                    }

                    let key = mem.data(&caller)[k_ptr as usize..(k_ptr + k_len) as usize].to_vec();
                    let val = mem.data(&caller)[v_ptr as usize..(v_ptr + v_len) as usize].to_vec();

                    // println!(
                    //     "Storing split key={}, val={}",
                    //     String::from_utf8_lossy(&key),
                    //     String::from_utf8_lossy(&val)
                    // );

                    if let Err(e) = kvdb::storage_put_db(&caller.data().db, &key, &val) {
                        log::error!("Database error: {}", e);
                        results[0] = Val::I32(3);
                        return Ok(());
                    }
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
                |mut caller: Caller<'_, Context>, params, results| {
                    results[0] = Val::I32(0);
                    let mem = match Context::wasm_mem(&mut caller) {
                        Ok(mem) => mem,
                        Err(_) => {
                            results[0] = Val::I32(2);
                            return Ok(());
                        } // Error code for memory access failure
                    };
                    let k_ptr = params[0].unwrap_i32();
                    let k_len = params[1].unwrap_i32();
                    let v_ptr = params[2].unwrap_i32();
                    let v_len_ptr = params[3].unwrap_i32();

                    // Boundary check
                    if (k_ptr as u64 + k_len as u64) > mem.data_size(&caller) as u64 {
                        results[0] = Val::I32(2);
                        return Ok(());
                    }
                    let key = mem.data(&caller)[k_ptr as usize..(k_ptr + k_len) as usize].to_vec();
                    let val = kvdb::storage_get_db(&caller.data().db, &key);
                    match val {
                        Ok(Some(v)) => {
                            if mem.write(&mut caller, v_ptr as usize, &v).is_err() {
                                results[0] = Val::I32(2);
                                return Ok(());
                            }
                            // println!("storage_get key={:?}, val={:?}", key, v);
                            if mem
                                .write(
                                    &mut caller,
                                    v_len_ptr as usize,
                                    &(v.len() as i32).to_le_bytes(),
                                )
                                .is_err()
                            {
                                results[0] = Val::I32(2);
                                return Ok(());
                            }
                        }
                        Ok(None) => {
                            results[0] = Val::I32(4);
                            return Ok(());
                        }
                        Err(e) => {
                            log::error!("storage error {}", e);
                            results[0] = Val::I32(3);
                        }
                    }
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
                    results[0] = Val::I32(0);
                    let mem = match Context::wasm_mem(&mut caller) {
                        Ok(mem) => mem,
                        Err(_) => {
                            results[0] = Val::I32(2);
                            return Ok(());
                        } // Error code for memory access failure
                    };
                    let k_ptr = params[0].unwrap_i32();
                    let k_len = params[1].unwrap_i32();
                    let v_len_ptr = params[2].unwrap_i32();

                    // Boundary check
                    if (k_ptr as u64 + k_len as u64) > mem.data_size(&caller) as u64 {
                        results[0] = Val::I32(2);
                        return Ok(());
                    }
                    let key = mem.data(&caller)[k_ptr as usize..(k_ptr + k_len) as usize].to_vec();
                    let val = kvdb::storage_get_db(&caller.data().db, &key);
                    match val {
                        Ok(Some(v)) => {
                            if mem
                                .write(
                                    &mut caller,
                                    v_len_ptr as usize,
                                    &(v.len() as i32).to_le_bytes(),
                                )
                                .is_err()
                            {
                                results[0] = Val::I32(2);
                                return Ok(());
                            }
                        }
                        Ok(None) => {
                            results[0] = Val::I32(4);
                            return Ok(());
                        }
                        Err(e) => {
                            log::error!("storage error {}", e);
                            results[0] = Val::I32(3);
                        }
                    }
                    Ok(())
                },
            )
            .unwrap();
        // linker
        //     .func_new(
        //         "env",
        //         "storage_get_data",
        //         FuncType::new(
        //             &engine,
        //             [ValType::I32, ValType::I32, ValType::I32, ValType::I32],
        //             [ValType::I32],
        //         ),
        //         |mut caller: Caller<'_, Context>, params, results| {
        //             let key_ptr = params[0].unwrap_i32();
        //             let key_len = params[1].unwrap_i32();
        //             let value_ptr = params[2].unwrap_i32();
        //             let value_len_ptr = params[3].unwrap_i32();

        //             // Get memory
        //             let memory = match caller.get_export("memory").and_then(|e| e.into_memory()) {
        //                 Some(mem) => mem,
        //                 None => {
        //                     results[0] = Val::I32(3); // Error code for memory export not found
        //                     return Ok(());
        //                 }
        //             };

        //             // Read the key from WebAssembly memory
        //             let mut key = vec![0u8; key_len as usize];
        //             if memory
        //                 .read(&mut caller, key_ptr as usize, &mut key)
        //                 .is_err()
        //             {
        //                 results[0] = Val::I32(4); // Error code for memory read failure
        //                 return Ok(());
        //             }
        //             // Check if the data is in cache
        //             let ca = caller.data().cache.clone();
        //             let status = if let Some(cache) = &*ca {
        //                 if cache.key == key {
        //                     // Use cached data
        //                     let value_len = cache.value.len() as u32;

        //                     // Write value length
        //                     if memory
        //                         .write(
        //                             &mut caller,
        //                             value_len_ptr as usize,
        //                             &value_len.to_le_bytes(),
        //                         )
        //                         .is_err()
        //                     {
        //                         results[0] = Val::I32(5); // Error code for memory write failure
        //                         return Ok(());
        //                     }

        //                     // Write value data
        //                     if memory
        //                         .write(&mut caller, value_ptr as usize, &cache.value)
        //                         .is_err()
        //                     {
        //                         results[0] = Val::I32(5); // Error code for memory write failure
        //                         return Ok(());
        //                     }

        //                     0 // Success status
        //                 } else {
        //                     // Key not in cache, proceed with normal flow
        //                     kvdb::storage_get_split(
        //                         caller,
        //                         key_ptr,
        //                         key_len,
        //                         value_ptr,
        //                         value_len_ptr,
        //                     )
        //                 }
        //             } else {
        //                 // No cache, proceed with normal flow
        //                 kvdb::storage_get_split(caller, key_ptr, key_len, value_ptr, value_len_ptr)
        //             };

        //             results[0] = Val::I32(status);
        //             Ok(())
        //         },
        //     )
        //     .unwrap();
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
