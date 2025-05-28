#[cfg(test)]
pub mod test_suite;

pub mod host_func;

mod code;
mod mem;
mod nucleus;
mod runtime;
pub mod state;
pub mod vm;

pub use code::WasmInfo;
pub use nucleus::{Event, Gluon, Nucleus};
pub use runtime::{Runtime, RuntimeParams};
pub use state::NucleusState;
pub use vm::WasmCallError;

pub type NucleusTunnel = std::sync::mpsc::Sender<(u64, Gluon)>;
pub type RpcReplyChannel = tokio::sync::oneshot::Sender<NucleusResponse>;
pub type NucleusResponse = Result<Vec<u8>, (i32, String)>;
