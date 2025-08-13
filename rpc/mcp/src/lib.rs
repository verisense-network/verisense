use async_trait::async_trait;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use serde::{Deserialize, Serialize};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use std::sync::Arc;
use vrs_mcp_runtime_api::McpRuntimeApi;
use vrs_primitives::AccountId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServer<T> {
    pub id: T,
    pub name: String,
    pub description: String,
    pub url: String,
    pub provider: T,
}

fn bytes_to_string(bytes: &[u8]) -> String {
    match String::from_utf8(bytes.to_vec()) {
        Ok(s) => {
            if s.chars().all(|c| !c.is_control() || c.is_whitespace()) {
                s
            } else {
                format!("0x{}", hex::encode(bytes))
            }
        }
        Err(_) => {
            format!("0x{}", hex::encode(bytes))
        }
    }
}

impl<T> From<(T, pallet_mcp::McpServerInfo<T>)> for McpServer<T> {
    fn from((id, server): (T, pallet_mcp::McpServerInfo<T>)) -> Self {
        Self {
            id,
            name: String::from_utf8_lossy(&server.name).into_owned(),
            description: bytes_to_string(&server.description),
            url: String::from_utf8_lossy(&server.url).into_owned(),
            provider: server.provider,
        }
    }
}

#[rpc(server)]
pub trait McpApi<Hash> {
    #[method(name = "mcp_list")]
    async fn list(&self) -> RpcResult<Vec<McpServer<AccountId>>>;

    #[method(name = "mcp_find")]
    async fn find(&self, id: AccountId) -> RpcResult<Option<McpServer<AccountId>>>;
}

pub struct Mcp<B, C> {
    client: Arc<C>,
    _phantom: std::marker::PhantomData<B>,
}

impl<B, C> Mcp<B, C> {
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<B, C> McpApiServer<B::Hash> for Mcp<B, C>
where
    B: sp_runtime::traits::Block + Send + Sync + 'static,
    C: HeaderBackend<B> + ProvideRuntimeApi<B> + Send + Sync + 'static,
    C::Api: McpRuntimeApi<B> + 'static,
{
    async fn list(&self) -> RpcResult<Vec<McpServer<AccountId>>> {
        let hash = self.client.info().best_hash;
        let api = self.client.runtime_api();
        let servers = api
            .get_all_mcp_servers(hash)
            .expect("Failed to invoke runtime api");
        let servers = servers
            .into_iter()
            .map(|info| McpServer::from(info))
            .collect();
        Ok(servers)
    }

    async fn find(&self, id: AccountId) -> RpcResult<Option<McpServer<AccountId>>> {
        let hash = self.client.info().best_hash;
        let api = self.client.runtime_api();
        let mcp = api
            .find_mcp_server(hash, &id)
            .expect("Failed to invoke runtime api")
            .map(|info| McpServer::from((id, info)));
        Ok(mcp)
    }
}
