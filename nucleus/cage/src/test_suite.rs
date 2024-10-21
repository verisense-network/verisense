use crate::{
    bytecode::*, host_func::*, nucleus::Nucleus, runtime::*, state::NucleusState, vm::Vm, Gluon,
};
use std::sync::{mpsc::Receiver, Arc};
use temp_dir::TempDir;
use timer::PendingTimerQueue;
use vrs_primitives::{AccountId, NucleusId};

#[derive(Debug)]
pub struct OutOfRuntime {
    pub http_executor: http::HttpCallExecutor,
}

pub fn load_wasm_file(f: impl AsRef<std::path::Path>) -> WasmInfo {
    WasmInfo {
        account: AccountId::from([0u8; 32]),
        name: "wasm-as-test".to_string(),
        version: 0,
        code: WasmCodeRef::File(f.as_ref().to_path_buf()),
    }
}

pub fn new_mock_runtime() -> (Runtime, OutOfRuntime) {
    let (http_register, http_executor) = crate::host_func::http::new_http_manager();
    let tmp_dir = TempDir::new().unwrap();
    let db_path = tmp_dir.child("test_nucleus").into_boxed_path();

    (
        Runtime {
            id: NucleusId::from([1u8; 32]),
            state: Arc::new(NucleusState::new(db_path).unwrap()),
            http: Arc::new(http_register),
            register_timer: Arc::new(PendingTimerQueue::new()),
            read_only: false,
            caller_infos: vec![],
        },
        OutOfRuntime { http_executor },
    )
}

pub fn new_vm_with_executable(f: impl AsRef<std::path::Path>) -> (Vm<Runtime>, OutOfRuntime) {
    let wasm = load_wasm_file(f);
    let (runtime, out_of_runtime) = new_mock_runtime();
    (Vm::new_instance(&wasm, runtime).unwrap(), out_of_runtime)
}

pub fn new_mock_nucleus(
    receiver: Receiver<(u64, Gluon)>,
    f: impl AsRef<std::path::Path>,
) -> (Nucleus<Runtime>, OutOfRuntime) {
    let (runtime, out_of_runtime) = new_mock_runtime();
    (
        Nucleus::init(receiver, runtime, load_wasm_file(f)).unwrap(),
        out_of_runtime,
    )
}
