
use crate::mem::ErrorWriter;
use crate::{
    mem,
    runtime::{ComponentProvider, ContextAware},
    Runtime,
};
use codec::Encode;
use vrs_core_sdk::{error::RuntimeError, CallResult, BUFFER_LEN, NO_MORE_DATA};
use vrs_tss::crypto::CryptoType;
use wasmtime::{Caller, Engine, FuncType, Val, ValType};

impl ComponentProvider<vrs_tss::NodeRuntime> for Runtime {
    fn get_component(&self) -> std::sync::Arc<vrs_tss::NodeRuntime> {
        self.tss_node.clone()
    }
}

/// the signature of this host function is:
///
/// ```
/// fn tss_get_public_key(crypto_type: u8, tweak_ptr: *const u8, tweak_len: u32,  return_ptr: *mut u8) -> i32;
/// ```
pub(crate) fn tss_get_public_key_signature(engine: &Engine) -> FuncType {
    FuncType::new(
        engine,
        [ValType::I32, ValType::I32, ValType::I32, ValType::I32],
        [ValType::I32],
    )
}

/// the signature of this host function is:
///
/// ```
/// fn tss_get_public_key(crypto_type: u8, tweak_ptr: *const u8, tweak_len: i32,  return_ptr: *mut u8) -> i32;
/// ```
pub(crate) fn tss_get_public_key<R>(
    mut caller: Caller<'_, R>,
    params: &[Val],
    result: &mut [Val],
) -> anyhow::Result<()>
where
    R: ContextAware + ComponentProvider<vrs_tss::NodeRuntime>,
{
    result[0] = Val::I32(NO_MORE_DATA);
    let crypto_type = params[0].unwrap_i32();
    let tweak_ptr = params[1].unwrap_i32();
    let tweak_len = params[2].unwrap_i32();
    let r_ptr = params[3].unwrap_i32();
    let tweak = mem::read_bytes_from_memory(&mut caller, tweak_ptr, tweak_len)?;
    let node = caller.data().get_component();
    // check crypto_type
    let crypto_type = match CryptoType::try_from(crypto_type as u8)
        .write_error_to_memory::<_, Vec<u8>, _>(&mut caller, r_ptr, |e| {
            RuntimeError::TssError(e.to_string())
        }) {
        Some(crypto_type) => crypto_type,
        None => return Ok(()),
    };
    let public_key = match node
        .get_public_key(crypto_type, tweak)
        .write_error_to_memory::<_, Vec<u8>, _>(&mut caller, r_ptr, |e| {
            RuntimeError::TssError(e.to_string())
        }) {
        Some(public_key) => public_key,
        None => return Ok(()),
    };
    let bytes = CallResult::<Vec<u8>>::Ok(public_key).encode();
    assert!(bytes.len() <= BUFFER_LEN);
    mem::write_bytes_to_memory(&mut caller, r_ptr, &bytes).expect("write to wasm failed");
    Ok(())
}

// fn _tss_sign(
//     crypto_type: u8,
//     tweak_ptr: *const u8,
//     tweak_len: i32,
//     message_ptr: *const u8,
//     message_len: i32,
//     return_ptr: *mut u8,
// ) -> i32;
/// the signature of this host function is:
///
/// ```
/// fn tss_sign(crypto_type: u8, tweak: *const u8, tweak_len: i32, message: *const u8, message_len: i32, return_ptr: *mut u8) -> i32;
/// ```
pub(crate) fn tss_sign_signature(engine: &Engine) -> FuncType {
    FuncType::new(
        engine,
        [
            ValType::I32,
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
/// ```
/// fn tss_sign(crypto_type: u8, tweak: *const u8, tweak_len: i32, message: *const u8, message_len: i32, return_ptr: *mut u8) -> i32;
/// ```
pub fn tss_sign<R>(
    mut caller: Caller<'_, R>,
    params: &[Val],
    result: &mut [Val],
) -> anyhow::Result<()>
where
    R: ComponentProvider<vrs_tss::NodeRuntime>,
{
    result[0] = Val::I32(NO_MORE_DATA);
    let crypto_type = params[0].unwrap_i32();
    let tweak_ptr = params[1].unwrap_i32();
    let tweak_len = params[2].unwrap_i32();
    let message_ptr = params[3].unwrap_i32();
    let message_len = params[4].unwrap_i32();
    let r_ptr = params[5].unwrap_i32();
    let tweak = mem::read_bytes_from_memory(&mut caller, tweak_ptr, tweak_len)
        .expect("can't read bytes from wasm");
    let message = mem::read_bytes_from_memory(&mut caller, message_ptr, message_len)
        .expect("can't read bytes from wasm");
    let node = caller.data().get_component();
    // check crypto_type
    let crypto_type = match CryptoType::try_from(crypto_type as u8)
        .write_error_to_memory::<_, Vec<u8>, _>(&mut caller, r_ptr, |e| {
            RuntimeError::TssError(e.to_string())
        }) {
        Some(crypto_type) => crypto_type,
        None => return Ok(()),
    };
    let signature = match node
        .sign(crypto_type, message, tweak)
        .write_error_to_memory::<_, Vec<u8>, _>(&mut caller, r_ptr, |e| {
            RuntimeError::TssError(e.to_string())
        }) {
        Some(signature) => signature,
        None => return Ok(()),
    };
    let bytes = CallResult::<Vec<u8>>::Ok(signature).encode();
    assert!(bytes.len() <= BUFFER_LEN);
    mem::write_bytes_to_memory(&mut caller, r_ptr, &bytes).expect("write to wasm failed");
    Ok(())
}
