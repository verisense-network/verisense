use std::time::Duration;

use crate::{MAX_DELAY_SEC, MAX_FUNC_SIZE, MAX_PARAMS_SIZE};

#[link(wasm_import_module = "env")]
extern "C" {
    fn timer_set_delay(
        delay: i32,
        func_ptr: *const u8,
        func_len: i32,
        params_ptr: *const u8,
        params_len: i32,
    ) -> i32;
}

pub fn _set_timer(ts: Duration, func: &[u8], params: &[u8]) -> anyhow::Result<()> {
    if params.len() > MAX_PARAMS_SIZE {
        return Err(anyhow::anyhow!("params size exceeds maximum allowed size"));
    }
    if ts.as_secs() > MAX_DELAY_SEC {
        return Err(anyhow::anyhow!("delay exceeds maximum allowed size"));
    }
    if func.len() > MAX_FUNC_SIZE {
        return Err(anyhow::anyhow!("func size exceeds maximum allowed size"));
    }
    let status = unsafe {
        timer_set_delay(
            ts.as_secs() as i32,
            func.as_ptr(),
            func.len() as i32,
            params.as_ptr(),
            params.len() as i32,
        )
    };
    if status != 0 {
        Err(anyhow::anyhow!("set timer failed"))
    } else {
        Ok(())
    }
}

#[macro_export]
macro_rules! set_timer {
    ($duration:expr, $func_name:ident, $($param:expr),* $(,)?) => {{

        let __duration = std::time::Duration::from_secs($duration);
        // func_name
        let __func_name_bytes = stringify!($func_name);
        let __func_bytes = __func_name_bytes.as_bytes();

        // params
        ::vrs_core_sdk::paste::paste! {
            let __params: [<_NUCLEUS_TIMER_PARAMS_TYPE_ $func_name>] = ($($param,)*);
            let __params_bytes = <[<_NUCLEUS_TIMER_PARAMS_TYPE_ $func_name>] as ::vrs_core_sdk::codec::Encode>::encode(&__params);
        }
        ::vrs_core_sdk::_set_timer(__duration, __func_bytes, __params_bytes.as_slice())
    }};
}
