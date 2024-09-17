use crate::{
    host_func::{http, kvdb},
    CallerInfo, TimerEntry, TimerQueue,
};
use chrono::{DateTime, Utc};
use rocksdb::DB;
use std::sync::{Arc, Mutex};
use vrs_primitives::{AccountId, NucleusId};
use wasmtime::{Caller, Engine, FuncType, Linker, Val, ValType};

pub trait FuncRegister {
    type Runtime;

    fn register_host_funcs(engine: &Engine) -> Linker<Self::Runtime>;
}

pub trait RuntimeProvider {
    type Context;

    fn get_component<T>(&self) -> Arc<T>
    where
        T: Send + Sync;

    fn get_context<'c>(&'c self) -> &'c Self::Context;
}

#[derive(Clone, Debug)]
pub struct RuntimeParams {
    pub db_path: Box<std::path::Path>,
    pub nucleus_id: NucleusId,
    pub http_register: Arc<http::HttpCallRegister>,
}

pub struct Runtime {
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

impl Runtime {
    pub fn init(config: RuntimeParams) -> anyhow::Result<Self> {
        Ok(Self {
            id: config.nucleus_id,
            db: Arc::new(kvdb::init_rocksdb(config.db_path)?),
            http: config.http_register,
            is_get_method: false,
            caller_infos: vec![],
            timer: Arc::new(Mutex::new(TimerQueue::new())),
        })
    }

    // pub fn push_caller_info(&mut self, caller_info: CallerInfo) {
    //     self.caller_infos.push(caller_info);
    // }

    // pub fn pop_caller_info(&mut self) -> Option<CallerInfo> {
    //     self.caller_infos.pop()
    // }

    // pub fn replace_caller_infos(&mut self, caller_infos: Vec<CallerInfo>) {
    //     self.caller_infos = caller_infos;
    // }

    // pub fn is_get_method(&self) -> bool {
    //     self.is_get_method
    // }

    // pub fn set_is_get_method(&mut self, value: bool) {
    //     self.is_get_method = value;
    // }

    // pub fn pop_timer_entry(&mut self) -> Option<TimerEntry> {
    //     self.timer.lock().unwrap().pop()
    // }

    // pub fn fetch_all_timer_entries(&self) -> Vec<TimerEntry> {
    //     let mut timer = self.timer.lock().unwrap();
    //     let mut entries = Vec::new();
    //     while let Some(entry) = timer.pop() {
    //         entries.push(entry);
    //     }
    //     entries
    // }

    // pub fn push_timer_entry(&self, entry: TimerEntry) {
    //     self.timer.lock().unwrap().push(entry);
    // }
}

impl FuncRegister for Runtime {
    type Runtime = Runtime;

    fn register_host_funcs(engine: &Engine) -> Linker<Runtime> {
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
                |mut caller: Caller<'_, Self>, params, results| {
                    if caller.data().is_get_method() {
                        results[0] = Val::I32(1);
                        return Ok(());
                    }
                    if let [Val::I32(delay), Val::I32(func_ptr), Val::I32(func_len), Val::I32(params_ptr), Val::I32(params_len)] = params {
                        results[0] = Val::I32(3);
                        let func_params = crate::mem::read_bytes_from_memory(&mut caller, *params_ptr, *params_len)?;
                        let func_name = crate::mem::read_bytes_from_memory(&mut caller, *func_ptr, *func_len)?;
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
}

#[derive(Debug)]
pub struct WasmInfo {
    pub account: AccountId,
    pub name: String,
    pub version: u32,
    pub code: WasmCodeRef,
}

#[derive(Debug)]
pub enum WasmCodeRef {
    File(std::path::PathBuf),
    Blob(Vec<u8>),
}
