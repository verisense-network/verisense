use crate::{
    host_func::{
        http::{self, HttpCallRegister},
        io, kvdb,
        timer::{self, PendingTimerQueue},
    },
    state::NucleusState,
    TimerEntry,
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

    fn rollback_all_pending_timer(&self) -> Vec<TimerEntry>;
    fn enqueue_all_pending_timer(&self);

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

#[derive(Clone)]
pub struct Runtime {
    pub(crate) id: NucleusId,
    pub(crate) state: Arc<NucleusState>,
    pub(crate) http: Arc<HttpCallRegister>,
    pub(crate) read_only: bool,
    pub(crate) timer_scheduler: Arc<timer::SchedulerAsync>,
    pub(crate) timer_register: Arc<PendingTimerQueue>,
    // TODO we need runtime storage to read
}

impl Runtime {
    pub fn init(config: RuntimeParams) -> anyhow::Result<Self> {
        Ok(Self {
            id: config.nucleus_id,
            state: Arc::new(NucleusState::new(config.db_path)?),
            http: config.http_register,
            read_only: false,
            timer_scheduler: config.timer_scheduler,
            timer_register: Arc::new(PendingTimerQueue::new()),
        })
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

    fn rollback_all_pending_timer(&self) -> Vec<TimerEntry> {
        let mut timers = vec![];
        while let Some(e) = self.timer_register.pop() {
            timers.push(e);
        }
        timers
    }
    fn enqueue_all_pending_timer(&self) {
        while let Some(e) = self.timer_register.pop() {
            self.timer_scheduler.push(e);
        }
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
            .func_new(
                "env",
                "now_timestamp",
                timer::now_timestamp_signature(engine),
                timer::now_timestamp,
            )
            .unwrap();
        linker
    }
}
