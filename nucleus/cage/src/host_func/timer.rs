use crate::{
    mem,
    runtime::{ComponentProvider, ContextAware},
    Runtime, TimerEntry, TimerQueue,
};
use chrono::Utc;
use wasmtime::{Caller, Engine, FuncType, Val, ValType};

impl ComponentProvider<std::sync::Mutex<TimerQueue>> for Runtime {
    fn get_component(&self) -> std::sync::Arc<std::sync::Mutex<TimerQueue>> {
        self.timer.clone()
    }
}

/// the signature of this host function is:
///
pub(crate) fn register_timer_signature(engine: &Engine) -> FuncType {
    FuncType::new(
        &engine,
        [
            ValType::I32,
            ValType::I32,
            ValType::I32,
            ValType::I32,
            ValType::I32,
        ],
        [ValType::I32],
    )
}

/// the signature of this host function is:
///
pub(crate) fn register_timer<R>(
    mut caller: Caller<'_, R>,
    params: &[Val],
    results: &mut [Val],
) -> anyhow::Result<()>
where
    R: ContextAware + ComponentProvider<std::sync::Mutex<TimerQueue>>,
{
    if caller.data().read_only() {
        results[0] = Val::I32(1);
        return Ok(());
    }
    if let [Val::I32(delay), Val::I32(func_ptr), Val::I32(func_len), Val::I32(params_ptr), Val::I32(params_len)] =
        params
    {
        results[0] = Val::I32(3);
        let func_params = mem::read_bytes_from_memory(&mut caller, *params_ptr, *params_len)?;
        let func_name = crate::mem::read_bytes_from_memory(&mut caller, *func_ptr, *func_len)?;
        let timestamp = Utc::now() + std::time::Duration::from_secs(*delay as u64);

        let entry = TimerEntry {
            timestamp,
            func_name: String::from_utf8(func_name)?,
            triggered_time: None,
            // TODO mark: we need to re-design the caller context, comment this to pass compile
            // caller_infos: caller.data().caller_infos.clone(),
            caller_infos: vec![],
            func_params,
        };
        let timer = caller.data().get_component();
        let mut timer = timer.lock().unwrap();
        timer.push(entry);
        results[0] = Val::I32(0);
    } else {
        results[0] = Val::I32(2);
    }
    Ok(())
}
