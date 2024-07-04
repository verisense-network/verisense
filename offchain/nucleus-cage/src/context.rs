mod kvdb;

use rocksdb::DB;
use wasmtime::{Caller, Extern, Func, Memory, Store, Trap};

pub struct Context {
    pub(crate) db: DB,
    // 1. we need runtime storage to read
    // 3. we need http manager
    // 4. we need timer
}

impl Context {
    // TODO
    pub fn init() -> anyhow::Result<Self> {
        Ok(Context {
            db: kvdb::init_rocksdb()?,
        })
    }

    pub(crate) fn inject_host_funcs(store: &mut Store<Context>) -> Vec<Extern> {
        vec![Func::wrap(store, kvdb::storage_put).into()]
    }

    pub(crate) fn wasm_mem(caller: &mut Caller<'_, Context>) -> Result<Memory, Trap> {
        match caller.get_export("memory") {
            Some(wasmtime::Extern::Memory(mem)) => Ok(mem),
            _ => Err(Trap::HeapMisaligned),
        }
    }
}
