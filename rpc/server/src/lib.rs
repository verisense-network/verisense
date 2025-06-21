mod proxy;

use constants::*;
use futures::prelude::*;
use jsonrpc_core::types::{
    Call, Error as JsonRpcError, ErrorCode, Failure, MethodCall, Output, Params,
    Request as JsonRpcRequest, Response as JsonRpcResponse, Success, Version,
};
use sc_network_types::PeerId;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::{mpsc::Sender, oneshot};
use vrs_nucleus_executor::{Gluon, NucleusError};
use vrs_nucleus_runtime_api::NucleusRuntimeApi;
use vrs_primitives::NucleusId;
use warp::{Buf, Filter, Reply};

pub struct NucleusRpcServerArgs<P, C> {
    pub sender: Sender<(NucleusId, Gluon)>,
    pub client: Arc<C>,
    pub pool: Arc<P>,
    pub node_id: PeerId,
    pub nucleus_home_dir: PathBuf,
    pub sys_rpc_port: u16,
    pub entry_rpc_port: u16,
}

impl<P, C> Clone for NucleusRpcServerArgs<P, C> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            client: self.client.clone(),
            pool: self.pool.clone(),
            node_id: self.node_id,
            nucleus_home_dir: self.nucleus_home_dir.clone(),
            sys_rpc_port: self.sys_rpc_port,
            entry_rpc_port: self.entry_rpc_port,
        }
    }
}

fn deserialize_body(body: bytes::Bytes) -> Result<JsonRpcRequest, JsonRpcError> {
    serde_json::from_slice::<JsonRpcRequest>(body.chunk())
        .map_err(|_| JsonRpcError::new(ErrorCode::ParseError))
}

fn deserialize_text(text: &str) -> Result<JsonRpcRequest, JsonRpcError> {
    serde_json::from_str::<JsonRpcRequest>(text)
        .map_err(|_| JsonRpcError::new(ErrorCode::ParseError))
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
    C::Api: NucleusRuntimeApi<P::Block> + 'static,
{
    let path = context
        .nucleus_home_dir
        .as_path()
        .join(nucleus_id.to_string())
        .join("wasm/abi.json");
    let abi = tokio::fs::read_to_string(path).await.map_err(|e| {
        Output::Failure(Failure {
            jsonrpc: Some(Version::V2),
            id: call.id.clone(),
            error: JsonRpcError::invalid_params(format!(
                "Couldn't read the abi file, caused by {:?}",
                e
            )),
        })
    })?;
    let abi: serde_json::Value = serde_json::from_str(&abi).map_err(|e| {
        Output::Failure(Failure {
            jsonrpc: Some(Version::V2),
            id: call.id.clone(),
            error: JsonRpcError::invalid_params(format!(
                "Couldn't parse the abi file, caused by {:?}",
                e
            )),
        })
    })?;
    Ok(Output::Success(Success {
        jsonrpc: Some(Version::V2),
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
    C::Api: NucleusRuntimeApi<P::Block> + 'static,
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
            jsonrpc: Some(Version::V2),
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
        jsonrpc: Some(Version::V2),
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
        jsonrpc: Some(Version::V2),
        id: call.id.clone(),
        error: JsonRpcError::new(ErrorCode::MethodNotFound),
    }))?;

    if let Err(_) = context.sender.send(req).await {
        return Err(Output::Failure(Failure {
            jsonrpc: Some(Version::V2),
            id: call.id.clone(),
            error: NucleusError::node(NUCLEUS_OFFLINE).into(),
        }));
    }

    match rx.await {
        Ok(Ok(r)) => Ok(Output::Success(Success {
            jsonrpc: Some(Version::V2),
            id: call.id.clone(),
            result: serde_json::Value::String(hex::encode(r)),
        })),
        Ok(Err(e)) => Err(Output::Failure(Failure {
            jsonrpc: Some(Version::V2),
            id: call.id.clone(),
            error: e.into(),
        })),
        Err(_) => Err(Output::Failure(Failure {
            jsonrpc: Some(Version::V2),
            id: call.id.clone(),
            error: NucleusError::node(NUCLEUS_OFFLINE).into(),
        })),
    }
}

fn with_context<P, C>(
    args: NucleusRpcServerArgs<P, C>,
) -> impl Filter<Extract = (NucleusRpcServerArgs<P, C>,), Error = std::convert::Infallible> + Clone
where
    P: TransactionPool + Sync + Send + 'static,
    P::Block: sp_runtime::traits::Block + Send + Sync + 'static,
    C: HeaderBackend<P::Block> + ProvideRuntimeApi<P::Block> + Send + Sync + 'static,
    C::Api: NucleusRuntimeApi<P::Block> + 'static,
{
    warp::any().map(move || args.clone())
}

fn with_nucleus_path(
    args: PathBuf,
) -> impl Filter<Extract = (PathBuf,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || args.clone())
}

