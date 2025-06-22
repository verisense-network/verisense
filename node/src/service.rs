//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use crate::cli::{ExtraConfig, TssCmd};
use futures::{prelude::*, FutureExt};
use sc_client_api::{Backend, BlockBackend, HeaderBackend};
use sc_network::{event::Event, NetworkEventStream};
use sc_service::{error::Error as ServiceError, Configuration, TaskManager, WarpSyncParams};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sc_transaction_pool_api::OffchainTransactionPoolFactory;
use sp_api::ProvideRuntimeApi;
use sp_authority_discovery::AuthorityId;
use sp_core::crypto::Ss58Codec;
use sp_runtime::key_types::AUTHORITY_DISCOVERY;
use std::{collections::HashSet, sync::Arc, time::Duration};
use vrs_runtime::{self, opaque::Block, RuntimeApi};
use vrs_tss::TssIdentity;
use vrs_tss::VrsTssValidatorIdentity;
use vrs_validator_runtime_api::ValidatorRuntimeApi;

pub(crate) type FullClient = sc_service::TFullClient<
    Block,
    RuntimeApi,
    sc_executor::WasmExecutor<sp_io::SubstrateHostFunctions>,
>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;
type FullGrandpaBlockImport =
    sc_consensus_grandpa::GrandpaBlockImport<FullBackend, Block, FullClient, FullSelectChain>;

const GRANDPA_JUSTIFICATION_PERIOD: u32 = 512;

/// Creates a new partial node.
pub fn new_partial(
    config: &Configuration,
) -> Result<
    sc_service::PartialComponents<
        FullClient,
        FullBackend,
        FullSelectChain,
        sc_consensus::DefaultImportQueue<Block>,
        sc_transaction_pool::FullPool<Block, FullClient>,
        (
            (
                sc_consensus_babe::BabeBlockImport<Block, FullClient, FullGrandpaBlockImport>,
                sc_consensus_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
                sc_consensus_babe::BabeLink<Block>,
                sc_consensus_babe::BabeWorkerHandle<Block>,
            ),
            sc_consensus_grandpa::SharedVoterState,
            Option<Telemetry>,
        ),
    >,
    ServiceError,
> {
    let telemetry = config
        .telemetry_endpoints
        .clone()
        .filter(|x| !x.is_empty())
        .map(|endpoints| -> Result<_, sc_telemetry::Error> {
            let worker = TelemetryWorker::new(16)?;
            let telemetry = worker.handle().new_telemetry(endpoints);
            Ok((worker, telemetry))
        })
        .transpose()?;

    let executor = sc_service::new_wasm_executor(&config);

    let (client, backend, keystore_container, task_manager) =
        sc_service::new_full_parts::<Block, RuntimeApi, _>(
            config,
            telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
            executor,
        )?;
    let client = Arc::new(client);

    let telemetry = telemetry.map(|(worker, telemetry)| {
        task_manager
            .spawn_handle()
            .spawn("telemetry", None, worker.run());
        telemetry
    });

    let select_chain = sc_consensus::LongestChain::new(backend.clone());
    let transaction_pool = sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_essential_handle(),
        client.clone(),
    );

    let (grandpa_block_import, grandpa_link) = sc_consensus_grandpa::block_import(
        client.clone(),
        GRANDPA_JUSTIFICATION_PERIOD,
        &(client.clone() as Arc<_>),
        select_chain.clone(),
        telemetry.as_ref().map(|x| x.handle()),
    )?;
    let justification_import = grandpa_block_import.clone();
    let (block_import, babe_link) = sc_consensus_babe::block_import(
        sc_consensus_babe::configuration(&*client)?,
        grandpa_block_import,
        client.clone(),
    )?;
    let slot_duration = babe_link.config().slot_duration();
    let (import_queue, babe_worker_handle) = sc_consensus_babe::import_queue(
        sc_consensus_babe::ImportQueueParams {
            link: babe_link.clone(),
            block_import: block_import.clone(),
            justification_import: Some(Box::new(justification_import)),
            client: client.clone(),
            select_chain: select_chain.clone(),
            create_inherent_data_providers: move |_, ()| async move {
                let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

                let slot =
                    sp_consensus_babe::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
                        *timestamp,
                        slot_duration,
                    );

                Ok((slot, timestamp))
            },
            spawner: &task_manager.spawn_essential_handle(),
            registry: config.prometheus_registry(),
            telemetry: telemetry.as_ref().map(|x| x.handle()),
            offchain_tx_pool_factory: OffchainTransactionPoolFactory::new(transaction_pool.clone()),
        },
    )?;
    let import_setup = (block_import, grandpa_link, babe_link, babe_worker_handle);
    let shared_voter_state = sc_consensus_grandpa::SharedVoterState::empty();

    Ok(sc_service::PartialComponents {
        client,
        backend,
        task_manager,
        keystore_container,
        select_chain,
        import_queue,
        transaction_pool,
        other: (import_setup, shared_voter_state, telemetry),
    })
}

