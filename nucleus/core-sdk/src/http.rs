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

/// make a http request and return the request_id;
/// a `#[callback]` function will be called with the request_id when the response is ready
///
/// ```
/// #[callback]
/// pub fn on_response(u64: request_id, response: CallResult<HttpResponse>) {
///     // handle response
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
    // we don't expect an asynchronous response more than 64k
    assert!(status == crate::NO_MORE_DATA);
    Result::<u64, RuntimeError>::decode(&mut &return_bytes[..])
        .map_err(|_| RuntimeError::DecodeReturnValueError)?
}
