use crate::{
    host_func::{http, io, kvdb, timer},
    CallerInfo, TimerEntry, TimerQueue,
};
use rocksdb::DB;
use std::sync::{Arc, Mutex};
use vrs_primitives::NucleusId;
use wasmtime::{Engine, Linker};

pub trait FuncRegister {
    type Runtime;

    fn register_host_funcs(engine: &Engine) -> Linker<Self::Runtime>;
}

pub trait ContextAware {
    fn read_only(&self) -> bool;

    fn get_nucleus_id(&self) -> NucleusId;
}

pub trait ComponentProvider<T> {
    fn get_component(&self) -> Arc<T>;
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
    pub(crate) timer: Arc<Mutex<TimerQueue>>,
    is_get_method: bool,
    caller_infos: Vec<CallerInfo>,
    // TODO we need runtime storage to read
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
}

impl ContextAware for Runtime {
    fn read_only(&self) -> bool {
        self.is_get_method
    }

    fn get_nucleus_id(&self) -> NucleusId {
        self.id.clone()
    }
}

impl FuncRegister for Runtime {
    type Runtime = Runtime;

    fn register_host_funcs(engine: &Engine) -> Linker<Runtime> {
        let mut linker = Linker::new(engine);
        linker
            .func_new(
                "env",
                "stdout_print",
                io::stdout_print_signature(engine),
                io::stdout_print,
            )
            .unwrap();
        linker
            .func_new(
                "env",
                "stderr_print",
                io::stderr_print_signature(engine),
                io::stderr_print,
            )
            .unwrap();
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
        linker
            .func_new(
                "env",
                "timer_set_delay",
                timer::register_timer_signature(engine),
                timer::register_timer,
            )
            .unwrap();
        linker
    }
}