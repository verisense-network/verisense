mod kvdb;

use rocksdb::DB;
use std::sync::Arc;
use wasmtime::{Caller, Extern, Func, Memory, Store, Trap};

pub struct Context {
    pub(crate) db: Arc<DB>,
    is_get_method: bool,
    // 1. we need runtime storage to read
    // 3. we need http manager
    // 4. we need timer
}

impl Context {
    pub fn init(config: ContextConfig) -> anyhow::Result<Self> {
        Ok(Context {
            db: Arc::new(kvdb::init_rocksdb(config.db_path)?),
            is_get_method: false,
        })
    }

    // pub(crate) fn inject_host_funcs(store: &mut Store<Context>) -> Vec<Extern> {
    //     vec![Func::wrap(store, kvdb::storage_put).into()]
    // }
    //
    pub fn is_get_method(&self) -> bool {
        self.is_get_method
    }
    pub fn set_is_get_method(&mut self, value: bool) {
        self.is_get_method = value;
    }
    pub(crate) fn inject_host_funcs(store: &mut Store<Context>) -> Vec<Extern> {
        let signature = kvdb::storage_put_signature(store);
        vec![Func::new(store, signature, kvdb::storage_put).into()]
    }

    pub(crate) fn wasm_mem(caller: &mut Caller<'_, Context>) -> Result<Memory, Trap> {
        match caller.get_export("memory") {
            Some(Extern::Memory(mem)) => Ok(mem),
            _ => Err(Trap::HeapMisaligned),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ContextConfig {
    pub db_path: Box<std::path::Path>,
}
