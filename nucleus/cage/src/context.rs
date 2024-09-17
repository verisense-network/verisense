pub mod http;
pub mod kvdb;

use crate::{CallerInfo, TimerEntry, TimerQueue};
use chrono::{DateTime, Utc};
use rocksdb::DB;
use std::sync::{Arc, Mutex};
use vrs_core_sdk::{BUFFER_LEN, NO_MORE_DATA};
use vrs_primitives::NucleusId;
use wasmtime::{Caller, Engine, Extern, FuncType, Linker, Memory, Trap, Val, ValType};

#[derive(Clone, Debug)]
pub struct ContextConfig {
    pub db_path: Box<std::path::Path>,
}

pub struct Context {
    pub(crate) id: NucleusId,
    pub(crate) db: Arc<DB>,
    pub(crate) http: Arc<http::HttpCallRegister>,
    is_get_method: bool,
    caller_infos: Vec<CallerInfo>,
    timer: Arc<Mutex<TimerQueue>>,
    // 1. we need runtime storage to read
    // 3. we need http manager
    // 4. we need timer
}

impl Context {
    pub fn init(
        id: NucleusId,
        http_register: Arc<http::HttpCallRegister>,
        config: ContextConfig,
    ) -> anyhow::Result<Self> {
        Ok(Context {
            id,
            db: Arc::new(kvdb::init_rocksdb(config.db_path)?),
            http: http_register,
            is_get_method: false,
            caller_infos: vec![],
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
                kvdb::storage_put_signature(engine),
                kvdb::storage_put,
            )
            .unwrap();
        linker
            .func_new(
                "env",
                "storage_get",
                kvdb::storage_get_signature(engine),
                kvdb::storage_get,
            )
            .unwrap();
        linker
            .func_new(
                "env",
                "storage_del",
                kvdb::storage_del_signature(engine),
                kvdb::storage_delete,
            )
            .unwrap();
        linker
            .func_new(
                "env",
                "http_request",
                http::http_request_signature(engine),
                http::enqueue_http_request,
            )
            .unwrap();
        // TODO for set_timer
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

    // this might panic the whole nucleus thread, maybe we should check it at startup
    pub(crate) fn wasm_mem(caller: &mut Caller<'_, Context>) -> Memory {
        match caller.get_export("memory") {
            Some(Extern::Memory(mem)) => mem,
            _ => panic!("Invalid WASM: no memory exported"),
        }
    }

    pub(crate) fn read_bytes_from_memory(
        caller: &mut Caller<'_, Context>,
        ptr: i32,
        len: i32,
    ) -> Result<Vec<u8>, Trap> {
        let mem = Self::wasm_mem(caller);
        // Boundary check
        // if (ptr as u64 + len as u64) > mem.data_size(&caller) as u64 {
        //     return Err(Trap::MemoryOutOfBounds);
        // }
        let data = mem.data(&caller)[ptr as usize..(ptr + len) as usize].to_vec();
        Ok(data)
    }

    pub(crate) fn write_bytes_to_memory(
        caller: &mut Caller<'_, Context>,
        ptr: i32,
        data: &[u8],
    ) -> Result<(), Trap> {
        let mem = Self::wasm_mem(caller);
        mem.write(caller, ptr as usize, data)
            .map_err(|_| Trap::MemoryOutOfBounds)
    }

    pub(crate) fn write_to_memory<T: codec::Encode>(
        caller: &mut Caller<'_, Context>,
        ptr: i32,
        data: T,
        offset: Option<i32>,
    ) -> Result<Val, Trap> {
        let bytes = data.encode();
        if bytes.len() > BUFFER_LEN {
            assert!(offset.is_some());
            let offset = offset.unwrap() as usize;
            if offset >= bytes.len() {
                return Err(Trap::MemoryOutOfBounds);
            } else if offset + BUFFER_LEN >= bytes.len() {
                Self::write_bytes_to_memory(caller, ptr, &bytes[offset..])?;
                Ok(Val::I32(NO_MORE_DATA))
            } else {
                let bytes = &bytes[offset..offset + BUFFER_LEN];
                Self::write_bytes_to_memory(caller, ptr, &bytes[offset..=offset + BUFFER_LEN])?;
                Ok(Val::I32(1))
            }
        } else {
            Self::write_bytes_to_memory(caller, ptr, &bytes)?;
            Ok(Val::I32(NO_MORE_DATA))
        }
    }
}
