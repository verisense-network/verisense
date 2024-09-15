mod kvdb;

use chrono::{DateTime, Utc};
use rocksdb::DB;
use std::collections::BinaryHeap;
use std::{
    cell::{Cell, RefCell},
    cmp::Ordering,
    rc::Rc,
    sync::{Arc, Mutex},
};
use thiserror::Error;
use wasmtime::{Caller, Engine, Extern, Func, FuncType, Linker, Memory, Store, Trap, Val, ValType};

use crate::{CallerInfo, TimerEntry, TimerQueue};

struct Cache {
    key: Vec<u8>,
    value: Vec<u8>,
}

pub struct Context {
    pub(crate) db: Arc<DB>,
    is_get_method: bool,
    caller_infos: Vec<CallerInfo>,
    //todo: cache
    cache: Arc<Option<Cache>>,
    timer: Arc<Mutex<TimerQueue>>,
    // 1. we need runtime storage to read
    // 3. we need http manager
    // 4. we need timer
}

impl Context {
    pub fn init(config: ContextConfig) -> anyhow::Result<Self> {
        Ok(Context {
            db: Arc::new(kvdb::init_rocksdb(config.db_path)?),
            is_get_method: false,
            caller_infos: vec![],
            cache: Arc::new(None),
            timer: Arc::new(Mutex::new(TimerQueue::new())),
        })
    }

    pub fn push_caller_info(&mut self, caller_info: CallerInfo) {
        self.caller_infos.push(caller_info);
    }

    pub fn pop_caller_info(&mut self) -> Option<CallerInfo> {
        self.caller_infos.pop()
    }

    pub fn replace_caller_infos(&mut self, caller_infos: Vec<CallerInfo>) {
        self.caller_infos = caller_infos;
    }

    pub fn is_get_method(&self) -> bool {
        self.is_get_method
    }

    pub fn set_is_get_method(&mut self, value: bool) {
        self.is_get_method = value;
    }

    pub fn pop_timer_entry(&mut self) -> Option<TimerEntry> {
        self.timer.lock().unwrap().pop()
    }

    pub fn fetch_all_timer_entries(&self) -> Vec<TimerEntry> {
        let mut timer = self.timer.lock().unwrap();
        let mut entries = Vec::new();
        while let Some(entry) = timer.pop() {
            entries.push(entry);
        }
        entries
    }

    pub fn push_timer_entry(&self, entry: TimerEntry) {
        self.timer.lock().unwrap().push(entry);
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
        // for set_time
        linker
            .func_new(
                "env",
                "timer_set_delay",
                FuncType::new(
                    &engine,
                    [
                        ValType::I32,
                        ValType::I32,
                        ValType::I32,
                        ValType::I32,
                        ValType::I32,
                    ],
                    [ValType::I32],
                ),
                |mut caller: Caller<'_, Context>, params, results| {
                    if caller.data().is_get_method() {
                        results[0] = Val::I32(1);
                        return Ok(());
                    }
                    if let [Val::I32(delay), Val::I32(func_ptr), Val::I32(func_len), Val::I32(params_ptr), Val::I32(params_len)] = params {
                        results[0] = Val::I32(3);
                        let func_params = Context::read_bytes_from_memory(&mut caller, *params_ptr, *params_len)?;
                        let func_name = Context::read_bytes_from_memory(&mut caller, *func_ptr, *func_len)?;
                        let timestamp = Utc::now() + std::time::Duration::from_secs(*delay as u64);

                        let entry = TimerEntry {
                            timestamp,
                            func_name:String::from_utf8(func_name)?,
                            triggered_time:None,
                            caller_infos: caller.data().caller_infos.clone(),
                            func_params,
                        };
                        let mut timer = caller.data().timer.lock().unwrap();
                        timer.push(entry);
                        results[0] = Val::I32(0);
                    } else {
                        results[0] = Val::I32(2);
                    }
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

    pub(crate) fn read_bytes_from_memory(
        caller: &mut Caller<'_, Context>,
        ptr: i32,
        len: i32,
    ) -> Result<Vec<u8>, Trap> {
        let mem = match caller.get_export("memory") {
            Some(Extern::Memory(mem)) => Ok(mem),
            _ => Err(Trap::HeapMisaligned),
        }?;
        // Boundary check
        if (ptr as u64 + len as u64) > mem.data_size(&caller) as u64 {
            return Err(Trap::MemoryOutOfBounds);
        }
        let data = mem.data(&caller)[ptr as usize..(ptr + len) as usize].to_vec();

        Ok(data)
    }
}

#[derive(Clone, Debug)]
pub struct ContextConfig {
    pub db_path: Box<std::path::Path>,
}
