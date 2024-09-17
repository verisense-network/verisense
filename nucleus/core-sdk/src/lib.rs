mod constant;

pub mod error;
pub mod http;
pub mod storage;
pub mod timer;

pub use codec;
pub use constant::*;
pub use paste::paste as macro_paste;
pub use timer::_set_timer;

/// the buffer used for transfering data from host to wasm
/// this should be equal to a page size
pub const BUFFER_LEN: usize = 64 * 1024;

/// if host function returns this value, it means there is no more data to read
pub const NO_MORE_DATA: i32 = 0;

/// result of host function, T should be `codec::Codec`
pub type CallResult<T> = Result<T, error::RuntimeError>;

#[inline]
pub(crate) fn allocate_buffer() -> Vec<u8> {
    Vec::with_capacity(BUFFER_LEN)
}
