use async_trait::async_trait;
use codec::Decode;
use constants::*;
use futures::prelude::*;
use jsonrpsee::{
    core::RpcResult, proc_macros::rpc, types::error::ErrorObjectOwned, PendingSubscriptionSink,
};
use sc_network_types::PeerId;
use sc_transaction_pool_api::{BlockHash, TransactionPool, TransactionSource, TransactionStatus};
use sp_api::{ApiExt, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_core::Bytes;
use std::{io::Write, path::PathBuf, sync::Arc};
use tokio::sync::{
    mpsc::Sender,
    oneshot::{self, Receiver},
};
use vrs_nucleus_cage::{Gluon, NucleusResponse};
use vrs_nucleus_runtime_api::NucleusApi;
use vrs_primitives::NucleusId;

#[rpc(server)]
pub trait NucleusRpc<Hash> {
    #[method(name = "nucleus_post")]
    async fn post(&self, nucleus: NucleusId, op: String, payload: Bytes) -> RpcResult<String>;

    #[method(name = "nucleus_get")]
    async fn get(&self, nucleus: NucleusId, op: String, payload: Bytes) -> RpcResult<String>;

    #[method(name = "nucleus_deploy")]
    async fn deploy(&self, tx: Bytes, wasm: Bytes) -> RpcResult<Hash>;

    #[subscription(name = "nucleus_subscribeState", unsubscribe = "nucleus_unsubscribeState", item = Bytes)]
    fn subscribe_state(&self, nucleus: NucleusId, key: String);
}

pub struct NucleusEntry<P, C> {
    sender: Sender<(NucleusId, Gluon)>,
    client: Arc<C>,
    pool: Arc<P>,
    node_id: PeerId,
    nucleus_home_dir: PathBuf,
}

impl<P, C> NucleusEntry<P, C> {
    pub fn new(
        sender: Sender<(NucleusId, Gluon)>,
        client: Arc<C>,
        pool: Arc<P>,
        node_id: PeerId,
        nucleus_home_dir: PathBuf,
    ) -> Self {
        Self {
            sender,
            client,
            pool,
            node_id,
            nucleus_home_dir,
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
                    return Err(ErrorObjectOwned::owned(NUCLEUS_OFFLINE_CODE, NUCLEUS_OFFLINE_MSG, None::<()>));
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
                Err(ErrorObjectOwned::owned(NUCLEUS_TIMEOUT_CODE, NUCLEUS_TIMEOUT_MSG, None::<()>))
            }
        }
    }
}

#[async_trait]
impl<P, C> NucleusRpcServer<BlockHash<P>> for NucleusEntry<P, C>
where
    P: TransactionPool + Sync + Send + 'static,
    P::Block: sp_runtime::traits::Block + Send + Sync + 'static,
    C: HeaderBackend<P::Block> + ProvideRuntimeApi<P::Block> + Send + Sync + 'static,
    C::Api: NucleusApi<P::Block> + 'static,
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

    async fn deploy(&self, tx: Bytes, wasm: Bytes) -> RpcResult<BlockHash<P>> {
        let api = self.client.runtime_api();
        let xt: <P::Block as sp_runtime::traits::Block>::Extrinsic =
            match Decode::decode(&mut &tx[..]) {
                Ok(xt) => xt,
                Err(_) => {
                    return Err(ErrorObjectOwned::owned(
                        NUCLEUS_UPGRADE_TX_ERR_CODE,
                        NUCLEUS_UPGRADE_TX_ERR_MSG,
                        None::<()>,
                    ))
                }
            };
        let best_block_hash = self.client.info().best_hash;
        let wasm_info = api
            .resolve_deploy_tx(best_block_hash, xt.clone())
            .ok()
            .flatten()
            .ok_or(ErrorObjectOwned::owned(
                NUCLEUS_UPGRADE_TX_ERR_CODE,
                NUCLEUS_UPGRADE_TX_ERR_MSG,
                None::<()>,
            ))?;
        PeerId::from_bytes(&wasm_info.node_id.0)
            .ok()
            .filter(|id| self.node_id == *id)
            .ok_or(ErrorObjectOwned::owned(
                INVALID_NODE_ADDRESS_CODE,
                INVALID_NODE_ADDRESS_MSG,
                None::<()>,
            ))?;

        let mut submit = self
            .pool
            .submit_and_watch(best_block_hash, TransactionSource::External, xt)
            .await
            .map_err(|e| {
                ErrorObjectOwned::owned(
                    TX_POOL_ERROR_CODE,
                    format!("Couldn't accept the transaction, caused by {:?}", e),
                    None::<()>,
                )
            })?;
        loop {
            match submit.next().await.ok_or(ErrorObjectOwned::owned(
                TX_POOL_ERROR_CODE,
                "Transaction is not included in the block.",
                None::<()>,
            ))? {
                TransactionStatus::InBlock((block, _)) => {
                    let dir = self
                        .nucleus_home_dir
                        .as_path()
                        .join(wasm_info.nucleus_id.to_string());
                    // TODO rename the previous wasm file
                    std::fs::create_dir_all(&dir)
                        .and_then(|_| std::fs::File::create(dir.join("code.wasm")))
                        .and_then(|mut f| f.write_all(&wasm.0))
                        .map_err(|e| {
                            ErrorObjectOwned::owned(
                                OS_ERR_CODE,
                                format!("Couldn't write the wasm file, caused by {:?}", e),
                                None::<()>,
                            )
                        })?;
                    return Ok(block);
                }
                TransactionStatus::FinalityTimeout(_)
                | TransactionStatus::Usurped(_)
                | TransactionStatus::Invalid
                | TransactionStatus::Dropped => {
                    break Err(ErrorObjectOwned::owned(
                        TX_POOL_ERROR_CODE,
                        "Transaction is not included in the block.",
                        None::<()>,
                    ));
                }
                TransactionStatus::Future
                | TransactionStatus::Ready
                | TransactionStatus::Retracted(_)
                | TransactionStatus::Broadcast(_) => {
                    continue;
                }
                TransactionStatus::Finalized(_) => unreachable!(),
            }
        }
    }

    fn subscribe_state(&self, sink: PendingSubscriptionSink, nucleus: NucleusId, key: String) {}
}

mod constants {
    pub const NUCLEUS_OFFLINE_CODE: i32 = -40001;
    pub const NUCLEUS_OFFLINE_MSG: &str = "The nucleus is offline.";
    pub const NUCLEUS_TIMEOUT_CODE: i32 = -40002;
    pub const NUCLEUS_TIMEOUT_MSG: &str = "The nucleus is not responding.";
    pub const TX_POOL_ERROR_CODE: i32 = -40010;
    pub const NUCLEUS_UPGRADE_TX_ERR_CODE: i32 = -40011;
    pub const NUCLEUS_UPGRADE_TX_ERR_MSG: &str = "The nucleus upgrading transaction is invalid.";
    pub const INVALID_NODE_ADDRESS_CODE: i32 = -40012;
    pub const INVALID_NODE_ADDRESS_MSG: &str = "Invalid node address.";
    pub const OS_ERR_CODE: i32 = -42000;
}
