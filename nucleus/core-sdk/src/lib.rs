mod constant;

pub mod error;
pub mod http;
pub mod io;
pub mod storage;
mod timer;

pub use codec;
pub use constant::*;
pub use io::{_eprint, _print, nucleus_id};
pub use paste;
pub use timer::_set_timer;
pub use vrs_core_macros::*;
pub use vrs_metadata::utils::AccountId32 as AccountId;

/// the buffer used for transfering data from host to wasm
/// this should be equal to a page size
pub const BUFFER_LEN: usize = 64 * 1024;

/// if host function returns this value, it means there is no more data to read
pub const NO_MORE_DATA: i32 = 0;

/// result of host function, T should be `codec::Codec`
pub type CallResult<T> = Result<T, error::RuntimeError>;

pub type NucleusId = vrs_metadata::utils::AccountId32;

#[inline]
pub(crate) fn allocate_buffer() -> Vec<u8> {
    vec![0; BUFFER_LEN]
}
