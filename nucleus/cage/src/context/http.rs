use super::*;
use bytes::Bytes;
use codec::{Decode, Encode};
use http_body_util::{BodyExt, Full};
use hyper::{
    header::{HeaderMap, HeaderValue},
    Request, Response, Uri,
};
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use std::sync::atomic::{AtomicU64, Ordering};
use vrs_core_sdk::{error::RuntimeError, http::*, CallResult, BUFFER_LEN, NO_MORE_DATA};
use wasmtime::{Caller, FuncType, Val, ValType};

#[derive(Debug)]
pub struct HttpManager {
    id: AtomicU64,
}

impl HttpManager {
    pub(crate) fn new() -> Self {
        HttpManager {
            id: AtomicU64::new(0),
        }
    }

    pub(crate) fn enqueue_request(
        &self,
        origin: NucleusId,
        req: HttpRequest,
    ) -> Result<u64, RuntimeError> {
        let id = self.id.fetch_add(1, Ordering::Relaxed);
        let request = Request::builder()
            .method(req.head.method.into())
            .uri(req.head.uri)
            .body(Full::from(req.body))
            .map_err(|e| RuntimeError::HttpError(e.to_string()))?;
        // TODO save this future then poll it in cage
        let future = async {
            // let host = request.uri().host().expect("uri has no host");
            // let port = request.uri().port_u16().unwrap_or(80);

            // let stream = TcpStream::connect((host, port)).await.unwrap();
            // let io = TokioIo::new(stream);

            // let (mut sender, conn) = Builder::new()
            //     .preserve_header_case(true)
            //     .title_case_headers(true)
            //     .handshake(io)
            //     .await?;
            // tokio::task::spawn(async move {
            //     if let Err(err) = conn.await {
            //         println!("Connection failed: {:?}", err);
            //     }
            // });

            // let resp = sender.send_request(req).await?;
        };
        // TODO
        Ok(id)
    }
}

/// the signature of this host function is:
///
/// ```
/// fn http_request(req_ptr: *const u8, req_len: u32, return_ptr: *mut u8) -> i32;
/// ```
pub(crate) fn http_request_signature(engine: &Engine) -> FuncType {
    FuncType::new(
        engine,
        [ValType::I32, ValType::I32, ValType::I32],
        [ValType::I32],
    )
}

/// the signature of this host function is:
///
/// ```
/// fn http_request(req_ptr: *const u8, req_len: u32, return_ptr: *mut u8) -> i32;
/// ```
pub(crate) fn enqueue_http_request(
    mut caller: Caller<'_, Context>,
    params: &[Val],
    result: &mut [Val],
) -> anyhow::Result<()> {
    result[0] = Val::I32(NO_MORE_DATA);
    let r_ptr = params[2].unwrap_i32();
    if caller.data().is_get_method() {
        let return_value = CallResult::<()>::Err(RuntimeError::WriteIsNotAllowInGetMethod);
        let bytes = return_value.encode();
        assert!(bytes.len() <= BUFFER_LEN);
        Context::write_bytes_to_memory(&mut caller, r_ptr, &bytes).expect("write to wasm failed");
        return Ok(());
    }
    let req_ptr = params[0].unwrap_i32();
    let req_len = params[1].unwrap_i32();
    let req = Context::read_bytes_from_memory(&mut caller, req_ptr, req_len)
        .expect("read from wasm failed");
    let request: HttpRequest = Decode::decode(&mut req.as_slice()).expect("decode request failed");
    let http_manager = &caller.data().http;
    let nucleus_id = caller.data().id.clone();
    let return_value = http_manager.enqueue_request(nucleus_id, request);
    let bytes = return_value.encode();
    assert!(bytes.len() <= BUFFER_LEN);
    Context::write_bytes_to_memory(&mut caller, r_ptr, &bytes).expect("write to wasm failed");
    Ok(())
}
