mod kvdb;

use std::cell::Cell;

use rocksdb::DB;
use std::sync::Arc;
use wasmtime::{Caller, Engine, Extern, Func, FuncType, Linker, Memory, Store, Trap, Val, ValType};

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
    // pub(crate) fn inject_host_get_funcs(store: &mut Store<Context>) -> Extern {
    //     let signature_get = kvdb::storage_get_signature(store);
    //     Func::new(store, signature_get, kvdb::storage_get).into()
    // }
    // pub(crate) fn inject_host_put_funcs(store: &mut Store<Context>) -> Extern {
    //     let signature_put = kvdb::storage_put_signature(store);
    //     Func::new(store, signature_put, kvdb::storage_put).into()
    // }
    pub(crate) fn inject_host_funcs(engine: &Engine) -> Linker<Context> {
        let mut linker = Linker::new(engine);

        linker
            .func_new(
                "env",
                "storage_put",
                FuncType::new(
                    &engine,
                    [ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                    [ValType::I32],
                ),
                |mut caller: Caller<'_, Context>, params, results| {
                    let result = kvdb::storage_put_split(
                        caller,
                        params[0].unwrap_i32(),
                        params[1].unwrap_i32(),
                        params[2].unwrap_i32(),
                        params[3].unwrap_i32(),
                    );
                    results[0] = Val::I32(result);
                    Ok(())
                },
            )
            .unwrap();

        linker
            .func_new(
                "env",
                "storage_get",
                FuncType::new(
                    &engine,
                    [ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                    [ValType::I32],
                ),
                |mut caller: Caller<'_, Context>, params, results| {
                    let result = kvdb::storage_get_split(
                        caller,
                        params[0].unwrap_i32(),
                        params[1].unwrap_i32(),
                        params[2].unwrap_i32(),
                        params[3].unwrap_i32(),
                    );
                    results[0] = Val::I32(result);
                    Ok(())
                },
            )
            .unwrap();

        linker
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
