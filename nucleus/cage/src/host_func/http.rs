use crate::{
    mem,
    runtime::{ComponentProvider, ContextAware},
    Runtime,
};
use bytes::Bytes;
use codec::{Decode, Encode};
use futures::FutureExt;
use http_body_util::{BodyExt, Full};
use hyper::{body::Incoming, Method, Request, Response, Uri};
use std::sync::atomic::{AtomicU64, Ordering};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::{
    net::TcpStream,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
};
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
    pub response: CallResult<HttpResponse>,
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
        self.rx
            .recv()
            .then(|req| async move {
                match req {
                    Some(req) => {
                        let uri = req.request.uri().clone();
                        match Self::connect(uri, local_handshake).await {
                            Ok((mut s, _c)) => Some((
                                req.nucleus_id,
                                req.req_id,
                                s.send_request(req.request)
                                    .await
                                    .map_err(|e| RuntimeError::HttpError(e.to_string())),
                            )),
                            Err(e) => Some((
                                req.nucleus_id,
                                req.req_id,
                                CallResult::Err(RuntimeError::HttpError(e.to_string())),
                            )),
                        }
                    }
                    None => None,
                }
            })
            .then(|write_r| async move {
                match write_r {
                    Some((nid, rid, r)) => match r {
                        Ok(response) => match Self::accumulate_frames(response).await {
                            Ok(response) => Some(HttpResponseWithCallback {
                                nucleus_id: nid,
                                req_id: rid,
                                response: CallResult::Ok(response),
                            }),
                            Err(e) => Some(HttpResponseWithCallback {
                                nucleus_id: nid,
                                req_id: rid,
                                response: CallResult::Err(RuntimeError::HttpError(e.to_string())),
                            }),
                        },
                        Err(e) => Some(HttpResponseWithCallback {
                            nucleus_id: nid,
                            req_id: rid,
                            response: CallResult::Err(e),
                        }),
                    },
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

    async fn accumulate_frames(response: Response<Incoming>) -> hyper::Result<HttpResponse> {
        let (p, b) = response.into_parts();
        let body = b.collect().await?.to_bytes().to_vec();
        let response = HttpResponse {
            head: ResponseHead {
                status: p.status.as_u16(),
                headers: p
                    .headers
                    .into_iter()
                    .filter_map(|(k, v)| v.to_str().ok().map(|vv| (k, vv.to_string())))
                    .filter_map(|(k, v)| k.map(|kk| (kk.to_string(), v.to_string())))
                    .collect(),
            },
            body,
        };
        Ok(response)
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

pin_project_lite::pin_project! {
    #[derive(Debug)]
    pub struct TokioStreamAdapter<T> {
        #[pin]
        inner: T,
    }
}

impl<T> hyper::rt::Read for TokioStreamAdapter<T>
where
    T: tokio::io::AsyncRead,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut buf: hyper::rt::ReadBufCursor<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        let n = unsafe {
            let mut tbuf = tokio::io::ReadBuf::uninit(buf.as_mut());
            match tokio::io::AsyncRead::poll_read(self.project().inner, cx, &mut tbuf) {
                Poll::Ready(Ok(())) => tbuf.filled().len(),
                other => return other,
            }
        };

        unsafe {
            buf.advance(n);
        }
        Poll::Ready(Ok(()))
    }
}

impl<T> hyper::rt::Write for TokioStreamAdapter<T>
where
    T: tokio::io::AsyncWrite,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        tokio::io::AsyncWrite::poll_write(self.project().inner, cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        tokio::io::AsyncWrite::poll_flush(self.project().inner, cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        tokio::io::AsyncWrite::poll_shutdown(self.project().inner, cx)
    }

    fn is_write_vectored(&self) -> bool {
        tokio::io::AsyncWrite::is_write_vectored(&self.inner)
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> Poll<Result<usize, std::io::Error>> {
        tokio::io::AsyncWrite::poll_write_vectored(self.project().inner, cx, bufs)
    }
}

impl<T> tokio::io::AsyncRead for TokioStreamAdapter<T>
where
    T: hyper::rt::Read,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        tbuf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        let filled = tbuf.filled().len();
        let sub_filled = unsafe {
            let mut buf = hyper::rt::ReadBuf::uninit(tbuf.unfilled_mut());

            match hyper::rt::Read::poll_read(self.project().inner, cx, buf.unfilled()) {
                Poll::Ready(Ok(())) => buf.filled().len(),
                other => return other,
            }
        };

        let n_filled = filled + sub_filled;
        let n_init = sub_filled;
        unsafe {
            tbuf.assume_init(n_init);
            tbuf.set_filled(n_filled);
        }

        Poll::Ready(Ok(()))
    }
}

impl<T> tokio::io::AsyncWrite for TokioStreamAdapter<T>
where
    T: hyper::rt::Write,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        hyper::rt::Write::poll_write(self.project().inner, cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        hyper::rt::Write::poll_flush(self.project().inner, cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        hyper::rt::Write::poll_shutdown(self.project().inner, cx)
    }

    fn is_write_vectored(&self) -> bool {
        hyper::rt::Write::is_write_vectored(&self.inner)
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> Poll<Result<usize, std::io::Error>> {
        hyper::rt::Write::poll_write_vectored(self.project().inner, cx, bufs)
    }
}

// TODO extend this to support deterministic handshake
async fn local_handshake<B>(
    stream: TcpStream,
) -> std::io::Result<(
    hyper::client::conn::http1::SendRequest<B>,
    hyper::client::conn::http1::Connection<TokioStreamAdapter<TcpStream>, B>,
)>
where
    B: hyper::body::Body + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn core::error::Error + Send + Sync>>,
{
    let stream = TokioStreamAdapter { inner: stream };
    hyper::client::conn::http1::handshake(stream)
        .await
        .map_err(|_| std::io::Error::last_os_error())
}
