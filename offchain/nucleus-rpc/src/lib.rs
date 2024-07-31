use async_trait::async_trait;
use constants::*;
use jsonrpsee::{core::RpcResult, proc_macros::rpc, types::error::ErrorObjectOwned};
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::{self, Receiver, Sender as ReplyHandle};

#[rpc(server)]
pub trait NucleusRpc {
    #[method(name = "nucleus_post")]
    async fn post(&self, nucleus: String, op: String, payload: Vec<u8>) -> RpcResult<Vec<u8>>;

    #[method(name = "nucleus_get")]
    async fn get(&self, nucleus: String, op: String, payload: Vec<u8>) -> RpcResult<Vec<u8>>;
}

pub type NucleusResponse = Result<Vec<u8>, (i32, String)>;
pub type ReplyTo = ReplyHandle<NucleusResponse>;

#[derive(Debug)]
pub enum NucleusRequest {
    Post(ReplyTo, String, String, Vec<u8>),
    Get(ReplyTo, String, String, Vec<u8>),
}

pub struct NucleusEntry {
    sender: Sender<NucleusRequest>,
}

impl NucleusEntry {
    pub fn new(sender: Sender<NucleusRequest>) -> Self {
        Self { sender }
    }

    async fn reply(
        &self,
        req: NucleusRequest,
        rx: Receiver<NucleusResponse>,
    ) -> RpcResult<Vec<u8>> {
        self.sender.send(req).await.map_err(|_| {
            ErrorObjectOwned::owned(NUCLEUS_OFFLINE_CODE, NUCLEUS_OFFLINE_MSG, None::<()>)
        })?;
        tokio::select! {
            reply = rx => {
                match reply {
                    Ok(Ok(r)) => Ok(r),
                    Ok(Err(e)) => Err(ErrorObjectOwned::owned(e.0, e.1, None::<()>)),
                    Err(_) => Err(ErrorObjectOwned::owned(NUCLEUS_OFFLINE_CODE, NUCLEUS_OFFLINE_MSG, None::<()>)),
                }
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {
                Err(ErrorObjectOwned::owned(NUCLEUS_TIMEOUT_CODE, NUCELUS_TIMEOUT_MSG, None::<()>))
            }
        }
    }
}

#[async_trait]
impl NucleusRpcServer for NucleusEntry {
    async fn post(&self, nucleus: String, op: String, payload: Vec<u8>) -> RpcResult<Vec<u8>> {
        let (tx, rx) = oneshot::channel();
        let req = NucleusRequest::Post(tx, nucleus, op, payload);
        self.reply(req, rx).await
    }

    async fn get(&self, nucleus: String, op: String, payload: Vec<u8>) -> RpcResult<Vec<u8>> {
        let (tx, rx) = oneshot::channel();
        let req = NucleusRequest::Get(tx, nucleus, op, payload);
        self.reply(req, rx).await
    }
}

mod constants {
    pub const NUCLEUS_OFFLINE_CODE: i32 = -40001;
    pub const NUCLEUS_OFFLINE_MSG: &str = "The nucleus is offline.";
    pub const NUCLEUS_TIMEOUT_CODE: i32 = -40002;
    pub const NUCELUS_TIMEOUT_MSG: &str = "The nucleus is not responding.";
}
