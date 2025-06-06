mod types;

use async_trait::async_trait;
use codec::Decode;
use constants::*;
use futures::prelude::*;
use jsonrpsee::{core::RpcResult, proc_macros::rpc, types::error::ErrorObjectOwned};
use sc_network_types::PeerId;
use sc_transaction_pool_api::{BlockHash, TransactionPool, TransactionSource, TransactionStatus};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::Bytes;
use std::{io::Write, path::PathBuf, sync::Arc};
use tokio::sync::{
    mpsc::Sender,
    oneshot::{self, Receiver},
};
use vrs_nucleus_executor::{Gluon, NucleusResponse};
// use vrs_nucleus_runtime_api::NucleusApi;
use vrs_primitives::NucleusId;

#[rpc(server)]
pub trait A2aRpc<Hash> {
    #[method(name = "a2a_register")]
    async fn register(&self, agent_card: types::AgentCard) -> RpcResult<NucleusId>;

    #[method(name = "a2a_list")]
    async fn list(&self) -> RpcResult<Vec<types::AgentCard>>;

    #[method(name = "a2a_find")]
    async fn find(&self, id: NucleusId) -> RpcResult<Option<types::AgentCard>>;
}

pub struct A2aServer<P, C> {
    client: Arc<C>,
    pool: Arc<P>,
}

impl<P, C> A2aServer<P, C> {
    pub fn new(client: Arc<C>, pool: Arc<P>) -> Self {
        Self { client, pool }
    }
}

