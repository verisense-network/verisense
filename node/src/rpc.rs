//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use jsonrpsee::RpcModule;
use sc_consensus_babe::BabeWorkerHandle;
use sc_consensus_babe_rpc::{Babe, BabeApiServer};
use sc_network_types::PeerId;
use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_consensus::SelectChain;
use sp_consensus_babe::BabeApi;
use sp_keystore::KeystorePtr;
use std::{path::PathBuf, sync::Arc};
use vrs_gluon_relayer::Relayer;
use vrs_nucleus_cage::NucleusRpcChannel;
use vrs_primitives::AssetId;
use vrs_runtime::{opaque::Block, AccountId, Balance, Nonce};

#[derive(Clone)]
pub struct NucleusDeps<N> {
    pub rpc_channel: NucleusRpcChannel,
    pub node_id: PeerId,
    pub home_dir: PathBuf,
    pub network: Arc<N>,
    pub authority_discover: sc_authority_discovery::Service,
    pub maybe_validator: Option<AccountId>,
}

pub struct BabeDeps {
    /// A handle to the BABE worker for issuing requests.
    pub babe_worker_handle: BabeWorkerHandle<Block>,
    /// The keystore that manages the keys of the node.
    pub keystore: KeystorePtr,
}

/// Full client dependencies.
pub struct FullDeps<C, P, SC, B, N> {
    /// The client instance to use.
    pub client: Arc<C>,
    pub select_chain: SC,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// blocks backend
    pub backend: Arc<B>,
    /// Dependencies for the BABE RPC extension.
    pub babe: BabeDeps,
    /// Nucleus cage connector
    pub nucleus: NucleusDeps<N>,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P, SC, B, N>(
    deny_unsafe: DenyUnsafe,
    deps: FullDeps<C, P, SC, B, N>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    P: TransactionPool<Block = Block, Hash = <Block as sp_runtime::traits::Block>::Hash> + 'static,
    B: sc_client_api::Backend<Block> + Send + Sync + 'static,
    N: sc_network::service::traits::NetworkService + Send + Sync + 'static,
    B::State: sc_client_api::backend::StateBackend<sp_runtime::traits::HashingFor<Block>>,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
    C: Send + Sync + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce> + 'static,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance> + 'static,
    C::Api: vrs_nucleus_runtime_api::NucleusRuntimeApi<Block> + 'static,
    C::Api: vrs_a2a_runtime_api::A2aRuntimeApi<Block> + 'static,
    C::Api: vrs_mcp_runtime_api::McpRuntimeApi<Block> + 'static,
    C::Api: vrs_swap_runtime_api::SwapApi<Block, AssetId, Balance, Balance> + 'static,
    C::Api: BlockBuilder<Block> + 'static,
    C::Api: BabeApi<Block> + 'static,
    SC: SelectChain<Block> + 'static,
{
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
    use substrate_frame_rpc_system::{System, SystemApiServer};
    use vrs_a2a_rpc::{A2a, A2aApiServer};
    use vrs_mcp_rpc::{Mcp, McpApiServer};
    use vrs_nucleus_rpc::{Nucleus, NucleusApiServer};

    // use beefy_gadget_rpc::{Beefy, BeefyApiServer};
    // use pallet_mmr_rpc::{Mmr, MmrApiServer};
    // use sc_consensus_babe_rpc::{Babe, BabeApiServer};
    // use sc_finality_grandpa_rpc::{Grandpa, GrandpaApiServer};
    // use sc_rpc::dev::{Dev, DevApiServer};
    // use sc_rpc_spec_v2::chain_spec::{ChainSpec, ChainSpecApiServer};
    // use sc_sync_state_rpc::{SyncState, SyncStateApiServer};

    let mut module = RpcModule::new(());
    let FullDeps {
        client,
        select_chain,
        pool,
        backend: _backend,
        babe,
        nucleus,
    } = deps;

    let BabeDeps {
        keystore,
        babe_worker_handle,
    } = babe;
    module.merge(
        Babe::new(
            client.clone(),
            babe_worker_handle.clone(),
            keystore,
            select_chain,
            deny_unsafe,
        )
        .into_rpc(),
    )?;
    module.merge(System::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
    module.merge(TransactionPayment::new(client.clone()).into_rpc())?;
    module.merge(A2a::new(client.clone()).into_rpc())?;
    module.merge(Mcp::new(client.clone()).into_rpc())?;

    let NucleusDeps {
        rpc_channel,
        node_id,
        home_dir,
        network,
        authority_discover,
        maybe_validator,
    } = nucleus;
    module.merge(
        Nucleus::new(
            rpc_channel,
            Relayer::new(client.clone(), network, authority_discover),
            client,
            pool,
            node_id,
            maybe_validator,
            home_dir,
        )
        .into_rpc(),
    )?;

    // Extend this RPC with a custom API by using the following syntax.
    // `YourRpcStruct` should have a reference to a client, which is needed
    // to call into the runtime.
    // `module.merge(YourRpcTrait::into_rpc(YourRpcStruct::new(ReferenceToClient, ...)))?;`

    // You probably want to enable the `rpc v2 chainSpec` API as well
    //
    // let chain_name = chain_spec.name().to_string();
    // let genesis_hash = client.block_hash(0).ok().flatten().expect("Genesis block exists; qed");
    // let properties = chain_spec.properties();
    // module.merge(ChainSpec::new(chain_name, genesis_hash, properties).into_rpc())?;

    Ok(module)
}
