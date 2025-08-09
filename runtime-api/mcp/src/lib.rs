#![cfg_attr(not(feature = "std"), no_std)]
use pallet_mcp::McpServerInfo;
use sp_std::vec::Vec;
use vrs_primitives::AccountId;

sp_api::decl_runtime_apis! {
    #[api_version(1)]
    pub trait McpRuntimeApi {
        fn find_mcp_server(mcp_id: &AccountId) -> Option<McpServerInfo<AccountId>>;

        fn get_all_mcp_servers() -> Vec<(AccountId, McpServerInfo<AccountId>)>;
    }
}
