pub mod runtime;

pub use runtime::codegen;
pub use subxt_core::*;

pub const METADATA_BYTES: &'static [u8] = include_bytes!("../metadata.scale");
