//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::{path::PathBuf, sync::Arc};

use jsonrpsee::RpcModule;
use sc_network_types::PeerId;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use vrs_nucleus_cage::NucleusRpcChannel;
use vrs_runtime::{opaque::Block, AccountId, Balance, Nonce};

pub use sc_rpc_api::DenyUnsafe;

#[derive(Clone)]
pub struct NucleusDeps {
    pub rpc_channel: NucleusRpcChannel,
    pub node_id: PeerId,
    pub home_dir: PathBuf,
}

/// Full client dependencies.
pub struct FullDeps<C, P, B> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// blocks backend
    pub backend: Arc<B>,
    /// Nucleus cage connector
    pub nucleus: Option<NucleusDeps>,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P, B>(
    deny_unsafe: DenyUnsafe,
    deps: FullDeps<C, P, B>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    P: TransactionPool<Block = Block, Hash = <Block as sp_runtime::traits::Block>::Hash> + 'static,
    B: sc_client_api::Backend<Block> + Send + Sync + 'static,
    B::State: sc_client_api::backend::StateBackend<sp_runtime::traits::HashingFor<Block>>,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
    C: Send + Sync + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce> + 'static,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance> + 'static,
    C::Api: vrs_nucleus_runtime_api::NucleusApi<Block> + 'static,
    C::Api: vrs_restaking_runtime_api::VrsRestakingRuntimeApi<Block, AccountId>,
    C::Api: BlockBuilder<Block> + 'static,
{
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
    use substrate_frame_rpc_system::{System, SystemApiServer};
    use vrs_nucleus_rpc_proxy::{NucleusEntry, NucleusRpcServer};

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
        pool,
        backend: _backend,
        nucleus,
    } = deps;
    module.merge(System::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
    module.merge(TransactionPayment::new(client.clone()).into_rpc())?;

    if let Some(nucleus) = nucleus {
        let NucleusDeps {
            rpc_channel,
            node_id,
            home_dir,
        } = nucleus;
        module.merge(NucleusEntry::new(rpc_channel, client, pool, node_id, home_dir).into_rpc())?;
    }

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