#[async_trait]
impl<P, C> A2aRpcServer<BlockHash<P>> for A2aServer<P, C>
where
    P: TransactionPool + Sync + Send + 'static,
    P::Block: sp_runtime::traits::Block + Send + Sync + 'static,
    C: HeaderBackend<P::Block> + ProvideRuntimeApi<P::Block> + Send + Sync + 'static,
    // C::Api: NucleusApi<P::Block> + 'static,
{
    async fn register(&self, agent_card: types::AgentCard) -> RpcResult<NucleusId> {
        todo!();
    }

    async fn list(&self) -> RpcResult<Vec<types::AgentCard>> {
        todo!();
    }

    async fn find(&self, id: NucleusId) -> RpcResult<Option<types::AgentCard>> {
        todo!();
    }

    // async fn deploy(
    //     &self,
    //     tx: Bytes,
    //     wasm: Bytes,
    //     abi: serde_json::Value,
    // ) -> RpcResult<BlockHash<P>> {
    //     let api = self.client.runtime_api();
    //     let xt: <P::Block as sp_runtime::traits::Block>::Extrinsic =
    //         match Decode::decode(&mut &tx[..]) {
    //             Ok(xt) => xt,
    //             Err(_) => {
    //                 return Err(ErrorObjectOwned::owned(
    //                     NUCLEUS_UPGRADE_TX_ERR_CODE,
    //                     NUCLEUS_UPGRADE_TX_ERR_MSG,
    //                     None::<()>,
    //                 ))
    //             }
    //         };
    //     let best_block_hash = self.client.info().best_hash;
    //     let wasm_info = api
    //         .resolve_deploy_tx(best_block_hash, xt.clone())
    //         .ok()
    //         .flatten()
    //         .ok_or(ErrorObjectOwned::owned(
    //             NUCLEUS_UPGRADE_TX_ERR_CODE,
    //             NUCLEUS_UPGRADE_TX_ERR_MSG,
    //             None::<()>,
    //         ))?;
    //     PeerId::from_bytes(&wasm_info.node_id.0)
    //         .ok()
    //         .filter(|id| self.node_id == *id)
    //         .ok_or(ErrorObjectOwned::owned(
    //             INVALID_NODE_ADDRESS_CODE,
    //             INVALID_NODE_ADDRESS_MSG,
    //             None::<()>,
    //         ))?;

    //     let nucleus_info = api
    //         .get_nucleus_info(best_block_hash, wasm_info.nucleus_id.clone())
    //         .inspect_err(|e| {
    //             log::error!(
    //                 "Couldn't get nucleus info while upgrading wasm, caused by {:?}",
    //                 e
    //             )
    //         })
    //         .ok()
    //         .flatten()
    //         .ok_or(ErrorObjectOwned::owned(
    //             NUCLEUS_NOT_EXISTS_CODE,
    //             NUCLEUS_NOT_EXISTS_MSG,
    //             None::<()>,
    //         ))?;
    //     vrs_nucleus_executor::vm::validate_wasm_abi(
    //         &wasm.0,
    //         abi.as_array().ok_or(ErrorObjectOwned::owned(
    //             INVALID_NUCLEUS_ABI_CODE,
    //             INVALID_NUCLEUS_ABI_MSG,
    //             None::<()>,
    //         ))?,
    //     )
    //     .map_err(|e| ErrorObjectOwned::owned(NUCLEUS_ABI_NOT_MATCH_CODE, e, None::<()>))?;

    //     let path = self
    //         .nucleus_home_dir
    //         .as_path()
    //         .join(wasm_info.nucleus_id.to_string())
    //         .join("wasm/");
    //     // TODO maybe this is an unnecessary check, we are considering to support accepting the wasm upgrade in RPC nodes.
    //     if !std::fs::exists(&path)
    //                     .expect("fail to check nucleus directory, make sure the you have access right on the directory.")
    //                 {
    //                     std::fs::create_dir_all(&path).map_err(|e| {
    //                         ErrorObjectOwned::owned(
    //                             OS_ERR_CODE,
    //                             format!("Couldn't write the wasm file, caused by {:?}", e),
    //                             None::<()>,
    //                         )
    //                     })?;
    //                 }
    //     std::fs::File::create(path.join(format!("{}.wasm", nucleus_info.wasm_version + 1)))
    //         .and_then(|mut f| f.write_all(&wasm.0))
    //         .map_err(|e| {
    //             ErrorObjectOwned::owned(
    //                 OS_ERR_CODE,
    //                 format!("Couldn't write the wasm file, caused by {:?}", e),
    //                 None::<()>,
    //             )
    //         })?;
    //     let abi = serde_json::to_vec(&abi).expect("abi should be serializable");
    //     std::fs::File::create(path.join("abi.json"))
    //         .and_then(|mut f| f.write_all(&abi))
    //         .map_err(|e| {
    //             ErrorObjectOwned::owned(
    //                 OS_ERR_CODE,
    //                 format!("Couldn't write the abi, caused by {:?}", e),
    //                 None::<()>,
    //             )
    //         })?;

    //     let mut submit = self
    //         .pool
    //         .submit_and_watch(best_block_hash, TransactionSource::External, xt)
    //         .await
    //         .map_err(|e| {
    //             ErrorObjectOwned::owned(
    //                 TX_POOL_ERROR_CODE,
    //                 format!("Couldn't accept the transaction, caused by {:?}", e),
    //                 None::<()>,
    //             )
    //         })?;
    //     loop {
    //         match submit.next().await.ok_or(ErrorObjectOwned::owned(
    //             TX_POOL_ERROR_CODE,
    //             "Transaction is not included in the block.",
    //             None::<()>,
    //         ))? {
    //             TransactionStatus::InBlock((block, _)) => {
    //                 return Ok(block);
    //             }
    //             TransactionStatus::FinalityTimeout(_)
    //             | TransactionStatus::Usurped(_)
    //             | TransactionStatus::Invalid
    //             | TransactionStatus::Dropped => {
    //                 break Err(ErrorObjectOwned::owned(
    //                     TX_POOL_ERROR_CODE,
    //                     "Transaction is not included in the block.",
    //                     None::<()>,
    //                 ));
    //             }
    //             TransactionStatus::Future
    //             | TransactionStatus::Ready
    //             | TransactionStatus::Retracted(_)
    //             | TransactionStatus::Broadcast(_) => {
    //                 continue;
    //             }
    //             TransactionStatus::Finalized(_) => unreachable!(),
    //         }
    //     }
    // }
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
    pub const NUCLEUS_NOT_EXISTS_CODE: i32 = -40014;
    pub const NUCLEUS_NOT_EXISTS_MSG: &str = "Nucleus not exists.";
    pub const OS_ERR_CODE: i32 = -42000;
    pub const NUCLEUS_ABI_NOT_MATCH_CODE: i32 = -40015;
    pub const INVALID_NUCLEUS_ABI_CODE: i32 = -40016;
    pub const INVALID_NUCLEUS_ABI_MSG: &str = "Invalid nucleus ABI.";
}
