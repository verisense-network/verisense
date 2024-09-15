mod constant;

pub mod error;
pub mod http;
pub mod storage;
pub mod timer;

pub use codec;
pub use constant::*;
pub use paste::paste as macro_paste;
// TODO remove this
pub use vrs_primitives::{AccountId, Balance, BlockNumber, Hash, Nonce, Signature};

pub use timer::_set_timer;

/// the buffer used for transfering data from host to wasm
pub const BUFFER_LEN: usize = 64 * 1024;

/// if host function returns this value, it means there is no more data to read
pub const NO_MORE_DATA: i32 = 0;

#[inline]
pub(crate) fn allocate_buffer() -> Vec<u8> {
    Vec::with_capacity(BUFFER_LEN)
}
