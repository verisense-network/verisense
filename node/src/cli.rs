use clap::Parser;
use sc_cli::RunCmd;

#[derive(Debug, clap::Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,
    #[clap(flatten)]
    pub tss: TssCmd,

    #[clap(flatten)]
    pub run: RunCmd,
}

#[derive(Debug, clap::Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommand {
    /// Key management cli utilities
    #[command(subcommand)]
    Key(sc_cli::KeySubcommand),

    /// Build a chain specification.
    BuildSpec(sc_cli::BuildSpecCmd),

    /// Validate blocks.
    CheckBlock(sc_cli::CheckBlockCmd),

    /// Export blocks.
    ExportBlocks(sc_cli::ExportBlocksCmd),

    /// Export the state of a given block into a chain spec.
    ExportState(sc_cli::ExportStateCmd),

    /// Import blocks.
    ImportBlocks(sc_cli::ImportBlocksCmd),

    /// Remove the whole chain.
    PurgeChain(sc_cli::PurgeChainCmd),

    /// Revert the chain to a previous state.
    Revert(sc_cli::RevertCmd),

    /// Db meta columns information.
    ChainInfo(sc_cli::ChainInfoCmd),
}
use clap::ArgGroup;
use libp2p::Multiaddr;
use libp2p::PeerId;
use sc_service::config::MultiaddrWithPeerId;

#[derive(Debug, Clone, Parser)]
#[command(group = ArgGroup::new("tss").args(&["tss-coordinator", "tss-signer", "tss-node"]).required(false))]
pub struct TssCmd {
    /// Launch the coordinator on the specified port.
    /// This also initializes the signer and node using the multiaddress format:
    /// `/ip4/127.0.0.1/tcp/{port}/p2p/{node_id}`.
    #[arg(long = "tss-coordinator", value_name = "Coordinator Port")]
    pub coordinator: Option<u16>,

    /// Initialize the signer using the provided coordinator multiaddress.
    /// This option also triggers the corresponding node initialization.
    #[arg(long = "tss-signer", value_name = "Coordinator MultiAddr")]
    pub signer: Option<MultiaddrWithPeerId>,

    /// Launch the node with the specified coordinator multiaddress.
    #[arg(long = "tss-node", value_name = "Coordinator MultiAddr")]
    pub node: Option<MultiaddrWithPeerId>,
}

impl TssCmd {
    pub fn coordinator_multiaddr(&self) -> Option<Multiaddr> {
        if let Some(coordinator) = self.coordinator {
            Some(
                format!("/ip4/127.0.0.1/tcp/{}", coordinator)
                    .parse::<Multiaddr>()
                    .unwrap(),
            )
        } else if let Some(coordinator_addr) = self.signer.as_ref() {
            Some(coordinator_addr.multiaddr.clone().into())
        } else if let Some(coordinator_addr) = self.node.as_ref() {
            Some(coordinator_addr.multiaddr.clone().into())
        } else {
            None
        }
    }
    pub fn peer_id(&self) -> Option<PeerId> {
        if let Some(coordinator_addr) = self.signer.as_ref() {
            Some(coordinator_addr.peer_id.into())
        } else if let Some(coordinator_addr) = self.node.as_ref() {
            Some(coordinator_addr.peer_id.into())
        } else {
            None
        }
    }
}
