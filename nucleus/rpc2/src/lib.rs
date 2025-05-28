use codec::Decode;
use constants::*;
use futures::prelude::*;
use jsonrpc_core::types::{
    Call, Error as JsonRpcError, ErrorCode, Failure, Id, MethodCall, Output, Params,
    Request as JsonRpcRequest, Response as JsonRpcResponse, Success, Version,
};
use jsonrpc_core::IoHandler;
use sc_network_types::PeerId;
use sc_transaction_pool_api::{BlockHash, TransactionPool, TransactionSource, TransactionStatus};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::Bytes;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::{io::Write, path::PathBuf};
use tokio::sync::{
    mpsc::Sender,
    oneshot::{self, Receiver},
};
use vrs_nucleus_executor::{Gluon, NucleusResponse};
use vrs_nucleus_runtime_api::NucleusApi;
use vrs_primitives::NucleusId;
use warp::{path::FullPath, Buf, Filter, Reply};

pub struct NucleusRpcServerArgs<P, C> {
    pub sender: Sender<(NucleusId, Gluon)>,
    pub client: Arc<C>,
    pub pool: Arc<P>,
    pub node_id: PeerId,
    pub nucleus_home_dir: PathBuf,
}

impl<P, C> Clone for NucleusRpcServerArgs<P, C> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            client: self.client.clone(),
            pool: self.pool.clone(),
            node_id: self.node_id,
            nucleus_home_dir: self.nucleus_home_dir.clone(),
        }
    }
}

fn deserialize_body(body: bytes::Bytes) -> Result<JsonRpcRequest, JsonRpcError> {
    serde_json::from_slice::<JsonRpcRequest>(body.chunk())
        .map_err(|_| JsonRpcError::new(ErrorCode::ParseError))
}

