use frame_support::{Deserialize, Serialize};
use log::{info, warn};
use crate::{Config, Pallet};
use sp_core::bounded::alloc;
use sp_core::offchain::Duration;
use sp_runtime::offchain::http;
use sp_runtime::offchain::http::Request;
use sp_std::vec::Vec;
use alloc::string::ToString;
use alloc::string::String;
use url::Url;

impl<T: Config> Pallet<T> {

    pub fn verify_domain(
        domain_name: &str,
    ) -> Result<bool, http::Error> {
       /* use alloc::format;
        let rpc_url = format!("https://dns.google/resolve?name=www.baidu.com&type=txt");
        info!("query: {}", rpc_url);
        let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(20_000));
        let request: Request<Vec<Vec<u8>>> = http::Request::default()
            .method(http::Method::Get)
            .url(rpc_url.as_str())
            .deadline(deadline)
            .add_header("Content-Type", "application/json");
        let pending = request
            .send()
            .map_err(|_| http::Error::IoError)?;
        let response = pending
            .try_wait(deadline)
            .map_err(|e| {
                info!("request timed out: {:?}", e);
                http::Error::DeadlineReached
            })?.map_err(|e|{
            info!("request timed out1: {:?}", e);
            e
        })?;
        info!("received1: {:?}", response);
        if response.code != 200 {
            warn!(
                "Unexpected status code when get validator: {}",
                response.code
            );
            return Ok(false);
        }
        let body = response.body().collect::<Vec<u8>>();
        info!("body: {:?}", body);
        let json_response: Result<DoHResponse, serde_json::Error> = serde_json::from_slice(&body);
        match json_response {
            Ok(r) => {
                info!( "query DoH {:?}", r);
                if r.status == 0 && r.answer.is_some() {
                    let answers = r.answer.unwrap();
                    for a in answers {
                        if domain_name.starts_with(a.data.as_str()) {
                            return Ok(true);
                        }
                    }
                }
            }
            Err(e) => {
                log::warn!("Failed to decode http body: {}", e.to_string());
            }
        }*/

        let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(5_000));
        let pending = http::Request::get("https://www.dns.google/resolve?nn=www.baidu.com&type=txt").send().unwrap();
        let repsonse= pending.try_wait(deadline).unwrap().unwrap();
        log::info!("repsonse: {:?}", repsonse.code);
        
        Ok(false)
    }
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