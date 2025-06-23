use async_trait::async_trait;
use codec::Decode;
use constants::*;
use futures::prelude::*;
use jsonrpsee::{core::RpcResult, proc_macros::rpc, types::ErrorObjectOwned};
use sc_network::service::traits::NetworkService;
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
use vrs_gluon_relayer::{ForwardRequest, Relayer};
use vrs_nucleus_executor::{Gluon, NucleusError, NucleusResponse};
use vrs_nucleus_runtime_api::NucleusRuntimeApi;
use vrs_primitives::{AccountId, NucleusId};

#[rpc(server)]
pub trait NucleusApi<Hash> {
    #[method(name = "nucleus_post")]
    async fn post(&self, nucleus: NucleusId, op: String, payload: Bytes) -> RpcResult<String>;

    #[method(name = "nucleus_get")]
    async fn get(&self, nucleus: NucleusId, op: String, payload: Bytes) -> RpcResult<String>;

    #[method(name = "nucleus_abi")]
    async fn abi(&self, nucleus: NucleusId) -> RpcResult<serde_json::Value>;

    #[method(name = "nucleus_deploy")]
    async fn deploy(&self, tx: Bytes, wasm: Bytes, abi: serde_json::Value) -> RpcResult<Hash>;
}

pub struct Nucleus<P, C, N, B> {
    sender: Sender<(NucleusId, Gluon)>,
    relayer: Relayer<C, N, B>,
    client: Arc<C>,
    pool: Arc<P>,
    node_id: PeerId,
    maybe_validator: Option<AccountId>,
    nucleus_home_dir: PathBuf,
}

impl<P, C, N> Nucleus<P, C, N, P::Block>
where
    P: TransactionPool + Sync + Send + 'static,
    P::Block: sp_runtime::traits::Block + Send + Sync + 'static,
    C: HeaderBackend<P::Block> + ProvideRuntimeApi<P::Block> + Send + Sync + 'static,
    C::Api: NucleusRuntimeApi<P::Block> + 'static,
    N: NetworkService + Send + Sync + 'static,
{
    pub fn new(
        sender: Sender<(NucleusId, Gluon)>,
        relayer: Relayer<C, N, P::Block>,
        client: Arc<C>,
        pool: Arc<P>,
        node_id: PeerId,
        maybe_validator: Option<AccountId>,
        nucleus_home_dir: PathBuf,
    ) -> Self {
        Self {
            sender,
            relayer,
            client,
            pool,
            node_id,
            maybe_validator,
            nucleus_home_dir,
        }
    }

    fn is_nucleus_member(&self, nucleus_id: &NucleusId) -> bool {
        if self.maybe_validator.is_none() {
            return false;
        }
        let best_block = self.client.info().best_hash;
        let api = self.client.runtime_api();
        api.is_member_of(
            best_block,
            &nucleus_id,
            self.maybe_validator.as_ref().unwrap(),
        )
        .unwrap_or(false)
    }

    async fn forward(&self, req: ForwardRequest) -> NucleusResponse {
        self.relayer.forward_to(req).await
    }

    async fn reply(
        &self,
        req: (NucleusId, Gluon),
        rx: Receiver<NucleusResponse>,
    ) -> RpcResult<String> {
        if let Err(e) = self.sender.send(req).await {
            log::error!("Failed to send nucleus request: {:?}", e);
            return Err(NucleusError::node(NUCLEUS_OFFLINE).into());
        }
        match rx.await {
            Ok(Ok(r)) => Ok(hex::encode(r)),
            Ok(Err(e)) => Err(e.into()),
            Err(_) => Err(NucleusError::node(NUCLEUS_OFFLINE).into()),
        }
    }
}

