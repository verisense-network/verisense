use crate::{
    runtime::{ContextAware, FuncRegister},
    vm::Vm,
    NucleusTunnel, RpcReplyChannel, WasmCallError, WasmInfo,
};
use codec::{Decode, Encode};
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;
use vrs_core_sdk::{
    http::{HttpRequest, HttpResponse},
    CallResult,
};
use vrs_primitives::NucleusId;

pub struct Nucleus<R> {
    receiver: Receiver<(u64, Gluon)>,
    vm: Option<Vm<R>>,
    runtime: R,
    token_timeout_tx: tokio::sync::mpsc::Sender<NucleusId>,
}

impl<R> Nucleus<R>
where
    R: ContextAware + FuncRegister<Runtime = R> + Send + Clone + Sync + 'static,
{
    /// spawn a native thread to run nucleus
    pub fn start(
        runtime: R,
        token_timeout_tx: tokio::sync::mpsc::Sender<NucleusId>,
    ) -> NucleusTunnel {
        let (tx, rx) = mpsc::channel();
        let mut nucleus = Nucleus {
            receiver: rx,
            vm: None,
            runtime,
            token_timeout_tx,
        };
        std::thread::spawn(move || nucleus.run());
        tx
    }

    fn run(&mut self) {
        loop {
            // limit the single call
            if let Ok((_id, msg)) = self.receiver.recv_timeout(Duration::from_secs(20)) {
                // TODO save msg with id to state
                if let Err(e) = self.accept(msg) {
                    log::error!(
                        "Nucleus {} interrupted: {:?}",
                        self.runtime.get_nucleus_id(),
                        e
                    );
                    break;
                }
            } else {
                let _ = self.token_timeout_tx.send(self.runtime.get_nucleus_id());
            }
        }
    }

    // this wasm's validity is guaranteed by the cage
    fn upgrade_code(&mut self, wasm: WasmInfo) {
        match Vm::new_instance(&wasm, &self.runtime) {
            Ok(mut vm) => {
                vm.call_init();
                self.vm.replace(vm);
            }
            Err(e) => {
                log::error!("Init VM for nucleus {} failed, {:?}.", &wasm.id, e);
            }
        }
    }

    fn call<F, T>(&mut self, f: F) -> Result<T, WasmCallError>
    where
        F: FnOnce(&mut Vm<R>) -> Result<T, WasmCallError>,
    {
        let mut vm = self.vm.as_mut().ok_or(WasmCallError::WasmNotInitialized)?;
        f(&mut vm)
    }

    fn accept(&mut self, msg: Gluon) -> anyhow::Result<()> {
        match msg {
            Gluon::CodeUpgrade { wasm } => {
                self.upgrade_code(wasm);
            }
            Gluon::HttpCallback {
                request_id,
                payload,
            } => {
                if let Err(e) = self.call(|vm| vm.call_http_callback((request_id, payload))) {
                    log::error!("fail to call http callback due to: {:?}", e);
                }
            }
            Gluon::GetRequest {
                endpoint,
                payload,
                reply_to,
            } => {
                let result = self
                    .call(|vm| vm.call_get(&endpoint, payload))
                    .map_err(|e| e.into());
                let _ = reply_to.map(|reply_to| reply_to.send(result));
            }
            Gluon::PostRequest {
                endpoint,
                payload,
                reply_to,
            } => {
                let result = self
                    .call(|vm| vm.call_post(&endpoint, payload))
                    .map_err(|e| e.into());
                let _ = reply_to.map(|reply_to| reply_to.send(result));
            }
            Gluon::AbiRequest { reply_to } => {
                let result = self.call(|vm| vm.call_abi()).map_err(|e| e.into());
                let _ = reply_to.map(|reply_to| reply_to.send(result));
            }
            Gluon::TimerTrigger {
                endpoint,
                task_id,
                payload,
            } => {
                if let Err(e) = self.call(|vm| vm.call_timer_trigger(&endpoint, payload)) {
                    log::error!("fail to call timer callback: {} due to: {:?}", &endpoint, e);
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Gluon {
    CodeUpgrade {
        wasm: WasmInfo,
    },
    PostRequest {
        endpoint: String,
        payload: Vec<u8>,
        reply_to: Option<RpcReplyChannel>,
    },
    GetRequest {
        endpoint: String,
        payload: Vec<u8>,
        reply_to: Option<RpcReplyChannel>,
    },
    AbiRequest {
        reply_to: Option<RpcReplyChannel>,
    },
    TimerTrigger {
        endpoint: String,
        task_id: u64,
        payload: Vec<u8>,
    },
    HttpCallback {
        request_id: u64,
        payload: CallResult<HttpResponse>,
    },
}

impl From<&Gluon> for Event {
    fn from(gluon: &Gluon) -> Self {
        match gluon {
            Gluon::CodeUpgrade { wasm } => Event::CodeUpgrade {
                version: wasm.version,
                digest: wasm.digest(),
            },
            Gluon::PostRequest {
                endpoint, payload, ..
            } => Event::PostRequest {
                endpoint: endpoint.clone(),
                payload: payload.clone(),
            },
            Gluon::GetRequest { .. } | Gluon::AbiRequest { .. } => Event::Dummy,
            Gluon::TimerTrigger {
                task_id,
                endpoint,
                payload,
                ..
            } => Event::TimerTrigger {
                task_id: *task_id,
                endpoint: endpoint.clone(),
                payload: payload.clone(),
            },
            Gluon::HttpCallback {
                request_id,
                payload,
            } => Event::HttpCallback {
                request_id: *request_id,
                payload: payload.clone(),
            },
        }
    }
}

/// Serialized `Gluon`, i.e. received from peers or call `gluon.into()` then save it
#[derive(Debug, Clone, Encode, Decode)]
pub enum Event {
    #[codec(index = 0)]
    CodeUpgrade { version: u32, digest: [u8; 32] },
    #[codec(index = 1)]
    PostRequest { endpoint: String, payload: Vec<u8> },
    #[codec(index = 2)]
    TimerRegister {
        task_id: u64,
        delay: u64,
        endpoint: String,
        payload: Vec<u8>,
    },
    #[codec(index = 3)]
    TimerTrigger {
        task_id: u64,
        endpoint: String,
        payload: Vec<u8>,
    },
    #[codec(index = 4)]
    HttpRequest {
        request_id: u64,
        ssl_key: Vec<u8>,
        request: HttpRequest,
    },
    #[codec(index = 5)]
    HttpCallback {
        request_id: u64,
        payload: CallResult<HttpResponse>,
    },
    #[codec(skip)]
    Dummy,
}
