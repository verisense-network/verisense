#![cfg_attr(not(feature = "std"), no_std)]

use alloc::string::String;
use alloc::string::ToString;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use sp_core::bounded::alloc;
use sp_core::offchain::Duration;
use sp_runtime::offchain::http;
use sp_runtime::offchain::http::Request;
use sp_std::prelude::*;
pub fn verify_domain(domain_name: &str, txt_content: &str) -> Result<bool, http::Error> {
    use alloc::format;
    let rpc_url = format!("https://dns.google/resolve?name={}&type=txt", domain_name);
    info!("query: {}", rpc_url);
    let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(20_000));
    let request: Request<Vec<Vec<u8>>> = http::Request::default()
        .method(http::Method::Get)
        .url(rpc_url.as_str())
        .deadline(deadline)
        .add_header("Content-Type", "application/json");
    let pending = request.send().map_err(|_| http::Error::IoError)?;
    let response = pending
        .try_wait(deadline)
        .map_err(|e| {
            info!("request time out: {:?}", e);
            http::Error::DeadlineReached
        })?
        .map_err(|e| {
            info!("request error: {:?}", e);
            e
        })?;
    if response.code != 200 {
        warn!(
            "Unexpected status code when get validator: {}",
            response.code
        );
        return Ok(false);
    }
    let body = response.body().collect::<Vec<u8>>();
    let json_response: Result<DoHResponse, serde_json::Error> = serde_json::from_slice(&body);
    match json_response {
        Ok(r) => {
            info!("query DoH result: {:?}", r);
            if r.status == 0 && r.answer.is_some() {
                let answers = r.answer.unwrap();
                for a in answers {
                    if txt_content == a.data.as_str() {
                        return Ok(true);
                    }
                }
            }
        }
        Err(e) => {
            log::warn!("Failed to decode http body: {}", e.to_string());
        }
    }

    Ok(false)
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct DoHResponse {
    #[serde(rename = "Status")]
    pub status: i32,
    #[serde(rename = "Answer")]
    pub answer: Option<Vec<Answer>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Answer {
    pub name: String,
    #[serde(rename = "type")]
    pub typex: i32,
    #[serde(rename = "TTL")]
    pub ttl: u32,
    pub data: String,
}
