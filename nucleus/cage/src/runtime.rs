use crate::{
    host_func::{http, kvdb, timer},
    CallerInfo, TimerEntry, TimerQueue,
};
use rocksdb::DB;
use std::sync::{Arc, Mutex};
use vrs_primitives::{AccountId, NucleusId};
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
    pub timer_scheduler: Arc<timer::SchedulerAsync>,
}

pub struct Runtime {
    pub(crate) id: NucleusId,
    pub(crate) db: Arc<DB>,
    pub(crate) http: Arc<http::HttpCallRegister>,
    pub(crate) timer: Arc<timer::SchedulerAsync>,
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
            timer: config.timer_scheduler,
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
