use crate::{
    host_func::{
        http::{self, HttpCallRegister},
        io, kvdb,
        timer::{self, PendingTimerQueue},
    },
    state::NucleusState,
    CallerInfo, TimerEntry,
};
use std::sync::Arc;
use vrs_primitives::NucleusId;
use wasmtime::{Engine, Linker};

pub trait FuncRegister {
    type Runtime;

    fn register_host_funcs(engine: &Engine) -> Linker<Self::Runtime>;
}

pub trait ContextAware {
    fn read_only(&self) -> bool;

    fn set_read_only(&mut self, read_only: bool);

    fn pop_all_pending_timer(&self) -> Vec<TimerEntry>;

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
    pub(crate) state: Arc<NucleusState>,
    pub(crate) http: Arc<HttpCallRegister>,
    pub(crate) register_timer: Arc<PendingTimerQueue>,
    pub(crate) read_only: bool,
    pub(crate) caller_infos: Vec<CallerInfo>,
    // TODO we need runtime storage to read
}

impl Runtime {
    pub fn init(config: RuntimeParams) -> anyhow::Result<Self> {
        Ok(Self {
            id: config.nucleus_id,
            state: Arc::new(NucleusState::new(config.db_path)?),
            http: config.http_register,
            read_only: false,
            caller_infos: vec![],
            register_timer: Arc::new(PendingTimerQueue::new()),
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
}

impl ContextAware for Runtime {
    fn read_only(&self) -> bool {
        self.read_only
    }

    fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }

    fn get_nucleus_id(&self) -> NucleusId {
        self.id.clone()
    }

    fn pop_all_pending_timer(&self) -> Vec<TimerEntry> {
        let mut timers = vec![];
        while let Some(e) = self.register_timer.pop() {
            timers.push(e);
        }
        timers
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
                "get_nucleus_id",
                io::get_nucleus_id_signature(engine),
                io::get_nucleus_id,
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
                "storage_get_prefix",
                kvdb::storage_get_prefix_signature(engine),
                kvdb::storage_get_prefix,
            )
            .unwrap();
        linker
            .func_new(
                "env",
                "storage_get_range",
                kvdb::storage_get_range_signature(engine),
                kvdb::storage_get_range,
            )
            .unwrap();
        linker
            .func_new(
                "env",
                "storage_del",
                kvdb::storage_del_signature(engine),
                kvdb::storage_del,
            )
            .unwrap();
        linker
            .func_new(
                "env",
                "storage_del_range",
                kvdb::storage_del_range_signature(engine),
                kvdb::storage_del_range,
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
