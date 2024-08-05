mod cage;
mod context;
mod nucleus;
mod vm;
mod wasm_code;

pub use cage::{start_nucleus_cage, CageParameters};
pub use nucleus::Gluon;
pub type NucleusResponse = Result<Vec<u8>, (i32, String)>;
pub type ReplyTo = tokio::sync::oneshot::Sender<NucleusResponse>;
