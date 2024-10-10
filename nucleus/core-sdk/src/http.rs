//! Module for making http requests within nucleus.

use crate::error::RuntimeError;
use codec::{Decode, Encode};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Encode, Decode)]
pub enum HttpMethod {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
}

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct RequestHead {
    pub method: HttpMethod,
    pub uri: String,
    pub headers: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct HttpRequest {
    pub head: RequestHead,
    pub body: Vec<u8>,
}

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct ResponseHead {
    pub status: u16,
    pub headers: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct HttpResponse {
    pub head: ResponseHead,
    pub body: Vec<u8>,
}

#[link(wasm_import_module = "env")]
extern "C" {
    fn http_request(req_ptr: *const u8, req_len: u32, return_ptr: *mut u8) -> i32;
}

/// Make a http request and return the request_id immediately.
///
/// A `#[callback]` function will be called with the request_id when the response is ready.
///
/// ```
/// use vrs_core_sdk::{CallResult, http::{*, self}, callback, post};
///
/// #[post]
/// pub fn request_google() {
///     let id = http::request(HttpRequest {
///         head: RequestHead {
///             method: HttpMethod::Get,
///             uri: "https://www.google.com".to_string(),
///             headers: Default::default(),
///         },
///         body: vec![],
///     })
///     .unwrap();
///     vrs_core_sdk::println!("http request {} enqueued", id);
/// }
///
/// #[callback]
/// pub fn on_response(id: u64, response: CallResult<HttpResponse>) {
///     match response {
///         Ok(response) => {
///             let body = String::from_utf8_lossy(&response.body);
///             vrs_core_sdk::println!("id = {}, response: {}", id, body);
///         }
///         Err(e) => {
///             vrs_core_sdk::eprintln!("id = {}, error: {:?}", id, e);
///         }
///     }
/// }
/// ```
pub fn request(request: HttpRequest) -> Result<u64, RuntimeError> {
    let bytes = request.encode();
    let mut return_bytes = crate::allocate_buffer();
    let status = unsafe {
        http_request(
            bytes.as_ptr(),
            bytes.len() as u32,
            return_bytes.as_mut_ptr(),
        )
    };
    assert!(status == crate::NO_MORE_DATA);
    Result::<u64, RuntimeError>::decode(&mut &return_bytes[..])
        .map_err(|_| RuntimeError::DecodeReturnValueError)?
}
