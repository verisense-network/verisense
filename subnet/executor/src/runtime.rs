use crate::{
    host_func::{http, io, kvdb, timer, tss, HttpCallRegister, PendingTimerQueue, TimerEntry},
    NucleusState,
};
use std::sync::Arc;
use vrs_primitives::NucleusId;
use vrs_tss::NodeRuntime;
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

    fn get_nucleus_home(&self) -> &std::path::Path;

    fn stdout(&mut self) -> &mut std::fs::File;
}

pub trait ComponentProvider<T> {
    fn get_component(&self) -> Arc<T>;
}

#[derive(Clone)]
pub struct RuntimeParams {
    pub nucleus_home: Box<std::path::Path>,
    pub nucleus_id: NucleusId,
    pub http_register: Arc<http::HttpCallRegister>,
    pub timer_scheduler: Arc<timer::SchedulerAsync>,
    pub tss_node: Arc<NodeRuntime>,
}

impl Runtime {
    pub fn init(config: RuntimeParams) -> anyhow::Result<Self> {
        Ok(Self {
            id: config.nucleus_id.clone(),
            state: Arc::new(NucleusState::new(config.nucleus_home.join("db"))?),
            stdout: std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(config.nucleus_home.join("stdout.log"))
                .map_err(|e| {
                    anyhow::anyhow!(
                        "Failed to open stdout for nucleus {}: {:?}",
                        config.nucleus_id,
                        e
                    )
                })?,
            nucleus_home: config.nucleus_home,
            http: config.http_register,
            read_only: false,
            timer_scheduler: config.timer_scheduler,
            timer_register: Arc::new(PendingTimerQueue::new()),
            tss_node: config.tss_node,
        })
    }

    pub fn state(&self) -> Arc<NucleusState> {
        self.state.clone()
    }

    fn db_path(&self) -> std::path::PathBuf {
        self.nucleus_home.join("db")
    }

    fn code_path(&self) -> std::path::PathBuf {
        self.nucleus_home.join("wasm")
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

    fn get_nucleus_home(&self) -> &std::path::Path {
        self.nucleus_home.as_ref()
    }

    fn stdout(&mut self) -> &mut std::fs::File {
        &mut self.stdout
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
            .func_new(
                "env",
                "tss_sign_host_fn",
                tss::tss_sign_signature(engine),
                tss::tss_sign,
            )
            .unwrap();
        linker
            .func_new(
                "env",
                "tss_get_public_key_host_fn",
                tss::tss_get_public_key_signature(engine),
                tss::tss_get_public_key,
            )
            .unwrap();
        linker
    }
}

pub struct Runtime {
    pub(crate) id: NucleusId,
    pub(crate) nucleus_home: Box<std::path::Path>,
    pub(crate) stdout: std::fs::File,
    pub(crate) state: Arc<NucleusState>,
    pub(crate) http: Arc<HttpCallRegister>,
    pub(crate) read_only: bool,
    pub(crate) timer_scheduler: Arc<timer::SchedulerAsync>,
    pub(crate) timer_register: Arc<PendingTimerQueue>,
    pub(crate) tss_node: Arc<NodeRuntime>,
    // TODO we need runtime storage to read
}

impl Clone for Runtime {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            nucleus_home: self.nucleus_home.clone(),
            stdout: std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(self.nucleus_home.join("stdout.log"))
                .expect("Failed to open stdout log"),
            state: self.state.clone(),
            http: self.http.clone(),
            read_only: self.read_only,
            timer_scheduler: self.timer_scheduler.clone(),
            timer_register: self.timer_register.clone(),
            tss_node: self.tss_node.clone(),
        }
    }
}
