mod types;

use async_trait::async_trait;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use std::sync::Arc;
use vrs_a2a_runtime_api::A2aRuntimeApi;
use vrs_primitives::NucleusId;

#[rpc(server)]
pub trait A2aApi<Hash> {
    #[method(name = "a2a_list")]
    async fn list(&self) -> RpcResult<Vec<(NucleusId, types::AgentCard)>>;

    #[method(name = "a2a_find")]
    async fn find(&self, id: NucleusId) -> RpcResult<Option<types::AgentCard>>;
}

pub struct A2a<B, C> {
    client: Arc<C>,
    _phantom: std::marker::PhantomData<B>,
}

impl<B, C> A2a<B, C> {
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<B, C> A2aApiServer<B::Hash> for A2a<B, C>
where
    B: sp_runtime::traits::Block + Send + Sync + 'static,
    C: HeaderBackend<B> + ProvideRuntimeApi<B> + Send + Sync + 'static,
    C::Api: A2aRuntimeApi<B> + 'static,
{
    async fn list(&self) -> RpcResult<Vec<(NucleusId, types::AgentCard)>> {
        let hash = self.client.info().best_hash;
        let api = self.client.runtime_api();
        let agents = api
            .get_all_agents(hash)
            .expect("Failed to invoke runtime api");
        let agents = agents
            .into_iter()
            .map(|agent_info| (agent_info.agent_id.clone(), agent_info.agent_card.into()))
            .collect();
        Ok(agents)
    }

    async fn find(&self, id: NucleusId) -> RpcResult<Option<types::AgentCard>> {
        let hash = self.client.info().best_hash;
        let api = self.client.runtime_api();
        let agent = api
            .find_agent(hash, id)
            .expect("Failed to invoke runtime api")
            .map(|agent_info| agent_info.agent_card.into());
        Ok(agent)
    }
}
