mod cage;
mod host_func;
mod mem;
mod nucleus;
mod runtime;
mod vm;

mod scheduler;
mod timer_entry;
pub(crate) use scheduler::*;
pub use timer_entry::*;

pub use cage::{start_nucleus_cage, CageParams};
pub use nucleus::Gluon;
pub use runtime::{FuncRegister, Runtime, RuntimeParams, WasmCodeRef, WasmInfo};

pub type NucleusResponse = Result<Vec<u8>, (i32, String)>;
pub type ReplyTo = tokio::sync::oneshot::Sender<NucleusResponse>;
