use std::time::Duration;

use constant::{MAX_DELAY_SEC, MAX_FUNC_SIZE, MAX_PARAMS_SIZE};

mod constant {
    pub const MAX_GET_RETURN_SIZE: usize = 65536 * 1024;
    pub const MAX_DELAY_SEC: u64 = 60 * 60 * 24 * 365;
    pub const MAX_PARAMS_SIZE: usize = 1024 * 1024;
    pub const MAX_FUNC_SIZE: usize = 1024;
}

#[link(wasm_import_module = "env")]
extern "C" {
    fn timer_set_delay(
        delay: i32,
        func_ptr: *const u8,
        func_len: i32,
        params_ptr: *const u8,
        params_len: i32,
    ) -> i32;

    fn now_timestamp() -> i32;
}

pub fn now() -> i32 {
    unsafe { now_timestamp() }
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
    ($duration:expr, $func_call:ident ( $($param:expr),* $(,)? )) => {{
        let __duration: std::time::Duration = $duration;

        // Extract the function name as a string
        let __func_name_bytes = stringify!($func_call);
        let __func_bytes = __func_name_bytes.as_bytes();

        // Handle parameters (or no parameters)
        ::vrs_core_sdk::paste::paste! {
            let __params: [<_NUCLEUS_TIMER_PARAMS_TYPE_ $func_call>] = ($($param,)*);
            let __params_bytes = <[<_NUCLEUS_TIMER_PARAMS_TYPE_ $func_call>] as ::vrs_core_sdk::codec::Encode>::encode(&__params);
        }

        ::vrs_core_sdk::_set_timer(__duration, __func_bytes, __params_bytes.as_slice())
    }};
}
