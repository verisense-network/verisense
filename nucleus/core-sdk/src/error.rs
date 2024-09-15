use codec::{Decode, Encode};

// *DO NOT* change the sequence of the variants
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub enum RuntimeError {
    DecodeReturnValueError,
    WriteIsNotAllowInGetMethod,
    MemoryAccessOutOfBounds,
}
