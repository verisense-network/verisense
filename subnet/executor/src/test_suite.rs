use crate::{
    bytecode::*, host_func::*, nucleus::Nucleus, runtime::*, state::NucleusState, vm::Vm, Gluon,
};
use std::sync::{mpsc::Sender, Arc};
use temp_dir::TempDir;
use timer::{PendingTimerQueue, SchedulerAsync};
use vrs_primitives::{AccountId, NucleusId};

#[derive(Debug)]
pub struct OutOfRuntime {
    pub http_executor: http::HttpCallExecutor,
    pub scheduler: Arc<SchedulerAsync>,
}

pub fn load_wasm_file(f: impl AsRef<std::path::Path>) -> WasmInfo {
    WasmInfo {
        version: 0,
        code: std::fs::read(f).unwrap(),
        id: NucleusId::from([1u8; 32]),
    }
}

pub fn new_mock_runtime() -> (Runtime, OutOfRuntime) {
    let (http_register, http_executor) = crate::host_func::http::new_http_manager();
    let tmp_dir = TempDir::new().unwrap();
    let db_path = tmp_dir.child("test_nucleus").into_boxed_path();
    let scheduler = Arc::new(SchedulerAsync::new());
    (
        Runtime {
            id: NucleusId::from([1u8; 32]),
            state: Arc::new(NucleusState::new(db_path).unwrap()),
            http: Arc::new(http_register),
            timer_register: Arc::new(PendingTimerQueue::new()),
            timer_scheduler: scheduler.clone(),
            tss_node: Arc::new(NodeRuntime::Empty),
            read_only: false,
            // caller_infos: vec![],
        },
        OutOfRuntime {
            http_executor,
            scheduler,
        },
    )
}

pub fn new_mock_nucleus(f: impl AsRef<std::path::Path>) -> (OutOfRuntime, Sender<(u64, Gluon)>) {
    let (sender, receiver) = std::sync::mpsc::channel();
    let (runtime, out_of_runtime) = new_mock_runtime();
    let mut nucleus = Nucleus::init(receiver, runtime);
    std::thread::spawn(move || {
        nucleus.run();
    });
    sender
        .send((
            0,
            Gluon::CodeUpgrade {
                version: 0,
                digest: [0; 32],
                wasm: Some(load_wasm_file(f)),
            },
        ))
        .unwrap();
    (out_of_runtime, sender)
}
