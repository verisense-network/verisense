mod bytecode;
mod cage;
mod host_func;
mod keystore;
mod mem;
mod nucleus;
mod runtime;
mod state;
mod vm;

#[cfg(test)]
pub mod test_suite;

pub(crate) use host_func::timer_entry::*;

pub use bytecode::WasmInfo;
pub use cage::{start_nucleus_cage, CageParams};
pub use nucleus::{Event, Gluon};
pub use runtime::{Runtime, RuntimeParams};

pub type NucleusResponse = Result<Vec<u8>, (i32, String)>;
pub type ReplyTo = tokio::sync::oneshot::Sender<NucleusResponse>;
