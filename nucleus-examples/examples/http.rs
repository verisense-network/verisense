use vrs_core_macros::{callback, post};
use vrs_core_sdk::{
    http::{self, *},
    CallResult,
};

#[post]
pub fn request_google() {
    let id = http::request(HttpRequest {
        head: Parts {
            method: HttpMethod::Get,
            uri: "https://www.google.com".to_string(),
            headers: Default::default(),
        },
        body: vec![],
    })
    .unwrap();
    vrs_core_sdk::println!("http request {} enqueued", id);
}

#[callback]
pub fn on_response(id: u64, response: CallResult<HttpResponse>) {
    match response {
        Ok(response) => {
            let body = String::from_utf8_lossy(&response.body);
            vrs_core_sdk::println!("id = {}, response: {}", id, body);
        }
        Err(e) => {
            vrs_core_sdk::eprintln!("id = {}, error: {:?}", id, e);
        }
    }
}
