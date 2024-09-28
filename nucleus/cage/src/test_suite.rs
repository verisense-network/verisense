use crate::{bytecode::*, host_func::*, runtime::*, vm::Vm, TimerQueue};
use temp_dir::TempDir;
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
    let (http_register, mut http_executor) = crate::host_func::http::new_http_manager();
    let tmp_dir = TempDir::new().unwrap();
    let db_path = tmp_dir.child("test_nucleus").into_boxed_path();

    Runtime {
        id: NucleusId::from([1u8; 32]),
        db: std::sync::Arc::new(kvdb::init_rocksdb(db_path).unwrap()),
        http: std::sync::Arc::new(http_register),
        timer: std::sync::Arc::new(std::sync::Mutex::new(TimerQueue::new())),
        is_get_method: false,
        caller_infos: vec![],
    }
}

pub fn new_vm_with_executable(f: impl AsRef<std::path::Path>) -> (Vm<Runtime>, OutOfRuntime) {
    let wasm = load_wasm_file(f);
    let (runtime, out_of_runtime) = new_mock_runtime();
    (Vm::new_instance(&wasm, runtime).unwrap(), out_of_runtime)
}
