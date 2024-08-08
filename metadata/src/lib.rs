pub mod runtime;

pub use runtime::codegen;
pub use subxt_core::*;

pub const METADATA_BYTES: &'static [u8] = include_bytes!("../metadata.scale");

// pub fn decode_metadata(bytes: &[u8]) -> Result<subxt_core::Metadata, subxt_core::Error> {
//     subxt_core::metadata::decode_from(bytes).map_err(|e| e.into())
// }

// pub fn decode_events(bytes: Vec<u8>, metadata: subxt_core::Metadata) -> VrsRuntimeEvents {
//     subxt_core::events::decode_from(bytes, metadata)
// }

// pub type VrsRuntimeMetadata = subxt_core::Metadata;
// pub type VrsRuntimeEvents = subxt_core::events::Events<subxt_core::config::SubstrateConfig>;
// pub use subxt_core::events::StaticEvent;