async fn ws_handshake<P, C>(
    nucleus_id: String,
    ws: warp::ws::Ws,
    context: NucleusRpcServerArgs<P, C>,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection>
where
    P: TransactionPool + Sync + Send + 'static,
    P::Block: sp_runtime::traits::Block + Send + Sync + 'static,
    C: HeaderBackend<P::Block> + ProvideRuntimeApi<P::Block> + Send + Sync + 'static,
    C::Api: NucleusRuntimeApi<P::Block> + 'static,
{
    let nucleus_id = NucleusId::from_str(&nucleus_id);
    match nucleus_id {
        Ok(id) => Ok(ws.on_upgrade(move |socket| ws_jsonrpc(socket, id, context))),
        Err(_) => Err(warp::reject::not_found()),
    }
}

async fn ws_jsonrpc<P, C>(
    socket: warp::ws::WebSocket,
    nucleus_id: NucleusId,
    context: NucleusRpcServerArgs<P, C>,
) where
    P: TransactionPool + Sync + Send + 'static,
    P::Block: sp_runtime::traits::Block + Send + Sync + 'static,
    C: HeaderBackend<P::Block> + ProvideRuntimeApi<P::Block> + Send + Sync + 'static,
    C::Api: NucleusRuntimeApi<P::Block> + 'static,
{
    let (mut ws_tx, mut ws_rx) = socket.split();
    tokio::spawn(async move {
        while let Some(result) = ws_rx.next().await {
            match result {
                Ok(msg) => {
                    if msg.is_close() {
                        break;
                    }
                    if msg.is_text() {
                        match deserialize_text(&msg.to_str().unwrap()) {
                            Ok(req) => {
                                let calls = match req {
                                    JsonRpcRequest::Single(call) => vec![call],
                                    JsonRpcRequest::Batch(calls) => calls,
                                };
                                for call in calls.into_iter() {
                                    let context = context.clone();
                                    let nucleus_id = nucleus_id.clone();
                                    match call {
                                        Call::MethodCall(method_call) => {
                                            let output =
                                                match make_call(context, nucleus_id, method_call)
                                                    .await
                                                {
                                                    Ok(output) => output,
                                                    Err(output) => output,
                                                };
                                            if let Err(_) = ws_tx
                                                .send(warp::ws::Message::text(
                                                    serde_json::to_string(&output).unwrap(),
                                                ))
                                                .await
                                            {
                                                let _ = ws_tx.close().await;
                                                return;
                                            }
                                        }
                                        Call::Notification(_notification) => {}
                                        Call::Invalid { id } => {
                                            let output = Output::Failure(Failure {
                                                jsonrpc: Some(Version::V2),
                                                id,
                                                error: JsonRpcError::new(ErrorCode::InvalidRequest),
                                            });
                                            if let Err(_) = ws_tx
                                                .send(warp::ws::Message::text(
                                                    serde_json::to_string(&output).unwrap(),
                                                ))
                                                .await
                                            {
                                                let _ = ws_tx.close().await;
                                                return;
                                            }
                                        }
                                    }
                                }
                            }
                            Err(_) => {
                                if let Err(_) = ws_tx
                                    .send(warp::ws::Message::text(
                                        serde_json::to_string(&JsonRpcResponse::from(
                                            JsonRpcError::new(ErrorCode::ParseError),
                                            None,
                                        ))
                                        .unwrap(),
                                    ))
                                    .await
                                {
                                    let _ = ws_tx.close().await;
                                    return;
                                }
                            }
                        }
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
        let _ = ws_tx.close().await;
    });
}

async fn jsonrpc<P, C>(
    nucleus_id: String,
    body: bytes::Bytes,
    context: NucleusRpcServerArgs<P, C>,
) -> warp::reply::Response
where
    P: TransactionPool + Sync + Send + 'static,
    P::Block: sp_runtime::traits::Block + Send + Sync + 'static,
    C: HeaderBackend<P::Block> + ProvideRuntimeApi<P::Block> + Send + Sync + 'static,
    C::Api: NucleusRuntimeApi<P::Block> + 'static,
{
    let Ok(nucleus_id) = NucleusId::from_str(&nucleus_id) else {
        return warp::reply::json(&JsonRpcResponse::from(
            JsonRpcError::new(ErrorCode::InvalidParams),
            None,
        ))
        .into_response();
    };
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
                    Call::Notification(_notification) => {}
                    Call::Invalid { id } => {
                        let r = async move {
                            Output::Failure(Failure {
                                jsonrpc: Some(Version::V2),
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
}

pub async fn start_nucleus_rpc<P, C>(args: NucleusRpcServerArgs<P, C>)
where
    P: TransactionPool + Sync + Send + 'static,
    P::Block: sp_runtime::traits::Block + Send + Sync + 'static,
    C: HeaderBackend<P::Block> + ProvideRuntimeApi<P::Block> + Send + Sync + 'static,
    C::Api: NucleusRuntimeApi<P::Block> + 'static,
{
    let sys_proxy = warp::path::end()
        .and(warp::post())
        .and(warp::header::headers_cloned())
        .and(warp::body::content_length_limit(10 * 1024 * 1024))
        .and(warp::body::bytes())
        .then(
            move |headers: warp::http::HeaderMap, body: bytes::Bytes| async move {
                match crate::proxy::forward_request(headers, body, args.sys_rpc_port).await {
                    Ok(r) => r,
                    Err(e) => e,
                }
            },
        );

    let ws_sys_proxy = warp::path::end()
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            ws.on_upgrade(move |socket| {
                crate::proxy::relay_ws_connection(socket, args.sys_rpc_port)
            })
        });

    let jsonrpc = warp::path!(String)
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::content_length_limit(10 * 1024 * 1024))
        .and(warp::body::bytes())
        .and(with_context(args.clone()))
        .then(jsonrpc);

    let ws_jsonrpc = warp::path!(String)
        .and(warp::path::end())
        .and(warp::ws())
        .and(with_context(args.clone()))
        .and_then(ws_handshake);

    let stdout = warp::path!(String / "logs")
        .and(warp::get())
        .and(warp::path::end())
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

    // TODO config the port and cors
    warp::serve(
        stdout
            .or(jsonrpc)
            .or(ws_jsonrpc)
            .or(sys_proxy)
            .or(ws_sys_proxy),
    )
    .run(([0, 0, 0, 0], args.entry_rpc_port))
    .await;
}

// TODO
mod constants {
    // pub const NUCLEUS_OFFLINE_CODE: i64 = -40001;
    pub const NUCLEUS_OFFLINE: &str = "The nucleus is offline.";
    // pub const NUCLEUS_TIMEOUT_CODE: i64 = -40002;
    // pub const NUCLEUS_TIMEOUT_MSG: &str = "The nucleus is not responding.";
    // pub const TX_POOL_ERROR_CODE: i64 = -40010;
    // pub const NUCLEUS_UPGRADE_TX_ERR_CODE: i64 = -40011;
    // pub const NUCLEUS_UPGRADE_TX_ERR_MSG: &str = "The nucleus upgrading transaction is invalid.";
    // pub const INVALID_NODE_ADDRESS_CODE: i64 = -40012;
    // pub const INVALID_NODE_ADDRESS_MSG: &str = "Invalid node address.";
    // pub const NUCLEUS_NOT_EXISTS_CODE: i64 = -40014;
    // pub const NUCLEUS_NOT_EXISTS_MSG: &str = "Nucleus not exists.";
    // pub const OS_ERR_CODE: i64 = -42000;
}
