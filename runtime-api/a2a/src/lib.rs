#![cfg_attr(not(feature = "std"), no_std)]
use a2a_rs::AgentInfo;
use sp_std::vec::Vec;
use vrs_primitives::AccountId;

sp_api::decl_runtime_apis! {
    #[api_version(1)]
    pub trait A2aRuntimeApi {
        fn find_agent(agent_id: AccountId) -> Option<AgentInfo<AccountId>>;

        fn get_all_agents() -> Vec<AgentInfo<AccountId>>;
    }
}
