use crate::{
    mem,
    runtime::{ComponentProvider, ContextAware},
    Runtime,
};
use bytes::Bytes;
use codec::{Decode, Encode};
use http_body_util::{BodyExt, Full};
use hyper::{body::Incoming, Method, Request, Response, Uri};
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use std::sync::atomic::{AtomicU64, Ordering};
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
    // rx: UnboundedReceiver<HttpRequestWithCallback>,
    // response_sender: UnboundedSender<HttpResponseWithCallback>,
    response_receiver: UnboundedReceiver<HttpResponseWithCallback>,
}

impl HttpCallExecutor {
    fn new(mut rx: UnboundedReceiver<HttpRequestWithCallback>) -> Self {
        let (rtx, rrx) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            loop {
                let request = rx.recv().await;
                if let Some(req) = request {
                    let nucleus_id = req.nucleus_id.clone();
                    let req_id = req.req_id;
                    let sender = rtx.clone();
                    tokio::spawn(async move {
                        let response = tokio::select! {
                            response = Self::send_request(req) => {
                                log::info!("🌐 Http response: {:?}", response);
                                response
                            },
                            _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
                                log::info!("🌐 Http response: timeout");
                                Err(RuntimeError::HttpError("timeout".to_string()))
                            },
                        };
                        let r = sender.send(HttpResponseWithCallback {
                            nucleus_id,
                            req_id,
                            response,
                        });
                        if r.is_err() {
                            log::error!("🌐 Send response failed");
                        }
                    });
                } else {
                    log::info!("🌐 Http executor receive none");
                }
            }
        });
        HttpCallExecutor {
            // rx,
            response_receiver: rrx,
        }
    }
    pub(crate) async fn recv_response(&mut self) -> Option<HttpResponseWithCallback> {
        self.response_receiver.recv().await
    }
    // pub(crate) async fn poll<'a>(
    //     &'a mut self,
    // ) -> Result<Option<HttpResponseWithCallback>, JoinError> {
    //     println!("start poll");
    //     self.rx
    //         .recv()
    //         .then(|req| {
    //             println!("recv req :{:?}", req);
    //             tokio::spawn(async move {
    //                 println!("spawn");
    //                 match req {
    //                     Some(req) => {
    //                         println!("req: {:?}", req);
    //                         let req_id = req.req_id;
    //                         let nucleus_id = req.nucleus_id.clone();
    //                         let response = tokio::select! {
    //                         response = Self::send_request(req) => {
    //                         println!("response: {:?}", response);
    //                         response
    //                         },
    //                         _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
    //                         println!("timeout");
    //                         Err(RuntimeError::HttpError("timeout".to_string()))
    //                         },
    //                         };
    //                         Some(HttpResponseWithCallback {
    //                             nucleus_id,
    //                             req_id,
    //                             response,
    //                         })
    //                     }
    //                     None => None,
    //                 }
    //             })
    //         })
    //         .await
    // }

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

    async fn send_request(req: HttpRequestWithCallback) -> CallResult<HttpResponse> {
        let https = hyper_tls::HttpsConnector::new();
        let client = Client::builder(TokioExecutor::new()).build::<_, Full<Bytes>>(https);
        let response = client
            .request(req.request)
            .await
            .map_err(|e| RuntimeError::HttpError(e.to_string()))?;
        Self::accumulate_frames(response)
            .await
            .map_err(|e| RuntimeError::HttpError(e.to_string()))
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
        let return_value = CallResult::<()>::Err(RuntimeError::ReadOnly);
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

// TODO extend this to support deterministic handshake
// async fn local_handshake<B>(
//     stream: TcpStream,
// ) -> std::io::Result<(
//     hyper::client::conn::http1::SendRequest<B>,
//     hyper::client::conn::http1::Connection<TokioStreamAdapter<TcpStream>, B>,
// )>
// where
//     B: hyper::body::Body + 'static,
//     B::Data: Send,
//     B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
// {
//     let stream = TokioStreamAdapter { inner: stream };
//     hyper::client::conn::http1::handshake(stream)
//         .await
//         .map_err(|_| std::io::Error::last_os_error())
// }

// #[tokio::test]
// pub async fn register_http_should_work() {
//     env_logger::init();
//     let (register, mut executor) = new_http_manager();
//     tokio::spawn(async move {
//         let req = HttpRequest {
//             head: RequestHead {
//                 method: HttpMethod::Get,
//                 uri: "https://www.baidu.com".to_string(),
//                 headers: Default::default(),
//             },
//             body: vec![],
//         };
//         register
//             .enqueue_request(NucleusId::new([0u8; 32]), req)
//             .unwrap();
//     });
//     let mut executed: usize = 0;
//     loop {
//         if executed > 5 {
//             panic!("http never triggered");
//         }
//         tokio::select! {
//             _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {
//                 executed += 1;
//             }
//             response = executor.poll() => {
//                 assert!(response.is_ok());
//                 let response = response.unwrap();
//                 assert!(response.is_some());
//                 let response = response.unwrap();
//                 assert_eq!(response.req_id, 0);
//                 break;
//             }
//         }
//     }
// }