#[async_trait]
impl<P, C, N> NucleusApiServer<BlockHash<P>> for Nucleus<P, C, N, P::Block>
where
    P: TransactionPool + Sync + Send + 'static,
    P::Block: sp_runtime::traits::Block + Send + Sync + 'static,
    C: HeaderBackend<P::Block> + ProvideRuntimeApi<P::Block> + Send + Sync + 'static,
    C::Api: NucleusRuntimeApi<P::Block> + 'static,
    N: NetworkService + Send + Sync + 'static,
{
    async fn post(&self, nucleus: NucleusId, op: String, payload: Bytes) -> RpcResult<String> {
        if self.is_nucleus_member(&nucleus) {
            let (tx, rx) = oneshot::channel();
            let req = (
                nucleus,
                Gluon::PostRequest {
                    endpoint: op,
                    payload: payload.0,
                    reply_to: Some(tx),
                },
            );
            return self.reply(req, rx).await;
        }
        let req = ForwardRequest::Post {
            nucleus_id: nucleus,
            endpoint: op,
            payload: payload.0,
        };
        self.forward(req)
            .await
            .map(|res| hex::encode(res))
            .map_err(|e| e.into())
    }

    async fn get(&self, nucleus: NucleusId, op: String, payload: Bytes) -> RpcResult<String> {
        if self.is_nucleus_member(&nucleus) {
            let (tx, rx) = oneshot::channel();
            let req = (
                nucleus,
                Gluon::GetRequest {
                    endpoint: op,
                    payload: payload.0,
                    reply_to: Some(tx),
                },
            );
            return self.reply(req, rx).await;
        }
        let req = ForwardRequest::Get {
            nucleus_id: nucleus,
            endpoint: op,
            payload: payload.0,
        };
        self.forward(req)
            .await
            .map(|res| hex::encode(res))
            .map_err(|e| e.into())
    }

    async fn abi(&self, nucleus: NucleusId) -> RpcResult<serde_json::Value> {
        if self.is_nucleus_member(&nucleus) {
            let path = self
                .nucleus_home_dir
                .as_path()
                .join(nucleus.to_string())
                .join("wasm/abi.json");
            let abi = tokio::fs::read_to_string(path)
                .await
                .map_err(|_| Into::<ErrorObjectOwned>::into(NucleusError::nucleus_not_found()))?;
            let abi: serde_json::Value = serde_json::from_str(&abi).map_err(|_| {
                Into::<ErrorObjectOwned>::into(NucleusError::abi("Invalid ABI file."))
            })?;
            return Ok(abi);
        }
        let req = ForwardRequest::Abi {
            nucleus_id: nucleus,
        };
        self.forward(req)
            .await
            .map(|res| serde_json::from_slice(&res))
            .map_err(|e| Into::<ErrorObjectOwned>::into(e))?
            .map_err(|_| Into::<ErrorObjectOwned>::into(NucleusError::abi("Invalid ABI file.")))
    }

    async fn deploy(
        &self,
        tx: Bytes,
        wasm: Bytes,
        abi: serde_json::Value,
    ) -> RpcResult<BlockHash<P>> {
        let api = self.client.runtime_api();
        let xt: <P::Block as sp_runtime::traits::Block>::Extrinsic =
            match Decode::decode(&mut &tx[..]) {
                Ok(xt) => xt,
                Err(_) => return Err(NucleusError::params(INVALID_UPGRADE_TX).into()),
            };
        let best_block_hash = self.client.info().best_hash;
        let wasm_info = api
            .resolve_deploy_tx(best_block_hash, xt.clone())
            .ok()
            .flatten()
            .ok_or(Into::<ErrorObjectOwned>::into(NucleusError::params(
                INVALID_UPGRADE_TX,
            )))?;
        PeerId::from_bytes(&wasm_info.node_id.0)
            .ok()
            .filter(|id| self.node_id == *id)
            .ok_or(Into::<ErrorObjectOwned>::into(NucleusError::params(
                INVALID_NODE_ADDRESS,
            )))?;

        let nucleus_info = api
            .get_nucleus_info(best_block_hash, &wasm_info.nucleus_id)
            .inspect_err(|e| {
                log::error!(
                    "Unable to get nucleus info while upgrading wasm, caused by {:?}",
                    e
                )
            })
            .map_err(|_| {
                Into::<ErrorObjectOwned>::into(NucleusError::node(
                    "Unable to get nucleus information from node. Please check the node status.",
                ))
            })?
            .ok_or(Into::<ErrorObjectOwned>::into(
                NucleusError::nucleus_not_found(),
            ))?;
        // TODO
        let abi_vec = abi
            .as_array()
            .ok_or(Into::<ErrorObjectOwned>::into(NucleusError::abi(
                INVALID_NUCLEUS_ABI,
            )))?;
        vrs_nucleus_executor::vm::validate_wasm_abi(&wasm.0, &abi_vec)
            .map_err(|e| Into::<ErrorObjectOwned>::into(e))?;

        let path = self
            .nucleus_home_dir
            .as_path()
            .join(wasm_info.nucleus_id.to_string())
            .join("wasm/");
        let exists = std::fs::exists(&path)
            .expect("make sure the you have right permissions to access the nucleus directory.");
        if !exists {
            std::fs::create_dir_all(&path).expect("Failed to create nucleus directory.");
        }
        std::fs::File::create(path.join(format!("{}.wasm", nucleus_info.wasm_version + 1)))
            .and_then(|mut f| f.write_all(&wasm.0))
            .expect("make sure the you have right permissions to access the nucleus directory.");
        let abi = serde_json::to_vec(&abi).expect("abi should be serializable");
        std::fs::File::create(path.join("abi.json"))
            .and_then(|mut f| f.write_all(&abi))
            .expect("make sure the you have right permissions to access the nucleus directory.");
        let mut submit = self
            .pool
            .submit_and_watch(best_block_hash, TransactionSource::External, xt)
            .await
            .inspect_err(|e| {
                log::error!("Failed to submit nucleus upgrade transaction: {:?}", e);
            })
            .map_err(|_| {
                Into::<ErrorObjectOwned>::into(NucleusError::node("Tx pool not connected."))
            })?;
        loop {
            match submit
                .next()
                .await
                .ok_or(Into::<ErrorObjectOwned>::into(NucleusError::node(
                    "Tx pool not connected.",
                )))? {
                TransactionStatus::InBlock((block, _)) => {
                    return Ok(block);
                }
                TransactionStatus::FinalityTimeout(_)
                | TransactionStatus::Usurped(_)
                | TransactionStatus::Invalid
                | TransactionStatus::Dropped => {
                    break Err(Into::<ErrorObjectOwned>::into(NucleusError::node(
                        "Tx pool rejected.",
                    )));
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
}

mod constants {
    pub const NUCLEUS_OFFLINE: &str = "The nucleus is offline.";
    pub const INVALID_UPGRADE_TX: &str = "The nucleus upgrading transaction is invalid.";
    pub const INVALID_NODE_ADDRESS: &str = "Invalid node address.";
    pub const INVALID_NUCLEUS_ABI: &str = "Invalid nucleus ABI.";
}
