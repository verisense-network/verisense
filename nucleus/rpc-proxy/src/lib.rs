use async_trait::async_trait;
use constants::*;
use jsonrpsee::{
    core::RpcResult,
    proc_macros::rpc,
    types::error::{ErrorCode, ErrorObjectOwned},
};
use sc_transaction_pool_api::{
    error::IntoPoolError, BlockHash, InPoolTransaction, TransactionFor, TransactionPool,
    TransactionSource, TxHash,
};
use sp_api::{ApiExt, ProvideRuntimeApi};
use sp_core::Bytes;
use tokio::sync::{
    mpsc::Sender,
    oneshot::{self, Receiver},
};
use vrs_nucleus_cage::{Gluon, NucleusResponse};
use vrs_primitives::NucleusId;

#[rpc(server)]
pub trait NucleusRpc<Hash, BlockHash> {
    #[method(name = "nucleus_post")]
    async fn post(&self, nucleus: NucleusId, op: String, payload: Bytes) -> RpcResult<String>;

    #[method(name = "nucleus_get")]
    async fn get(&self, nucleus: NucleusId, op: String, payload: Bytes) -> RpcResult<String>;

    #[method(name = "nucleus_deploy")]
    async fn deploy(&self, tx: Bytes, wasm: Bytes) -> RpcResult<Hash>;
}

pub struct NucleusEntry<P, C> {
    sender: Sender<(NucleusId, Gluon)>,
    client: Arc<Client>,
    pool: Arc<P>,
}

impl<P, C> NucleusEntry<P, C> {
    pub fn new(sender: Sender<(NucleusId, Gluon)>, client: Arc<Client>, pool: Arc<P>) -> Self {
        Self {
            sender,
            client,
            pool,
        }
    }

    async fn reply(
        &self,
        req: (NucleusId, Gluon),
        rx: Receiver<NucleusResponse>,
    ) -> RpcResult<String> {
        tokio::select! {
            v = self.sender.send(req) => {
                if v.is_err() {
                    return ErrorObjectOwned::owned(NUCLEUS_OFFLINE_CODE, NUCLEUS_OFFLINE_MSG, None::<()>);
                }
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(2)) => {
                return Err(ErrorObjectOwned::owned(NUCLEUS_TIMEOUT_CODE, NUCLEUS_TIMEOUT_MSG, None::<()>));
            }
        }
        tokio::select! {
            reply = rx => {
                match reply {
                    Ok(Ok(r)) => Ok(hex::encode(r)),
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
impl<P, C> NucleusRpcServer<TxHash<P>, BlockHash<P>> for NucleusEntry<P, C>
where
    P: TransactionPool + Sync + Send + 'static,
    C: HeaderBackend<P::Block> + ProvideRuntimeApi<P::Block> + Send + Sync + 'static,
    C::Api: SessionKeys<P::Block>,
    P::Hash: Unpin,
    <P::Block as sp_runtime::traits::Block>::Hash: Unpin,
{
    async fn post(&self, nucleus: NucleusId, op: String, payload: Bytes) -> RpcResult<String> {
        let (tx, rx) = oneshot::channel();
        let req = (
            nucleus,
            Gluon::PostRequest {
                endpoint: op,
                payload: payload.0,
                reply_to: Some(tx),
            },
        );
        self.reply(req, rx).await
    }

    async fn get(&self, nucleus: NucleusId, op: String, payload: Bytes) -> RpcResult<String> {
        let (tx, rx) = oneshot::channel();
        let req = (
            nucleus,
            Gluon::GetRequest {
                endpoint: op,
                payload: payload.0,
                reply_to: Some(tx),
            },
        );
        self.reply(req, rx).await
    }

    async fn deploy(&self, tx: Bytes, wasm: Bytes) -> RpcResult<TxHash<P>> {
        let xt: Block::Extrinsic = match Decode::decode(&mut &tx[..]) {
            Ok(xt) => xt,
            Err(err) => return Err(Error::Client(Box::new(err)).into()),
        };

        let best_block_hash = self.client.info().best_hash;
        self.pool
            .submit_one(best_block_hash, TX_SOURCE, xt)
            .await
            .map_err(|e| {
                e.into_pool_error()
                    .map(|e| Error::Pool(e))
                    .unwrap_or_else(|e| Error::Verification(Box::new(e)))
                    .into()
            })
    }
}

mod constants {
    pub const NUCLEUS_OFFLINE_CODE: i32 = -40001;
    pub const NUCLEUS_OFFLINE_MSG: &str = "The nucleus is offline.";
    pub const NUCLEUS_TIMEOUT_CODE: i32 = -40002;
    pub const NUCELUS_TIMEOUT_MSG: &str = "The nucleus is not responding.";
}