async fn deploy<P, C>(
    method_call: MethodCall,
    context: NucleusRpcServerArgs<P, C>,
) -> Result<Output, Output>
where
    P: TransactionPool + Sync + Send + 'static,
    P::Block: sp_runtime::traits::Block + Send + Sync + 'static,
    C: HeaderBackend<P::Block> + ProvideRuntimeApi<P::Block> + Send + Sync + 'static,
    C::Api: NucleusApi<P::Block> + 'static,
{
    let args = match method_call.params {
        Params::None | Params::Map(_) => None,
        Params::Array(vec) => {
            if vec.len() == 3 {
                let arg0 = vec[0]
                    .as_str()
                    .map(|s| s.trim_start_matches("0x"))
                    .map(|b| hex::decode(b).ok())
                    .flatten()
                    .ok_or(Output::Failure(Failure {
                        jsonrpc: Version::V2,
                        id: method_call.id.clone(),
                        error: JsonRpcError::new(ErrorCode::InvalidParams),
                    }))?;
                let arg1 = vec[1]
                    .as_str()
                    .map(|s| s.trim_start_matches("0x"))
                    .map(|b| hex::decode(b).ok())
                    .flatten()
                    .ok_or(Output::Failure(Failure {
                        jsonrpc: Version::V2,
                        id: method_call.id.clone(),
                        error: JsonRpcError::new(ErrorCode::InvalidParams),
                    }))?;
                let arg2 = vec[2].as_array().cloned().ok_or(Output::Failure(Failure {
                    jsonrpc: Version::V2,
                    id: method_call.id.clone(),
                    error: JsonRpcError::new(ErrorCode::InvalidParams),
                }))?;
                Some((arg0, arg1, arg2))
            } else {
                None
            }
        }
    };
    let (tx, wasm, abi) = args.ok_or(Output::Failure(Failure {
        jsonrpc: Version:V2,
        id: method_call.id.clone(),
        error: JsonRpcError::new(ErrorCode::InvalidParams),
    }))?;
    let api = context.client.runtime_api();
    let xt: <P::Block as sp_runtime::traits::Block>::Extrinsic = match Decode::decode(&mut &tx[..])
    {
        Ok(xt) => xt,
        Err(_) => {
            return Err(Output::Failure(Failure {
                jsonrpc: Version::V2,
                id: method_call.id.clone(),
                error: JsonRpcError::invalid_params(NUCLEUS_UPGRADE_TX_ERR_MSG),
            }));
        }
    };
    let best_block_hash = context.client.info().best_hash;
    let wasm_info = api
        .resolve_deploy_tx(best_block_hash, xt.clone())
        .ok()
        .flatten()
        .ok_or(Output::Failure(Failure {
            jsonrpc: Version::V2,
            id: method_call.id.clone(),
            error: JsonRpcError::invalid_params(NUCLEUS_UPGRADE_TX_ERR_MSG),
        }))?;
    PeerId::from_bytes(&wasm_info.node_id.0)
        .ok()
        .filter(|id| context.node_id == *id)
        .ok_or(Output::Failure(Failure {
            jsonrpc: Version::V2,
            id: method_call.id.clone(),
            error: JsonRpcError::invalid_params(INVALID_NODE_ADDRESS_MSG),
        }))?;

    let nucleus_info = api
        .get_nucleus_info(best_block_hash, wasm_info.nucleus_id.clone())
        .inspect_err(|e| {
            log::error!(
                "Couldn't get nucleus info while upgrading wasm, caused by {:?}",
                e
            )
        })
        .ok()
        .flatten()
        .ok_or(Output::Failure(Failure {
            jsonrpc: Version::V2,
            id: method_call.id.clone(),
            error: JsonRpcError::invalid_params(NUCLEUS_NOT_EXISTS_MSG),
        }))?;
    vrs_nucleus_executor::vm::validate_wasm_abi(&wasm, &abi).map_err(|e| {
        Output::Failure(Failure {
            jsonrpc: Version::V2,
            id: method_call.id.clone(),
            error: JsonRpcError::invalid_params(format!("invalid abi: {}", e)),
        })
    })?;
    let path = context
        .nucleus_home_dir
        .as_path()
        .join(wasm_info.nucleus_id.to_string())
        .join("wasm/");
    // TODO maybe this is an unnecessary check, we are considering to support accepting the wasm upgrade in RPC nodes.
    if !std::fs::exists(&path).expect(
        "fail to check nucleus directory, make sure the you have access right on the directory.",
    ) {
        std::fs::create_dir_all(&path).map_err(|e| {
            Output::Failure(Failure {
                jsonrpc: Version::V2,
                id: method_call.id.clone(),
                error: JsonRpcError::invalid_params(format!(
                    "Couldn't write the wasm file, caused by {:?}",
                    e
                )),
            })
        })?;
    }
    std::fs::File::create(path.join(format!("{}.wasm", nucleus_info.wasm_version + 1)))
        .and_then(|mut f| f.write_all(&wasm))
        .map_err(|e| {
            Output::Failure(Failure {
                jsonrpc: Version::V2,
                id: method_call.id.clone(),
                error: JsonRpcError::invalid_params(format!(
                    "Couldn't write the wasm file, caused by {:?}",
                    e
                )),
            })
        })?;
    let abi = serde_json::to_vec(&abi).expect("abi should be serializable");
    std::fs::File::create(path.join("abi.json"))
        .and_then(|mut f| f.write_all(&abi))
        .map_err(|e| {
            Output::Failure(Failure {
                jsonrpc: Version::V2,
                id: method_call.id.clone(),
                error: JsonRpcError::invalid_params(format!(
                    "Couldn't write abi file, caused by {:?}",
                    e
                )),
            })
        })?;

    let mut submit = context
        .pool
        .submit_and_watch(best_block_hash, TransactionSource::External, xt)
        .await
        .map_err(|e| {
            Output::Failure(Failure {
                jsonrpc: Version::V2,
                id: method_call.id.clone(),
                error: JsonRpcError::invalid_params(format!(
                    "Couldn't accept the transaction, caused by {:?}",
                    e
                )),
            })
        })?;
    loop {
        match submit.next().await.ok_or(Output::Failure(Failure {
            jsonrpc: Version::V2,
            id: method_call.id.clone(),
            error: JsonRpcError::invalid_params("Transaction is not included in the block."),
        }))? {
            TransactionStatus::InBlock((block, _)) => {
                return Ok(Output::Success(Success {
                    jsonrpc: Version::V2,
                    id: method_call.id.clone(),
                    result: serde_json::Value::String(block.to_string()),
                }));
            }
            TransactionStatus::FinalityTimeout(_)
            | TransactionStatus::Usurped(_)
            | TransactionStatus::Invalid
            | TransactionStatus::Dropped => {
                break Err(Output::Failure(Failure {
                    jsonrpc: Version::V2,
                    id: method_call.id.clone(),
                    error: JsonRpcError::invalid_params(
                        "Transaction is not included in the block.",
                    ),
                }));
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

async fn abi<P, C>(
    context: NucleusRpcServerArgs<P, C>,
    nucleus_id: NucleusId,
    call: MethodCall,
) -> Result<Output, Output>
where
    P: TransactionPool + Sync + Send + 'static,
    P::Block: sp_runtime::traits::Block + Send + Sync + 'static,
    C: HeaderBackend<P::Block> + ProvideRuntimeApi<P::Block> + Send + Sync + 'static,
    C::Api: NucleusApi<P::Block> + 'static,
{
    let path = context
        .nucleus_home_dir
        .as_path()
        .join(nucleus_id.to_string())
        .join("wasm/abi.json");
    let abi = tokio::fs::read_to_string(path).await.map_err(|e| {
        Output::Failure(Failure {
            jsonrpc: Version::V2,
            id: call.id.clone(),
            error: JsonRpcError::invalid_params(format!(
                "Couldn't read the abi file, caused by {:?}",
                e
            )),
        })
    })?;
    let abi: serde_json::Value = serde_json::from_str(&abi).map_err(|e| {
        Output::Failure(Failure {
            jsonrpc: Version::V2,
            id: call.id.clone(),
            error: JsonRpcError::invalid_params(format!(
                "Couldn't parse the abi file, caused by {:?}",
                e
            )),
        })
    })?;
    Ok(Output::Success(Success {
        jsonrpc: Version::V2,
        id: call.id.clone(),
        result: abi,
    }))
}

async fn make_call<P, C>(
    context: NucleusRpcServerArgs<P, C>,
    nucleus_id: NucleusId,
    call: MethodCall,
) -> Result<Output, Output>
where
    P: TransactionPool + Sync + Send + 'static,
    P::Block: sp_runtime::traits::Block + Send + Sync + 'static,
    C: HeaderBackend<P::Block> + ProvideRuntimeApi<P::Block> + Send + Sync + 'static,
    C::Api: NucleusApi<P::Block> + 'static,
{
    let (tx, rx) = oneshot::channel();
    match call.method.as_str() {
        // "deploy" => return deploy(call, context).await,
        "abi" => return abi(context, nucleus_id, call).await,
        _ => {}
    }
    let (ty, method) = call
        .method
        .as_str()
        .split_once("_")
        .ok_or(Output::Failure(Failure {
            jsonrpc: Version::V2,
            id: call.id.clone(),
            error: JsonRpcError::new(ErrorCode::MethodNotFound),
        }))?;
    let args = match call.params {
        Params::None | Params::Map(_) => None,
        Params::Array(vec) => vec
            .first()
            .map(|v| v.as_str())
            .flatten()
            .map(|s| s.trim_start_matches("0x"))
            .and_then(|s| hex::decode(s).ok()),
    };
    let args = args.ok_or(Output::Failure(Failure {
        jsonrpc: Version::V2,
        id: call.id.clone(),
        error: JsonRpcError::new(ErrorCode::InvalidParams),
    }))?;
    let payload = match ty {
        "post" => Some((
            nucleus_id.clone(),
            Gluon::PostRequest {
                endpoint: method.to_string(),
                payload: args,
                reply_to: Some(tx),
            },
        )),
        "get" => Some((
            nucleus_id.clone(),
            Gluon::PostRequest {
                endpoint: method.to_string(),
                payload: args,
                reply_to: Some(tx),
            },
        )),
        _ => None,
    };
    let req = payload.ok_or(Output::Failure(Failure {
        jsonrpc: Version::V2,
        id: call.id.clone(),
        error: JsonRpcError::new(ErrorCode::MethodNotFound),
    }))?;

    tokio::select! {
        v = context.sender.send(req) => {
            if v.is_err() {
                return Err(Output::Failure(Failure {
                    jsonrpc: Version::V2,
                    id: call.id.clone(),
                    error: JsonRpcError {
                        code: ErrorCode::ServerError(NUCLEUS_OFFLINE_CODE),
                        message: NUCLEUS_OFFLINE_MSG.to_string(),
                        data: None,
                    },
                }));
            }
        }
        _ = tokio::time::sleep(std::time::Duration::from_secs(2)) => {
            return Err(Output::Failure(Failure {
                    jsonrpc: Version::V2,
                    id: call.id.clone(),
                    error: JsonRpcError {
                        code: ErrorCode::ServerError(NUCLEUS_OFFLINE_CODE),
                        message: NUCLEUS_OFFLINE_MSG.to_string(),
                        data: None,
                    },
                }));
        }
    }
    tokio::select! {
        reply = rx => {
            match reply {
                Ok(Ok(r)) => Ok(Output::Success(Success {
                    jsonrpc: Version::V2,
                    id: call.id.clone(),
                    result: serde_json::Value::String(hex::encode(r)),
                })),
                Ok(Err(e)) => Err(Output::Failure(Failure {
                    jsonrpc: Version::V2,
                    id: call.id.clone(),
                    error: JsonRpcError {
                        code: ErrorCode::ServerError(e.0.into()),
                        message: e.1,
                        data: None,
                    },
                })),
                Err(_) => Err(Output::Failure(Failure {
                    jsonrpc: Version::V2,
                    id: call.id.clone(),
                    error: JsonRpcError {
                        code: ErrorCode::ServerError(NUCLEUS_OFFLINE_CODE),
                        message: NUCLEUS_OFFLINE_MSG.to_string(),
                        data: None,
                    },
                })),
            }
        }
        _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {
            Err(Output::Failure(Failure {
                    jsonrpc: Version::V2,
                    id: call.id.clone(),
                    error: JsonRpcError {
                        code: ErrorCode::ServerError(NUCLEUS_OFFLINE_CODE),
                        message: NUCLEUS_OFFLINE_MSG.to_string(),
                        data: None,
                    },
                }))
        }
    }
}

fn with_context<P, C>(
    args: NucleusRpcServerArgs<P, C>,
) -> impl Filter<Extract = (NucleusRpcServerArgs<P, C>,), Error = std::convert::Infallible> + Clone
where
    P: TransactionPool + Sync + Send + 'static,
    P::Block: sp_runtime::traits::Block + Send + Sync + 'static,
    C: HeaderBackend<P::Block> + ProvideRuntimeApi<P::Block> + Send + Sync + 'static,
    C::Api: NucleusApi<P::Block> + 'static,
{
    warp::any().map(move || args.clone())
}

fn with_nucleus_path(
    args: PathBuf,
) -> impl Filter<Extract = (PathBuf,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || args.clone())
}

pub async fn start_nucleus_rpc<P, C>(args: NucleusRpcServerArgs<P, C>)
where
    P: TransactionPool + Sync + Send + 'static,
    P::Block: sp_runtime::traits::Block + Send + Sync + 'static,
    C: HeaderBackend<P::Block> + ProvideRuntimeApi<P::Block> + Send + Sync + 'static,
    C::Api: NucleusApi<P::Block> + 'static,
{
    let jsonrpc = warp::post()
        .and(warp::path::param())
        .and(warp::path::end())
        .and(warp::body::content_length_limit(10 * 1024 * 1024))
        .and(warp::body::bytes())
        .and(with_context(args.clone()))
        .then(
            |nucleus_id: String, body: bytes::Bytes, context: NucleusRpcServerArgs<P, C>| async move {
                let nucleus_id = NucleusId::from_str(&nucleus_id);
                if nucleus_id.is_err() {
                    return warp::reply::json(&JsonRpcResponse::from(
                        JsonRpcError::new(ErrorCode::InvalidParams),
                        None,
                    ))
                    .into_response();
                }
                let nucleus_id = nucleus_id.unwrap();
                match deserialize_body(body) {
                    Ok(body) => {
                        let calls = match body {
                            JsonRpcRequest::Single(call) => vec![call],
                            JsonRpcRequest::Batch(calls) => calls,
                        };
                        let mut replies = vec![];
                        for call in calls.into_iter() {
                            let context = context.clone();
                            let nucleus_id = nucleus_id.clone();
                            match call {
                                Call::MethodCall(method_call) => {
                                    let result = async move {
                                        match make_call(context, nucleus_id, method_call).await {
                                            Ok(output) => output,
                                            Err(output) => output,
                                        }
                                    }
                                    .boxed();
                                    replies.push(result);
                                }
                                Call::Notification(notification) => {}
                                Call::Invalid { id } => {
                                    let r = async move {
                                        Output::Failure(Failure {
                                            jsonrpc: Version::V2,
                                            id,
                                            error: JsonRpcError::new(ErrorCode::InvalidRequest),
                                        })
                                    }
                                    .boxed();
                                    replies.push(r);
                                }
                            }
                        }
                        let replies = future::join_all(replies).await;
                        if replies.is_empty() {
                            warp::reply::Response::default()
                        } else if replies.len() == 1 {
                            warp::reply::json(&replies[0]).into_response()
                        } else {
                            let batch_response = JsonRpcResponse::Batch(replies);
                            warp::reply::json(&batch_response).into_response()
                        }
                    }
                    Err(e) => warp::reply::json(&JsonRpcResponse::from(e, None)).into_response(),
                }
            },
        );
    // TODO config the port and bind_addr
    let stdout = warp::path!(String / "logs")
        .and(warp::get())
        .and(with_nucleus_path(args.nucleus_home_dir.clone()))
        .and_then(|nucleus_id: String, home: PathBuf| async move {
            let nucleus_id =
                NucleusId::from_str(&nucleus_id).map_err(|_| warp::reject::not_found())?;
            let path = home
                .as_path()
                .join(nucleus_id.to_string())
                .join("stdout.log");
            tokio::fs::read_to_string(&path)
                .await
                .map_err(|_| warp::reject::not_found())
        });
    warp::serve(stdout.or(jsonrpc))
        .run(([0, 0, 0, 0], 9955))
        .await;
}

// TODO
mod constants {
    pub const NUCLEUS_OFFLINE_CODE: i64 = -40001;
    pub const NUCLEUS_OFFLINE_MSG: &str = "The nucleus is offline.";
    pub const NUCLEUS_TIMEOUT_CODE: i64 = -40002;
    pub const NUCLEUS_TIMEOUT_MSG: &str = "The nucleus is not responding.";
    pub const TX_POOL_ERROR_CODE: i64 = -40010;
    pub const NUCLEUS_UPGRADE_TX_ERR_CODE: i64 = -40011;
    pub const NUCLEUS_UPGRADE_TX_ERR_MSG: &str = "The nucleus upgrading transaction is invalid.";
    pub const INVALID_NODE_ADDRESS_CODE: i64 = -40012;
    pub const INVALID_NODE_ADDRESS_MSG: &str = "Invalid node address.";
    pub const NUCLEUS_NOT_EXISTS_CODE: i64 = -40014;
    pub const NUCLEUS_NOT_EXISTS_MSG: &str = "Nucleus not exists.";
    pub const OS_ERR_CODE: i64 = -42000;
}
