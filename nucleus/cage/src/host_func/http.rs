use crate::{
    mem,
    runtime::{ComponentProvider, ContextAware},
    Runtime,
};
use bytes::Bytes;
use codec::{Decode, Encode};
use futures::FutureExt;
use http_body_util::Full;
use hyper::{Method, Request, Response, Uri};
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use vrs_core_sdk::{error::RuntimeError, http::*, CallResult, BUFFER_LEN, NO_MORE_DATA};
use vrs_primitives::NucleusId;
use wasmtime::{Caller, Engine, FuncType, Val, ValType};

impl ComponentProvider<HttpCallRegister> for Runtime {
    fn get_component(&self) -> std::sync::Arc<HttpCallRegister> {
        self.http.clone()
    }
}

// TODO the id should be unique
pub fn new_http_manager() -> (HttpCallRegister, HttpCallExecutor) {
    let (tx, rx) = mpsc::unbounded_channel();
    (HttpCallRegister::new(tx), HttpCallExecutor::new(rx))
}

#[derive(Debug)]
pub struct HttpRequestWithCallback {
    pub nucleus_id: NucleusId,
    pub req_id: u64,
    pub request: Request<Full<Bytes>>,
}

#[derive(Debug)]
pub struct HttpResponseWithCallback {
    pub nucleus_id: NucleusId,
    pub req_id: u64,
    pub response: Response<Full<Bytes>>,
}

#[derive(Debug)]
pub struct HttpCallExecutor {
    rx: UnboundedReceiver<HttpRequestWithCallback>,
}

impl HttpCallExecutor {
    fn new(rx: UnboundedReceiver<HttpRequestWithCallback>) -> Self {
        HttpCallExecutor { rx }
    }

    pub(crate) fn poll<'a>(
        &'a mut self,
    ) -> impl futures::Future<Output = Option<HttpResponseWithCallback>> + 'a {
        self.rx.recv().then(|req| async move {
            match req {
                Some(req) => {
                    let uri = req.request.uri().clone();
                    // TODO make the request
                    Some(HttpResponseWithCallback {
                        nucleus_id: req.nucleus_id,
                        req_id: req.req_id,
                        response: Response::new(Full::from(Bytes::new())),
                    })
                }
                None => None,
            }
        })
    }

    async fn connect<S, C, H, F>(url: Uri, handshake: H) -> std::io::Result<(S, C)>
    where
        H: FnOnce(TcpStream) -> F,
        F: futures::Future<Output = std::io::Result<(S, C)>> + 'static,
    {
        let host = url.host().expect("already check");
        let port = url.port_u16().unwrap_or(443);
        let addr = format!("{}:{}", host, port);
        let stream = TcpStream::connect(addr)
            .await
            .map_err(|_| std::io::Error::last_os_error())?;
        handshake(stream).await
    }
}

#[derive(Debug)]
pub struct HttpCallRegister {
    id: AtomicU64,
    tx: UnboundedSender<HttpRequestWithCallback>,
}

impl HttpCallRegister {
    // TODO the id should be unique even the node reboot
    fn new(tx: UnboundedSender<HttpRequestWithCallback>) -> Self {
        HttpCallRegister {
            id: AtomicU64::new(0),
            tx,
        }
    }

    pub(crate) fn enqueue_request(
        &self,
        origin: NucleusId,
        req: HttpRequest,
    ) -> Result<u64, RuntimeError> {
        let uri = req
            .head
            .uri
            .parse::<Uri>()
            .map_err(|e| RuntimeError::HttpError(e.to_string()))?;
        (uri.scheme_str() == Some("https"))
            .then(|| ())
            .ok_or(RuntimeError::HttpError("only support https".to_string()))?;
        let mut builder = Request::builder()
            .method(from_decode_method(req.head.method))
            .uri(uri);
        for (key, value) in req.head.headers {
            builder = builder.header(key, value);
        }
        let request = builder
            .body(Full::from(req.body))
            .map_err(|e| RuntimeError::HttpError(e.to_string()))?;
        let id = self.id.fetch_add(1, Ordering::Relaxed);
        self.tx
            .send(HttpRequestWithCallback {
                nucleus_id: origin,
                req_id: id,
                request,
            })
            .map_err(|e| RuntimeError::HttpError(e.to_string()))?;
        Ok(id)
    }
}

fn from_decode_method(method: HttpMethod) -> Method {
    match method {
        HttpMethod::Options => Method::OPTIONS,
        HttpMethod::Get => Method::GET,
        HttpMethod::Post => Method::POST,
        HttpMethod::Put => Method::PUT,
        HttpMethod::Delete => Method::DELETE,
        HttpMethod::Head => Method::HEAD,
        HttpMethod::Trace => Method::TRACE,
        HttpMethod::Connect => Method::CONNECT,
        HttpMethod::Patch => Method::PATCH,
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
pub(crate) fn enqueue_http_request<R>(
    mut caller: Caller<'_, R>,
    params: &[Val],
    result: &mut [Val],
) -> anyhow::Result<()>
where
    R: ContextAware + ComponentProvider<HttpCallRegister>,
{
    result[0] = Val::I32(NO_MORE_DATA);
    let r_ptr = params[2].unwrap_i32();
    if caller.data().read_only() {
        let return_value = CallResult::<()>::Err(RuntimeError::WriteIsNotAllowInGetMethod);
        let bytes = return_value.encode();
        assert!(bytes.len() <= BUFFER_LEN);
        mem::write_bytes_to_memory(&mut caller, r_ptr, &bytes).expect("write to wasm failed");
        return Ok(());
    }
    let req_ptr = params[0].unwrap_i32();
    let req_len = params[1].unwrap_i32();
    let req =
        mem::read_bytes_from_memory(&mut caller, req_ptr, req_len).expect("read from wasm failed");
    let request: HttpRequest = Decode::decode(&mut req.as_slice()).expect("decode request failed");
    let http_manager = &caller.data().get_component();
    let nucleus_id = caller.data().get_nucleus_id().clone();
    let return_value = http_manager.enqueue_request(nucleus_id, request);
    let bytes = return_value.encode();
    assert!(bytes.len() <= BUFFER_LEN);
    mem::write_bytes_to_memory(&mut caller, r_ptr, &bytes).expect("write to wasm failed");
    Ok(())
}
