use crate::{
    runtime::{ContextAware, FuncRegister, Runtime},
    vm::{Vm, WasmCallError},
    ReplyTo, TimerEntry, TimersReplyTo, WasmInfo,
};
use codec::{Decode, Encode};
use std::sync::mpsc::Receiver;
use vrs_core_sdk::{
    http::{HttpRequest, HttpResponse},
    CallResult,
};

pub struct Nucleus<R> {
    receiver: Receiver<(u64, Gluon)>,
    vm: Option<Vm<R>>,
    runtime: R,
}

#[derive(Debug)]
pub enum Gluon {
    CodeUpgrade {
        version: u32,
        digest: [u8; 32],
        wasm: Option<WasmInfo>,
    },
    PostRequest {
        endpoint: String,
        payload: Vec<u8>,
        reply_to: Option<ReplyTo>,
    },
    GetRequest {
        endpoint: String,
        payload: Vec<u8>,
        reply_to: Option<ReplyTo>,
    },
    TimerRequest {
        endpoint: String,
        payload: Vec<u8>,
    },
    HttpCallback {
        request_id: u64,
        payload: CallResult<HttpResponse>,
    },
}

impl From<&Gluon> for Event {
    fn from(gluon: &Gluon) -> Self {
        match gluon {
            Gluon::CodeUpgrade {
                version, digest, ..
            } => Event::CodeUpgrade {
                version: *version,
                digest: *digest,
            },
            Gluon::PostRequest {
                endpoint, payload, ..
            } => Event::PostRequest {
                endpoint: endpoint.clone(),
                payload: payload.clone(),
            },
            Gluon::GetRequest { .. } => Event::Dummy,
            Gluon::TimerRequest {
                // task_id,
                endpoint,
                payload,
                ..
            } => Event::TimerTrigger {
                task_id: 0,
                endpoint: endpoint.clone(),
                payload: payload.clone(),
            },
            Gluon::HttpCallback {
                request_id,
                payload,
            } => Event::HttpCallback {
                request_id: *request_id,
                payload: payload.clone(),
            },
        }
    }
}

impl Into<Event> for Gluon {
    fn into(self) -> Event {
        match self {
            Self::CodeUpgrade {
                version, digest, ..
            } => Event::CodeUpgrade { version, digest },
            Self::PostRequest {
                endpoint, payload, ..
            } => Event::PostRequest { endpoint, payload },
            Self::GetRequest { .. } => Event::Dummy,
            Self::TimerRequest {
                // task_id,
                endpoint,
                payload,
                ..
            } => Event::TimerTrigger {
                task_id: 0,
                endpoint,
                payload,
            },
            Self::HttpCallback {
                request_id,
                payload,
            } => Event::HttpCallback {
                request_id,
                payload,
            },
        }
    }
}

/// Serialized `Gluon`, i.e. received from peers or call `gluon.into()` then save it
#[derive(Debug, Encode, Decode)]
pub enum Event {
    #[codec(index = 0)]
    CodeUpgrade { version: u32, digest: [u8; 32] },
    #[codec(index = 1)]
    PostRequest { endpoint: String, payload: Vec<u8> },
    #[codec(index = 2)]
    TimerRegister {
        task_id: u64,
        delay: u64,
        endpoint: String,
        payload: Vec<u8>,
    },
    #[codec(index = 3)]
    TimerTrigger {
        task_id: u64,
        endpoint: String,
        payload: Vec<u8>,
    },
    #[codec(index = 4)]
    HttpRequest {
        request_id: u64,
        ssl_key: Vec<u8>,
        request: HttpRequest,
    },
    #[codec(index = 5)]
    HttpCallback {
        request_id: u64,
        payload: CallResult<HttpResponse>,
    },
    #[codec(index = 6)]
    TimerInit {},
    #[codec(skip)]
    Dummy,
}

