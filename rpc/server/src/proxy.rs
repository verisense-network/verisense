use anyhow::Result;
use futures::{FutureExt, SinkExt, StreamExt};
use jsonrpc_core::types::{Error as JsonRpcError, ErrorCode, Response as JsonRpcResponse};
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::protocol::Message;
use warp::filters::ws::{Message as WarpMessage, WebSocket};
use warp::http::HeaderMap;
use warp::{reply::Response, Reply};

pub async fn forward_request(
    headers: HeaderMap,
    body: bytes::Bytes,
    port: u16,
) -> Result<Response, Response> {
    let client = reqwest::Client::new();
    let mut req_builder = client.post(format!("http://127.0.0.1:{}", port));

    for (name, value) in headers.iter() {
        if name.as_str().to_lowercase() != "host" {
            req_builder = req_builder.header(name.as_str(), value);
        }
    }
    let backend = req_builder.body(body).send().await.map_err(|_| {
        warp::reply::json(&JsonRpcResponse::from(
            JsonRpcError::new(ErrorCode::InternalError),
            None,
        ))
        .into_response()
    })?;
    let status = backend.status();
    let headers = backend.headers().clone();
    let body = backend.bytes().await.map_err(|_| {
        warp::reply::json(&JsonRpcResponse::from(
            JsonRpcError::new(ErrorCode::InternalError),
            None,
        ))
        .into_response()
    })?;
    let mut response = Response::new(body.into());
    *response.status_mut() = reqwest::StatusCode::from_u16(status.as_u16()).unwrap();
    for (name, value) in headers {
        if let Some(name) = name {
            response.headers_mut().insert(name, value.clone());
        }
    }
    Ok(response)
}

pub async fn relay_ws_connection(ws: WebSocket, port: u16) {
    let (ws_tx, mut ws_rx) = ws.split();

    let (tx, rx) = mpsc::unbounded_channel();
    let rx = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);

    tokio::task::spawn(rx.forward(ws_tx).map(|_| ()));

    match tokio_tungstenite::connect_async(&format!("ws://127.0.0.1:{}", port)).await {
        Ok((target_ws_stream, _)) => {
            let (mut target_write, mut target_read) = target_ws_stream.split();
            tokio::spawn(async move {
                while let Some(result) = ws_rx.next().await {
                    match result {
                        Ok(msg) => {
                            if msg.is_close() {
                                break;
                            }
                            let forwarded_msg = if msg.is_text() {
                                if let Ok(txt) = msg.to_str() {
                                    Message::Text(txt.to_string())
                                } else {
                                    continue;
                                }
                            } else if msg.is_binary() {
                                Message::Binary(msg.as_bytes().to_vec())
                            } else if msg.is_ping() {
                                Message::Ping(msg.as_bytes().to_vec())
                            } else if msg.is_pong() {
                                Message::Pong(msg.as_bytes().to_vec())
                            } else {
                                continue;
                            };
                            if let Err(_) = target_write.send(forwarded_msg).await {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
            });

            tokio::spawn(async move {
                while let Some(result) = target_read.next().await {
                    match result {
                        Ok(msg) => {
                            let forwarded_msg = match msg {
                                Message::Text(txt) => WarpMessage::text(txt),
                                Message::Binary(bin) => WarpMessage::binary(bin),
                                Message::Ping(data) => WarpMessage::ping(data),
                                Message::Pong(data) => WarpMessage::pong(data),
                                Message::Close(_) => WarpMessage::close(),
                                _ => continue,
                            };
                            if let Err(_) = tx.send(Ok(forwarded_msg)) {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
            });
        }
        Err(_) => {
            let _ = tx.send(Ok(WarpMessage::text(
                serde_json::to_string(&JsonRpcResponse::from(
                    JsonRpcError::new(ErrorCode::InternalError),
                    None,
                ))
                .unwrap(),
            )));
        }
    }
}
