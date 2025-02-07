use tokio::time::Duration;
use tss_sdk::crypto::CryptoType;
use tss_sdk::node::Node;

pub enum NodeRuntime {
    Node {
        tss_node: Node<crate::VrsTssValidatorIdentity>,
        runtime: tokio::runtime::Runtime,
        timeout: Option<Duration>,
    },
    Empty,
}
pub static EMPTY_NODE_ERROR: &str =
    "NodeRuntime is empty. it is probably the number of nodes is not enough";

impl NodeRuntime {
    pub fn new_empty() -> Self {
        NodeRuntime::Empty
    }
    pub fn new(node: Node<crate::VrsTssValidatorIdentity>, timeout: Option<Duration>) -> Self {
        Self::Node {
            tss_node: node,
            runtime: tokio::runtime::Runtime::new().unwrap(),
            timeout,
        }
    }
    pub fn get_public_key(
        &self,
        crypto_type: CryptoType,
        tweak_data: Vec<u8>,
    ) -> Result<Vec<u8>, String> {
        match self {
            NodeRuntime::Node {
                tss_node,
                runtime,
                timeout,
            } => {
                let auto_dkg = runtime
                    .block_on(tss_node.auto_dkg_async(timeout.clone()))
                    .map_err(|e| e.to_string())?;
                let pkid = auto_dkg
                    .get_pkid_by_crypto_type(crypto_type)
                    .map_err(|e| e.to_string())?;
                let public_key = runtime
                    .block_on(tss_node.pk_async(pkid, Some(tweak_data), timeout.clone()))
                    .map_err(|e| e.to_string())?;
                Ok(public_key.group_public_key_tweak)
            }
            NodeRuntime::Empty => {
                return Err(EMPTY_NODE_ERROR.to_string());
            }
        }
    }
    pub fn sign(
        &self,
        crypto_type: CryptoType,
        message: Vec<u8>,
        tweak_data: Vec<u8>,
    ) -> Result<Vec<u8>, String> {
        match self {
            NodeRuntime::Node {
                tss_node,
                runtime,
                timeout,
            } => {
                let auto_dkg = runtime
                    .block_on(tss_node.auto_dkg_async(timeout.clone()))
                    .map_err(|e| e.to_string())?;
                let pkid = auto_dkg
                    .get_pkid_by_crypto_type(crypto_type)
                    .map_err(|e| e.to_string())?;
                let signature = runtime
                    .block_on(tss_node.sign_async(pkid, message, Some(tweak_data), timeout.clone()))
                    .map_err(|e| e.to_string())?;
                Ok(signature.signature())
            }
            NodeRuntime::Empty => {
                return Err(EMPTY_NODE_ERROR.to_string());
            }
        }
    }
}
