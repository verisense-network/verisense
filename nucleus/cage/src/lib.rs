mod bytecode;
mod cage;
mod host_func;
mod mem;
mod nucleus;
mod runtime;
mod vm;

#[cfg(test)]
pub mod test_suite;

pub(crate) use host_func::scheduler::*;
pub(crate) use host_func::timer_entry::*;

pub use bytecode::{WasmCodeRef, WasmInfo};
pub use cage::{start_nucleus_cage, CageParams};
pub use nucleus::Gluon;
pub use runtime::{Runtime, RuntimeParams};

pub type NucleusResponse = Result<Vec<u8>, (i32, String)>;
pub type ReplyTo = tokio::sync::oneshot::Sender<NucleusResponse>;
pub type TimersReplyTo = tokio::sync::oneshot::Sender<Vec<TimerEntry>>;
