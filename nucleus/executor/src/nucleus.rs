use crate::{
    runtime::{ContextAware, FuncRegister},
    vm::Vm,
    NucleusTunnel, RpcReplyChannel, WasmCallError, WasmInfo,
};
use codec::{Decode, Encode};
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;
use vrs_core_sdk::{
    http::{HttpRequest, HttpResponse},
    CallResult,
};
use vrs_primitives::NucleusId;

pub struct Nucleus<R> {
    receiver: Receiver<(u64, Gluon)>,
    vm: Option<Vm<R>>,
    runtime: R,
    token_timeout_tx: tokio::sync::mpsc::Sender<NucleusId>
}

impl<R> Nucleus<R>
where
    R: ContextAware + FuncRegister<Runtime = R> + Clone + Send + Sync + 'static,
{
    /// spawn a native thread to run nucleus
    pub fn start(runtime: R,   token_timeout_tx: tokio::sync::mpsc::Sender<NucleusId>) -> NucleusTunnel {
        let (tx, rx) = mpsc::channel();
        let mut nucleus = Nucleus {
            receiver: rx,
            vm: None,
            runtime,
            token_timeout_tx,
        };
        std::thread::spawn(move || nucleus.run());
        tx
    }

    fn run(&mut self) {
        loop {
            if let Ok((id, msg)) = self.receiver.recv_timeout(Duration::from_secs(20)) {
                // TODO save msg with id to state
                if let Err(e) = self.accept(msg) {
                    log::error!(
                    "Nucleus {} interrupted: {:?}",
                    self.runtime.get_nucleus_id(),
                    e);
                    break;
                }
            } else {
                let _ = self.token_timeout_tx.send(self.runtime.get_nucleus_id());
            }
        }
    }

    // this wasm's validity is guaranteed by the cage
    fn upgrade_code(&mut self, wasm: WasmInfo) {
        match Vm::new_instance(&wasm, &self.runtime) {
            Ok(mut vm) => {
                vm.call_init();
                self.vm.replace(vm);
            }
            Err(e) => {
                log::error!("Init VM for nucleus {} failed, {:?}.", &wasm.id, e);
            }
        }
    }

    fn call<F, T>(&mut self, f: F) -> Result<T, WasmCallError>
    where
        F: FnOnce(&mut Vm<R>) -> Result<T, WasmCallError>,
    {
        let mut vm = self.vm.as_mut().ok_or(WasmCallError::WasmNotInitialized)?;
        f(&mut vm)
    }

    fn accept(&mut self, msg: Gluon) -> anyhow::Result<()> {
        match msg {
            Gluon::CodeUpgrade { wasm } => {
                self.upgrade_code(wasm);
            }
            Gluon::HttpCallback {
                request_id,
                payload,
            } => {
                if let Err(e) = self.call(|vm| vm.call_http_callback((request_id, payload))) {
                    log::error!("fail to call http callback due to: {:?}", e);
                }
            }
            Gluon::GetRequest {
                endpoint,
                payload,
                reply_to,
            } => {
                let result = self
                    .call(|vm| vm.call_get(&endpoint, payload))
                    .map_err(|e| e.into());
                let _ = reply_to.map(|reply_to| reply_to.send(result));
            }
            Gluon::PostRequest {
                endpoint,
                payload,
                reply_to,
            } => {
                let result = self
                    .call(|vm| vm.call_post(&endpoint, payload))
                    .map_err(|e| e.into());
                let _ = reply_to.map(|reply_to| reply_to.send(result));
            }
            Gluon::TimerTrigger {
                endpoint,
                task_id,
                payload,
            } => {
                if let Err(e) = self.call(|vm| vm.call_timer_trigger(&endpoint, payload)) {
                    log::error!("fail to call timer callback: {} due to: {:?}", &endpoint, e);
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Gluon {
    CodeUpgrade {
        wasm: WasmInfo,
    },
    PostRequest {
        endpoint: String,
        payload: Vec<u8>,
        reply_to: Option<RpcReplyChannel>,
    },
    GetRequest {
        endpoint: String,
        payload: Vec<u8>,
        reply_to: Option<RpcReplyChannel>,
    },
    TimerTrigger {
        endpoint: String,
        task_id: u64,
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
            Gluon::CodeUpgrade { wasm } => Event::CodeUpgrade {
                version: wasm.version,
                digest: wasm.digest(),
            },
            Gluon::PostRequest {
                endpoint, payload, ..
            } => Event::PostRequest {
                endpoint: endpoint.clone(),
                payload: payload.clone(),
            },
            Gluon::GetRequest { .. } => Event::Dummy,
            Gluon::TimerTrigger {
                task_id,
                endpoint,
                payload,
                ..
            } => Event::TimerTrigger {
                task_id: *task_id,
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

/// Serialized `Gluon`, i.e. received from peers or call `gluon.into()` then save it
#[derive(Debug, Clone, Encode, Decode)]
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
    #[codec(skip)]
    Dummy,
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