impl<R> Nucleus<R>
where
    R: ContextAware + FuncRegister<Runtime = R> + Clone,
{
    pub(crate) fn init(receiver: Receiver<(u64, Gluon)>, runtime: R) -> Self {
        Self {
            receiver,
            vm: None,
            runtime,
        }
    }

    pub(crate) fn run(&mut self) {
        while let Ok((id, msg)) = self.receiver.recv() {
            // TODO save msg with id to state
            // only interrupted if events save failed or VM not initialized
            if let Err(e) = self.accept(msg) {
                log::error!("Nucleus interrupted due to: {:?}", e);
                break;
            }
        }
    }

    fn accept(&mut self, msg: Gluon) -> anyhow::Result<()> {
        match msg {
            Gluon::CodeUpgrade {
                version,
                digest,
                wasm,
            } => {
                let wasm = wasm.expect("lookup wasm after deserializing `Event::CodeUpgrade`");
                let vm = Vm::new_instance(&wasm, &self.runtime).inspect_err(|e| {
                    log::error!("Init vm for nucleus {} failed due to {:?}", &wasm.id, e)
                });
                if let Ok(mut vm) = vm {
                    match vm.call_init() {
                        Ok(_) => {
                            self.vm.replace(vm);
                        }
                        Err(e) => {
                            log::error!("Init vm for nucleus {} failed due to {:?}", &wasm.id, e);
                        }
                    }
                }
            }
            Gluon::HttpCallback {
                request_id,
                payload,
            } => {
                let vm = self
                    .vm
                    .as_mut()
                    .ok_or(anyhow::anyhow!("VM not initialized"))?;
                let _ = vm.call_inner("__nucleus_http_callback", (request_id, payload));
            }
            Gluon::GetRequest {
                endpoint,
                payload,
                reply_to,
            } => match self.vm {
                Some(ref mut vm) => {
                    let vm_result = vm.call_get(&endpoint, payload).map_err(|e| {
                        log::error!("fail to call get endpoint: {} due to: {:?}", &endpoint, e);
                        (e.to_error_code(), e.to_string())
                    });
                    if let Some(reply_to) = reply_to {
                        let _ = reply_to.send(vm_result);
                    }
                }
                None => {
                    if let Some(reply_to) = reply_to {
                        let _ = reply_to.send(Err(WasmCallError::WasmNotInitialized.into()));
                    }
                }
            },
            Gluon::PostRequest {
                endpoint,
                payload,
                reply_to,
            } => match self.vm {
                Some(ref mut vm) => {
                    let vm_result = vm.call_post(&endpoint, payload.clone()).map_err(|e| {
                        log::error!("fail to call post endpoint: {} due to: {:?}", &endpoint, e);
                        (e.to_error_code(), e.to_string())
                    });
                    println!("vm_result: {:?}", reply_to);
                    if let Some(reply_to) = reply_to {
                        println!("send vm_result: {:?}", vm_result);
                        let _ = reply_to.send(vm_result);
                    }
                }
                None => {
                    if let Some(reply_to) = reply_to {
                        let _ = reply_to.send(Err(WasmCallError::WasmNotInitialized.into()));
                    }
                }
            },
            Gluon::TimerRequest { endpoint, payload } => {
                let vm = self
                    .vm
                    .as_mut()
                    .ok_or(anyhow::anyhow!("VM not initialized"))?;
                let vm_result = vm.call_timer(&endpoint, payload.clone());
                match vm_result {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("fail to call timer endpoint: {} due to: {:?}", &endpoint, e);
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_suite::new_mock_nucleus;

    use super::*;
    use codec::Decode;
    use std::sync::mpsc;
    use tokio::sync::oneshot;

    #[tokio::test]
    async fn test_nucleus_accept() {
        let wasm_path = "../../nucleus-examples/basic_macros.wasm";
        let (out_of_run_time, sender) = new_mock_nucleus(format!("{}", wasm_path));
        let (tx_get, rx_get) = oneshot::channel();
        let get_msg = Gluon::GetRequest {
            endpoint: "get".to_string(),
            payload: vec![],
            reply_to: Some(tx_get),
        };
        sender.send((0, get_msg)).unwrap();

        let (tx_post, rx_post) = oneshot::channel();
        let post_msg = Gluon::PostRequest {
            endpoint: "cc".to_string(),
            payload: vec![
                40, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 40, 98, 98, 98, 98, 98, 98, 98, 98, 98,
                98,
            ],
            reply_to: Some(tx_post),
        };
        sender.send((1, post_msg)).unwrap();

        let (tx_post, rx_post1) = oneshot::channel();
        let post_msg = Gluon::PostRequest {
            endpoint: "bc".to_string(),
            payload: vec![
                40, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 40, 98, 98, 98, 98, 98, 98, 98, 98, 98,
                98,
            ],
            reply_to: Some(tx_post),
        };
        sender.send((1, post_msg)).unwrap();

        let get_result = rx_get.await.unwrap().unwrap();
        let get_result = <i32 as Decode>::decode(&mut &get_result[..]).unwrap();
        assert_eq!(get_result, 5);
        let post_result = rx_post.await.unwrap().unwrap();
        let post_result =
            <Result<String, String> as Decode>::decode(&mut &post_result[..]).unwrap();
        assert_eq!(
            post_result,
            Result::<String, String>::Ok("abababababababababab".to_owned())
        );
        let post_result = rx_post1.await.unwrap();
        assert_eq!(post_result, Err((-5000, "Endpoint not found".to_owned())))
    }
}
#[cfg(test)]
mod forum_tests {
    use super::*;
    use crate::host_func::http::HttpResponseWithCallback;
    use crate::test_suite::new_mock_nucleus;
    use codec::Decode;
    use codec::Encode;
    use core::time;
    use serde::{Deserialize, Serialize};
    use std::sync::mpsc;
    use tokio::sync::oneshot;

    #[derive(Debug, Decode, Encode, Deserialize, Serialize)]
    pub enum Method {
        Create,
        Update,
        Delete,
    }

    #[derive(Debug, Decode, Encode, Deserialize, Serialize)]
    pub struct VeSubspace {
        pub id: u64,
        pub title: String,
        pub slug: String,
        pub description: String,
        pub banner: String,
        pub status: i16,
        pub weight: i16,
        pub created_time: i64,
    }

    #[derive(Debug, Decode, Encode, Deserialize, Serialize)]
    pub struct VeArticle {
        pub id: u64,
        pub title: String,
        pub content: String,
        pub author_id: u64,
        pub author_nickname: String,
        pub subspace_id: u64,
        pub ext_link: String,
        pub status: i16,
        pub weight: i16,
        pub created_time: i64,
        pub updated_time: i64,
    }

    #[derive(Debug, Decode, Encode, Deserialize, Serialize)]
    pub struct VeComment {
        pub id: u64,
        pub content: String,
        pub author_id: u64,
        pub author_nickname: String,
        pub article_id: u64,
        pub status: i16,
        pub weight: i16,
        pub created_time: i64,
    }

    // const PREFIX_USER_KEY: &[u8; 5] = b"veus:";
    pub const PREFIX_SUBSPACE_KEY: &[u8; 5] = b"vesb:";
    pub const PREFIX_ARTICLE_KEY: &[u8; 5] = b"vear:";
    pub const PREFIX_COMMENT_KEY: &[u8; 5] = b"veco:";

    pub const REQNUM_KEY: &[u8; 7] = b"_reqnum";
    pub const COMMON_KEY: &[u8; 7] = b"_common";

    #[tokio::test]
    async fn test_nucleus_accept() {
        let wasm_path = "../../../veforum/veavs.wasm";
        let (mut out_of_runtime, sender) = new_mock_nucleus(format!("{}", wasm_path));
        let article = VeArticle {
            id: 0,
            title: "title".to_string(),
            content: "Today is a good day".to_string(),
            author_id: 0,
            author_nickname: "AI Assistant".to_string(),
            subspace_id: 0,
            ext_link: "ext_link".to_string(),
            status: 0,
            weight: 0,
            created_time: chrono::Utc::now().timestamp(),
            updated_time: 0,
        };
        let (article_tx, article_rx) = oneshot::channel();
        let article_post = Gluon::PostRequest {
            endpoint: "add_article".to_string(),
            payload: article.encode(),
            reply_to: Some(article_tx),
        };
        sender.send((0, article_post)).unwrap();
        let article_post_result = article_rx.await.unwrap().unwrap();
        let article_post_result =
            <Result<(), String> as Decode>::decode(&mut &article_post_result[..]).unwrap();
        println!("{:?}", article_post_result);

        // fetch lastest article
        let (get_article_tx, get_article_rx) = oneshot::channel();
        let article_get = Gluon::GetRequest {
            endpoint: "fetch_lastest_article".to_string(),
            payload: vec![],
            reply_to: Some(get_article_tx),
        };
        sender.send((0, article_get)).unwrap();
        let article_get_result = get_article_rx.await.unwrap().unwrap();
        let article_get_result =
            <Result<Vec<VeArticle>, String>>::decode(&mut &article_get_result[..]).unwrap();
        println!("{:?}", article_get_result);

        // check article replied
        let (get_article_tx, get_article_rx) = oneshot::channel();
        let data: (u64,) = (1,);
        let article_get = Gluon::GetRequest {
            endpoint: "check_article_processing".to_string(),
            payload: data.encode(),
            reply_to: Some(get_article_tx),
        };
        sender.send((0, article_get)).unwrap();
        let article_get_result = get_article_rx.await.unwrap().unwrap();
        let article_get_result =
            <Result<bool, String>>::decode(&mut &article_get_result[..]).unwrap();
        println!("{:?}", article_get_result);

        //reply article
        let (reply_article_tx, reply_article_rx) = oneshot::channel();
        let article_reply: Gluon = Gluon::PostRequest {
            endpoint: "reply_all_articles".to_string(),
            payload: vec![],
            reply_to: Some(reply_article_tx),
        };
        sender.send((0, article_reply)).unwrap();

        let article_reply_result = reply_article_rx.await.unwrap().unwrap();
        let article_reply_result =
            <Result<(), String>>::decode(&mut &article_reply_result[..]).unwrap();
        println!("{:?}", article_reply_result);

        // check article replied
        let (get_article_tx, get_article_rx) = oneshot::channel();
        let data: (u64,) = (1,);
        let article_get = Gluon::GetRequest {
            endpoint: "check_article_processing".to_string(),
            payload: data.encode(),
            reply_to: Some(get_article_tx),
        };
        sender.send((0, article_get)).unwrap();
        let article_get_result = get_article_rx.await.unwrap().unwrap();
        let article_get_result =
            <Result<bool, String>>::decode(&mut &article_get_result[..]).unwrap();
        println!("{:?}", article_get_result);

        // let http_reply = http_executor.http_executor.poll().await;
        // let HttpResponseWithCallback {
        //     nucleus_id,
        //     req_id,
        //     response,
        // } = http_reply
        //     .expect("already checked")
        //     .expect("HttpCallRegister must exists;qed");
        // nucleus.accept(Gluon::HttpCallback {
        //     request_id: req_id,
        //     payload: response,
        // });
        // let request = http_executor.http_executor.recv_request().await.unwrap();
        // http_executor.http_executor.process_request(request);

        let http_reply = out_of_runtime.http_executor.recv_response().await.unwrap();
        let HttpResponseWithCallback {
            nucleus_id,
            req_id,
            response,
        } = http_reply;
        sender
            .send((
                0,
                Gluon::HttpCallback {
                    request_id: req_id,
                    payload: response,
                },
            ))
            .unwrap();

        // // check comment
        // let (check_comment_tx, check_comment_rx) = oneshot::channel();
        // let data: (u64,) = (2,);
        // let comment_check = Gluon::GetRequest {
        //     endpoint: "get_comment".to_string(),
        //     payload: data.encode(),
        //     reply_to: Some(check_comment_tx),
        // };
        // sender.send((0, comment_check)).unwrap();
        // let (_, msg) = nucleus.receiver.recv().unwrap();
        // nucleus.accept(msg);
        // let comment_check_result = check_comment_rx.await.unwrap().unwrap();
        // let comment_check_result =
        //     <Result<Option<VeComment>, String>>::decode(&mut &comment_check_result[..]).unwrap();
        // println!("{:?}", comment_check_result);

        // let data: (u64,) = (1,);
        // let (tx_post, rx_post) = oneshot::channel();
        // let post_msg = Gluon::PostRequest {
        //     endpoint: "cc".to_string(),
        //     payload: vec![
        //         40, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 40, 98, 98, 98, 98, 98, 98, 98, 98, 98,
        //         98,
        //     ],
        //     reply_to: Some(tx_post),
        // };
        // sender.send((1, post_msg)).unwrap();

        // let (tx_post, rx_post1) = oneshot::channel();
        // let post_msg = Gluon::PostRequest {
        //     endpoint: "bc".to_string(),
        //     payload: vec![
        //         40, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 40, 98, 98, 98, 98, 98, 98, 98, 98, 98,
        //         98,
        //     ],
        //     reply_to: Some(tx_post),
        // };
        // sender.send((1, post_msg)).unwrap();

        // for _ in 0..3 {
        //     let (_, msg) = nucleus.receiver.recv().unwrap();
        //     nucleus.accept(msg);
        // }

        // let get_result = <i32 as Decode>::decode(&mut &get_result[..]).unwrap();
        // assert_eq!(get_result, 5);
        // let post_result = rx_post.await.unwrap().unwrap();
        // let post_result =
        //     <Result<String, String> as Decode>::decode(&mut &post_result[..]).unwrap();
        // assert_eq!(
        //     post_result,
        //     Result::<String, String>::Ok("abababababababababab".to_owned())
        // );
        // let post_result = rx_post1.await.unwrap();
        // assert_eq!(post_result, Err((1024, "Endpoint not found".to_owned())))
    }
}