pub fn new_full<
    N: sc_network::NetworkBackend<Block, <Block as sp_runtime::traits::Block>::Hash>,
>(
    config: Configuration,
    tss_config: TssCmd,
    extra_config: ExtraConfig,
) -> Result<TaskManager, ServiceError> {
    let network_config = config.network.clone();

    let sc_service::PartialComponents {
        client,
        backend,
        mut task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        other:
            (
                (block_import, grandpa_link, babe_link, babe_worker_handle),
                shared_vote_state,
                mut telemetry,
            ),
    } = new_partial(&config)?;

    let mut net_config = sc_network::config::FullNetworkConfiguration::<
        Block,
        <Block as sp_runtime::traits::Block>::Hash,
        N,
    >::new(&config.network);
    let node_id = config
        .network
        .node_key
        .clone()
        .into_keypair()
        .unwrap()
        .public()
        .to_peer_id();
    let auth_disc_public_addresses = config.network.public_addresses.clone();
    let metrics = N::register_notification_metrics(config.prometheus_registry());
    let peer_store_handle = net_config.peer_store_handle();
    let grandpa_protocol_name = sc_consensus_grandpa::protocol_standard_name(
        &client
            .block_hash(0)
            .ok()
            .flatten()
            .expect("Genesis block exists; qed"),
        &config.chain_spec,
    );
    let (grandpa_protocol_config, grandpa_notification_service) =
        sc_consensus_grandpa::grandpa_peers_set_config::<_, N>(
            grandpa_protocol_name.clone(),
            metrics.clone(),
            peer_store_handle,
        );
    net_config.add_notification_protocol(grandpa_protocol_config);

    let warp_sync = Arc::new(sc_consensus_grandpa::warp_proof::NetworkProvider::new(
        backend.clone(),
        grandpa_link.shared_authority_set().clone(),
        Vec::default(),
    ));

    // --- add nucleus-p2p subprotocol
    let (reqres_sender, reqres_receiver) = async_channel::bounded(1024);
    let nucleus_p2p_reqres_config = N::request_response_config(
        sc_network::types::ProtocolName::Static("/nucleus/p2p/reqres"),
        vec![],
        1024 * 1024,
        16 * 1024 * 1024,
        Duration::from_secs(20),
        Some(reqres_sender),
    );
    net_config.add_request_response_protocol(nucleus_p2p_reqres_config);

    let (gluon_relay_sender, gluon_relay_receiver) = async_channel::bounded(1024);
    let gluon_relay_protocol = N::request_response_config(
        sc_network::types::ProtocolName::Static("/gluon/1"),
        vec![],
        10 * 1024 * 1024,
        16 * 1024 * 1024,
        Duration::from_secs(20),
        Some(gluon_relay_sender),
    );
    net_config.add_request_response_protocol(gluon_relay_protocol);

    let (network, system_rpc_tx, tx_handler_controller, network_starter, sync_service) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            net_config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            block_announce_validator_builder: None,
            warp_sync_params: Some(WarpSyncParams::WithProvider(warp_sync)),
            block_relay: None,
            metrics,
        })?;

    // launch authority discovery worker
    let discovery_mode = if config.role.is_authority() {
        sc_authority_discovery::Role::PublishAndDiscover(keystore_container.keystore())
    } else {
        sc_authority_discovery::Role::Discover
    };

    let dht_event_stream = network
        .event_stream("authority-discovery")
        .filter_map(|e| async move {
            match e {
                Event::Dht(e) => Some(e),
                _ => None,
            }
        });
    let (authority_discovery_worker, authority_discovery_service) =
        sc_authority_discovery::new_worker_and_service_with_config(
            sc_authority_discovery::WorkerConfig {
                publish_non_global_ips: true,
                max_publish_interval: Duration::from_secs(60),
                max_query_interval: Duration::from_secs(60),
                public_addresses: auth_disc_public_addresses,
                ..Default::default()
            },
            client.clone(),
            Arc::new(network.clone()),
            Box::pin(dht_event_stream),
            discovery_mode,
            config.prometheus_registry().cloned(),
        );
    task_manager.spawn_handle().spawn(
        "authority-discovery-worker",
        Some("networking"),
        authority_discovery_worker.run(),
    );

    if config.offchain_worker.enabled {
        task_manager.spawn_handle().spawn(
            "offchain-workers-runner",
            "offchain-worker",
            sc_offchain::OffchainWorkers::new(sc_offchain::OffchainWorkerOptions {
                runtime_api_provider: client.clone(),
                is_validator: config.role.is_authority(),
                keystore: Some(keystore_container.keystore()),
                offchain_db: backend.offchain_storage(),
                transaction_pool: Some(OffchainTransactionPoolFactory::new(
                    transaction_pool.clone(),
                )),
                network_provider: Arc::new(network.clone()),
                enable_http_requests: true,
                custom_extensions: |_| vec![],
            })
            .run(client.clone(), task_manager.spawn_handle())
            .boxed(),
        );
    }

    let role = config.role.clone();
    let authority_discovery = authority_discovery_service.clone();
    let force_authoring = config.force_authoring;
    let backoff_authoring_blocks: Option<()> = None;
    let name = config.network.node_name.clone();
    let sys_rpc_port = config.rpc_port;
    let enable_grandpa = !config.disable_grandpa;
    let prometheus_registry = config.prometheus_registry().cloned();
    let nucleus_home_dir = config.data_path.as_path().join("nucleus");
    let tss_path = config.base_path.path().join("veritss");
    let author = keystore_container
        .keystore()
        .sr25519_public_keys(sp_core::crypto::key_types::BABE)
        .first()
        .copied();
    let best_hash = client.info().best_hash;
    let api = client.runtime_api();
    let maybe_validator = author
        .map(|author| {
            api.lookup_active_validator(
                best_hash,
                sp_core::crypto::key_types::BABE,
                author.to_vec(),
            )
            .expect("couldn't load runtime api")
        })
        .flatten();
    init_nucleus_home(&nucleus_home_dir);
    // TODO config the capacity of pending requests
    let (nucleus_rpc_tx, nucleus_rpc_rx) = tokio::sync::mpsc::channel(10000);
    let network_service = Arc::new(network.clone());
    let rpc_extensions_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();
        let select_chain = select_chain.clone();
        let keystore = keystore_container.keystore();
        let rpc_backend = backend.clone();
        let nucleus_rpc_tx = nucleus_rpc_tx.clone();
        let node_id = node_id.clone();
        let nucleus_home_dir = nucleus_home_dir.clone();
        let network_service = network_service.clone();
        let maybe_validator = maybe_validator.clone();

        move |deny_unsafe, _subscription_executor: sc_rpc::SubscriptionTaskExecutor| {
            let deps = crate::rpc::FullDeps {
                client: client.clone(),
                pool: pool.clone(),
                select_chain: select_chain.clone(),
                babe: crate::rpc::BabeDeps {
                    keystore: keystore.clone(),
                    babe_worker_handle: babe_worker_handle.clone(),
                },
                backend: rpc_backend.clone(),
                nucleus: crate::rpc::NucleusDeps {
                    rpc_channel: nucleus_rpc_tx.clone(),
                    node_id: node_id.clone(),
                    home_dir: nucleus_home_dir.clone(),
                    network: network_service.clone(),
                    authority_discover: authority_discovery.clone(),
                    maybe_validator: maybe_validator.clone(),
                },
            };
            crate::rpc::create_full(deny_unsafe, deps).map_err(Into::into)
        }
    };
    let args = vrs_rpc_server::NucleusRpcServerArgs {
        sender: nucleus_rpc_tx,
        client: client.clone(),
        relayer: vrs_gluon_relayer::Relayer::new(
            client.clone(),
            network_service.clone(),
            authority_discovery_service.clone(),
        ),
        maybe_validator,
        pool: transaction_pool.clone(),
        node_id: node_id.clone(),
        nucleus_home_dir: nucleus_home_dir.clone(),
        sys_rpc_port,
        entry_rpc_port: extra_config.extra_rpc_port.unwrap_or(9955),
    };
    task_manager.spawn_essential_handle().spawn_blocking(
        "nucleus-rpc-server",
        None,
        vrs_rpc_server::start_nucleus_rpc(args),
    );

    let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        network: network.clone(),
        client: client.clone(),
        keystore: keystore_container.keystore(),
        task_manager: &mut task_manager,
        transaction_pool: transaction_pool.clone(),
        rpc_builder: Box::new(rpc_extensions_builder),
        backend,
        system_rpc_tx,
        tx_handler_controller,
        sync_service: sync_service.clone(),
        config,
        telemetry: telemetry.as_mut(),
    })?;
    // tss by shiotoli
    let best_block_hash = client.info().best_hash;
    let validators = client
        .runtime_api()
        .get_genesis_validators(best_block_hash)
        .map_err(|e| sc_service::Error::Other(format!("Failed to get all validators: {}", e)))?;
    let whitelisted_ids = validators
        .iter()
        .map(|id| TssIdentity(id.clone()))
        .collect::<HashSet<_>>();

    let start_tss = whitelisted_ids.len() >= 2;
    if role.is_authority() && start_tss {
        let tss_keystore =
            vrs_tss::TssKeystore::new(keystore_container.keystore(), AUTHORITY_DISCOVERY).map_err(
                |e| sc_service::Error::Other(format!("Failed to initialize signer: {}", e)),
            )?;
        // if the node is an authority, it will run a signer service
        // since we cannot get the sudo account from the chain spec, we start the coordinator for all authorities
        let min_signers = whitelisted_ids.len() as u16 / 2 + 1;
        // if the coordinator is set, start the coordinator
        if let Some(coordinator_port) = tss_config.coordinator {
            log::info!(
                "start coordinator with {} whitelisted ids, min signers: {}, listen port: {}",
                whitelisted_ids.len(),
                min_signers,
                coordinator_port
            );
            let node_key_pair = libp2p::identity::Keypair::ed25519_from_bytes(
                network_config
                    .node_key
                    .clone()
                    .into_keypair()?
                    .secret()
                    .to_bytes(),
            )
            .unwrap();
            assert_eq!(
                node_key_pair.public().to_peer_id().to_string(),
                node_id.to_string()
            );
            tracing::info!(
                "whitelisted ids: {:?}, node peer id: {:?}",
                whitelisted_ids,
                node_key_pair.public().to_peer_id()
            );

            let coordinator =
                vrs_tss::coordinator::Coordinator::<vrs_tss::VrsTssValidatorIdentity>::new(
                    node_key_pair,
                    tss_path.clone(),
                    Some(whitelisted_ids.clone()),
                    coordinator_port,
                    Some(min_signers),
                )
                .map_err(|e| {
                    sc_service::Error::Other(format!("Failed to initialize coordinator: {}", e))
                })?;
            task_manager
                .spawn_essential_handle()
                .spawn_blocking("coordinator", None, async move {
                    let r = coordinator.start_listening().await;
                    if let Err(e) = r {
                        tracing::error!("coordinator start listening failed: {:?}", e);
                    }
                });
            let signer = vrs_tss::signer::Signer::<vrs_tss::VrsTssValidatorIdentity>::new(
                tss_keystore,
                tss_path.clone(),
                tss_config.coordinator_multiaddr().unwrap(),
                node_id.into(),
                |_, _| true,
            )
            .map_err(|e| sc_service::Error::Other(format!("Failed to initialize signer: {}", e)))?;
            task_manager
                .spawn_essential_handle()
                .spawn_blocking("signer", None, async move {
                    let r = signer.start_listening().await;
                    if let Err(e) = r {
                        tracing::error!("signer start listening failed: {:?}", e);
                    }
                });
        } else if let Some(bootnode) = tss_config.signer.as_ref() {
            log::info!("start signer with coordinator multiaddr: {}", bootnode);
            let coordinator_ip_addr = bootnode.multiaddr.clone();
            let coordinator_peer_id = bootnode.peer_id;
            let signer = vrs_tss::signer::Signer::<vrs_tss::VrsTssValidatorIdentity>::new(
                tss_keystore,
                tss_path.clone(),
                coordinator_ip_addr.into(),
                coordinator_peer_id.into(),
                |_, _| true,
            )
            .map_err(|e| sc_service::Error::Other(format!("Failed to initialize signer: {}", e)))?;
            task_manager
                .spawn_essential_handle()
                .spawn_blocking("signer", None, async move {
                    let r = signer.start_listening().await;
                    if let Err(e) = r {
                        tracing::error!("signer start listening failed: {:?}", e);
                    }
                });
        }
    }
    // new tss node
    let tss_node = if start_tss && tss_config.coordinator_multiaddr().is_some() {
        let tss_keystore =
            vrs_tss::TssKeystore::new(keystore_container.keystore(), AUTHORITY_DISCOVERY).map_err(
                |e| sc_service::Error::Other(format!("Failed to initialize signer: {}", e)),
            )?;
        let coordinator_multiaddr = tss_config.coordinator_multiaddr().unwrap();
        let coordinator_peer_id = if let Some(peer_id) = tss_config.peer_id() {
            peer_id
        } else {
            node_id.into()
        };
        let tss_node = vrs_tss::node::Node::<VrsTssValidatorIdentity>::new(
            tss_keystore,
            tss_path.clone(),
            coordinator_multiaddr,
            coordinator_peer_id,
        )
        .map_err(|e| sc_service::Error::Other(format!("Failed to initialize tss node: {}", e)))?;
        let tss_node = Arc::new(vrs_tss::NodeRuntime::new(
            tss_node,
            Some(tokio::time::Duration::from_secs(5)),
        ));
        tss_node
    } else {
        println!("tss_path: {:?}", tss_path);
        if !tss_path.exists() {
            std::fs::create_dir_all(tss_path.clone()).unwrap();
        }
        Arc::new(vrs_tss::NodeRuntime::new_empty(&tss_path))
    };
    // tss finished

    if role.is_authority() {
        let node_key_pair = libp2p::identity::Keypair::ed25519_from_bytes(
            network_config
                .node_key
                .clone()
                .into_keypair()?
                .secret()
                .to_bytes(),
        )
        .unwrap();

        // FIXME
        let best_hash = client.info().best_hash;
        let validators = client
            .runtime_api()
            .get_current_validators(best_hash)
            .unwrap();
        let validators: Vec<AuthorityId> = validators
            .into_iter()
            .map(|a| AuthorityId::from_ss58check(a.to_ss58check().as_str()).unwrap())
            .collect::<Vec<AuthorityId>>();

        let (p2p_cage_tx, p2p_cage_rx) = tokio::sync::mpsc::channel(10000);

        let (cage_p2p_tx, cage_p2p_rx) = tokio::sync::mpsc::channel(10000);
        let params = vrs_nucleus_network::P2pParams {
            keystore: keystore_container.keystore(),
            reqres_receiver,
            client: client.clone(),
            node_key_pair,
            net_service: network.clone(),
            p2p_cage_tx,
            cage_p2p_rx,
            controller: sp_keyring::AccountKeyring::Alice.to_account_id(),
            authorities: validators,
            authority_discovery: authority_discovery_service.clone(),
            _phantom: std::marker::PhantomData,
        };

        task_manager.spawn_essential_handle().spawn_blocking(
            "nucleus-p2p",
            None,
            vrs_nucleus_network::start_nucleus_p2p(params),
        );

        // launch nucleus cage
        let params = vrs_nucleus_cage::CageParams {
            client: client.clone(),
            keystore: keystore_container.keystore(),
            transaction_pool: transaction_pool.clone(),
            authority_discovery: authority_discovery_service.clone(),
            nucleus_signal: nucleus_rpc_rx,
            net_service: network.clone(),
            gluon_relay_rx: gluon_relay_receiver,
            tss_node,
            nucleus_home_dir: nucleus_home_dir.clone(),
            p2p_cage_rx,
            cage_p2p_tx,
            _phantom: std::marker::PhantomData,
        };
        task_manager.spawn_essential_handle().spawn_blocking(
            "nucleus-cage",
            None,
            vrs_nucleus_cage::start_nucleus_cage(params),
        );
    }

    if role.is_authority() {
        let proposer = sc_basic_authorship::ProposerFactory::new(
            task_manager.spawn_handle(),
            client.clone(),
            transaction_pool.clone(),
            prometheus_registry.as_ref(),
            telemetry.as_ref().map(|x| x.handle()),
        );

        let client_clone = client.clone();
        let slot_duration = babe_link.config().slot_duration();
        let babe_config = sc_consensus_babe::BabeParams {
            keystore: keystore_container.keystore(),
            client: client.clone(),
            select_chain,
            env: proposer,
            block_import,
            sync_oracle: sync_service.clone(),
            justification_sync_link: sync_service.clone(),
            create_inherent_data_providers: move |parent, ()| {
                let client_clone = client_clone.clone();
                async move {
                    let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

                    let slot =
                        sp_consensus_babe::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
                            *timestamp,
                            slot_duration,
                        );

                    let storage_proof =
                        sp_transaction_storage_proof::registration::new_data_provider(
                            &*client_clone,
                            &parent,
                        )?;

                    Ok((slot, timestamp, storage_proof))
                }
            },
            force_authoring,
            backoff_authoring_blocks,
            babe_link,
            block_proposal_slot_portion: sc_consensus_babe::SlotProportion::new(0.5),
            max_block_proposal_slot_portion: None,
            telemetry: telemetry.as_ref().map(|x| x.handle()),
        };

        let babe = sc_consensus_babe::start_babe(babe_config)?;
        task_manager.spawn_essential_handle().spawn_blocking(
            "babe-proposer",
            Some("block-authoring"),
            babe,
        );
    }

    if enable_grandpa {
        // if the node isn't actively participating in consensus then it doesn't
        // need a keystore, regardless of which protocol we use below.
        let keystore = if role.is_authority() {
            Some(keystore_container.keystore())
        } else {
            None
        };

        let grandpa_config = sc_consensus_grandpa::Config {
            // FIXME #1578 make this available through chainspec
            gossip_duration: Duration::from_millis(333),
            justification_generation_period: GRANDPA_JUSTIFICATION_PERIOD,
            name: Some(name),
            observer_enabled: false,
            keystore,
            local_role: role.clone(),
            telemetry: telemetry.as_ref().map(|x| x.handle()),
            protocol_name: grandpa_protocol_name,
        };

        // start the full GRANDPA voter
        // NOTE: non-authorities could run the GRANDPA observer protocol, but at
        // this point the full voter should provide better guarantees of block
        // and vote data availability than the observer. The observer has not
        // been tested extensively yet and having most nodes in a network run it
        // could lead to finality stalls.
        let grandpa_config = sc_consensus_grandpa::GrandpaParams {
            config: grandpa_config,
            link: grandpa_link,
            network,
            sync: Arc::new(sync_service),
            notification_service: grandpa_notification_service,
            voting_rule: sc_consensus_grandpa::VotingRulesBuilder::default().build(),
            prometheus_registry,
            shared_voter_state: sc_consensus_grandpa::SharedVoterState::empty(),
            telemetry: telemetry.as_ref().map(|x| x.handle()),
            offchain_tx_pool_factory: OffchainTransactionPoolFactory::new(transaction_pool),
        };

        // the GRANDPA voter task is considered infallible, i.e.
        // if it fails we take down the service with it.
        task_manager.spawn_essential_handle().spawn_blocking(
            "grandpa-voter",
            None,
            sc_consensus_grandpa::run_grandpa_voter(grandpa_config)?,
        );
    }
    network_starter.start_network();
    Ok(task_manager)
}

fn init_nucleus_home<P: AsRef<std::path::Path>>(dir: P) {
    if !std::fs::exists(dir.as_ref()).expect(
        "fail to check nucleus directory, make sure the you have access right on the directory.",
    ) {
        std::fs::create_dir_all(dir.as_ref())
            .inspect_err(|e| log::error!("fail to create nucleus home, due to {:?}", e))
            .expect("fail to create nucleus directory");
    }
}
