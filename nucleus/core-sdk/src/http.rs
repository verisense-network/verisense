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
pub struct Parts {
    pub method: HttpMethod,
    pub uri: String,
    pub headers: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct Request {
    head: Parts,
    body: Vec<u8>,
}

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct Response {
    head: Parts,
    body: Vec<u8>,
}

#[cfg(feature = "host")]
impl Into<http_type::Request<Vec<u8>>> for Request {
    fn into(self) -> http_type::Request<Vec<u8>> {
        let mut builder = http_type::Request::builder()
            .method(self.head.method)
            .uri(self.head.uri);

        for (key, value) in self.head.headers {
            builder = builder.header(key, value);
        }

        builder.body(self.body).unwrap()
    }
}

#[cfg(feature = "host")]
impl Into<http_type::Method> for HttpMethod {
    fn into(self) -> http_type::Method {
        match self {
            HttpMethod::Options => http_type::Method::OPTIONS,
            HttpMethod::Get => http_type::Method::GET,
            HttpMethod::Post => http_type::Method::POST,
            HttpMethod::Put => http_type::Method::PUT,
            HttpMethod::Delete => http_type::Method::DELETE,
            HttpMethod::Head => http_type::Method::HEAD,
            HttpMethod::Trace => http_type::Method::TRACE,
            HttpMethod::Connect => http_type::Method::CONNECT,
            HttpMethod::Patch => http_type::Method::PATCH,
        }
    }
}

extern "C" {
    fn http_request(req_ptr: *const u8, req_len: i32) -> i32;
}

/// make a http request and return the request_id;
/// a `#[callback]` function will be called with the request_id when the response is ready
///
/// ```
/// #[callback]
/// pub fn on_response(u64: request_id, response: Response) {
///     // handle response
/// }
/// ```
pub fn request(request: Request) -> anyhow::Result<u64> {
    let bytes = request.encode();
    let id = unsafe { http_request(bytes.as_ptr(), bytes.len() as i32) };
    Ok(id as u64)
}
