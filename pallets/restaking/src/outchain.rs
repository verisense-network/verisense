use super::*;
use crate::solidity::{decode_query_validators_resp, query_validators_params};
use frame_system::pallet_prelude::BlockNumberFor;
use serde::Serialize;
use serde_json::Value;
use sp_core::offchain::Duration;
use sp_core::U256;
use sp_runtime::offchain::http;
use sp_runtime::offchain::http::Response;
use sp_runtime::traits::TrailingZeroInput;

const SYMBIOTIC_EVM_URL: &str = "https://ethereum-holesky-rpc.publicnode.com";
const SYMBIOTIC_MIDDLEWARE: &str = "0x6aee7796C5574b5806245E8EFdDB7b7d6F8D0181";
const KARAK_EVM_URL: &str = "https://ethereum-holesky-rpc.publicnode.com";
const KARAK_MIDDLEWARE: &str = "0x6aee7796C5574b5806245E8EFdDB7b7d6F8D0181";

impl<T: Config> Pallet<T> {

    fn request_validators_list(rpc_url: &str, middleware: &str, source: &str) -> Result<Vec<(T::AccountId,u128, String, String)>, http::Error> {
        let data = query_validators_params();
        let mut body = br#"
        {
          "id": 1,
          "jsonrpc": "2.0",
          "method": "eth_call",
          "params": [
            {
              "accessList": [],
              "data": ""#
            .to_vec();
        body.extend(data.as_bytes().to_vec());
        body.extend(
            br#"",
              "to": ""#,
        );
        body.extend(middleware.as_bytes().to_vec());
        body.extend(
            br#"",
              "type": "0x02"
            },
            "latest"
          ]
        }"#,
        );
        log!(info, "request body: {}", String::from_utf8(body.clone()).unwrap());

        let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));
        let request = http::Request::default()
            .method(http::Method::Post)
            .url(rpc_url)
            .body(vec![body])
            .add_header("Content-Type", "application/json");
        let pending = request
            .deadline(deadline)
            .send()
            .map_err(|_| http::Error::IoError)?;

        let response = pending
            .try_wait(deadline)
            .map_err(|_| http::Error::DeadlineReached)??;
        if response.code != 200 {
            log!(
                warn,
                "Unexpected status code when get validator: {}",
                response.code
            );
            return Ok(vec![]);
        }
        let body = response.body().collect::<Vec<u8>>();
        log!(info, "body: {:?}", body);
        let json_response: Result<JsonResponse, serde_json::Error> = serde_json::from_slice(&body);
        match json_response {
            Ok(r) => {
                log!(info, "{:?}", &r);
                let mut v = vec![];
                log!(info, "query validators result {}", r.result.clone());
                let vd = decode_query_validators_resp(r.result);
                for d in vd {
                    v.push((
                        T::AccountId::decode(&mut TrailingZeroInput::new(d.key.as_slice()))
                            .unwrap(),
                        d.stake,
                        d.evm_address,
                        source.to_string(),
                    ));
                }
                Ok(v)
            }
            Err(_) => {
                log::warn!("Failed to decode http body");
                return Ok(vec![]);
            }
        }

    }
    pub fn get_validators_list() -> Result<Vec<(T::AccountId,u128, String, String)>, http::Error> {
        let mut vc = vec![];

        for (k, v) in RestakingPlatform::<T>::iter() {
            let mut r = Self::request_validators_list(v.0.as_str(), v.1.as_str(), k.as_str())?;
            vc.append(&mut r);
        }
        Ok(vc)
    }

    pub fn submit_unsigned_transaction(
        block_number: BlockNumberFor<T>,
        public: <T as SigningTypes>::Public,
        key_data: Vec<u8>,
        _validator_id: T::AccountId,
    ) -> Result<(), &'static str> {
        let ret = Self::get_validators_list().unwrap_or_default();
        if ret.len() == 0 {
            return Ok(());
        }
        let result = Signer::<T, T::AppCrypto>::all_accounts()
            .with_filter(vec![public])
            .send_unsigned_transaction(
                |account| ObservationsPayload {
                    public: account.public.clone(),
                    key_data: key_data.clone(),
                    block_number,
                    observations: ret.clone(),
                },
                |payload, signature| Call::update_validators { payload, signature },
            );
        if result.len() != 1 {
            return Err("No account found");
        }
        if result[0].1.is_err() {
            log!(warn, "Failed to update_validators: {:?}", result[0].1);

            return Err("Failed to update_validators");
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct JsonRequest {
    pub id: u32,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<Value>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TxParams {
    #[serde(rename = "accessList")]
    pub access_list: Vec<u32>,
    pub data: String,
    pub to: String,
    #[serde(rename = "type")]
    pub ntype: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct JsonResponse {
    pub jsonrpc: String,
    pub id: u32,
    pub result: String,
}
