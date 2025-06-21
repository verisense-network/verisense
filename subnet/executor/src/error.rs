use serde::{Deserialize, Serialize};
use vrs_core_sdk::codec::{Decode, Encode};

/// below is a2a spec error codes
/// -32001	TaskNotFoundError	Task not found	The specified task id does not correspond to an existing or active task. It might be invalid, expired, or already completed and purged.
/// -32002	TaskNotCancelableError	Task cannot be canceled	An attempt was made to cancel a task that is not in a cancelable state (e.g., it has already reached a terminal state like completed, failed, or canceled).
/// -32003	PushNotificationNotSupportedError	Push Notification is not supported	Client attempted to use push notification features (e.g., tasks/pushNotificationConfig/set) but the server agent does not support them (i.e., AgentCard.capabilities.pushNotifications is false).
/// -32004	UnsupportedOperationError	This operation is not supported	The requested operation or a specific aspect of it (perhaps implied by parameters) is not supported by this server agent implementation. Broader than just method not found.
/// -32005	ContentTypeNotSupportedError	Incompatible content types	A Media Type provided in the request's message.parts (or implied for an artifact) is not supported by the agent or the specific skill being invoked.
/// -32006	InvalidAgentResponseError	Invalid agent response type	Agent generated an invalid response for the requested method
///
/// -32070 ~ -32079 are reserved for WASM errors.
#[derive(Clone, Debug, Deserialize, Serialize, Encode, Decode, Eq, PartialEq)]
pub struct NucleusError {
    pub code: i32,
    pub message: String,
}

impl NucleusError {
    pub fn new(code: i32, message: String) -> Self {
        Self { code, message }
    }

    pub fn abi(msg: impl ToString) -> Self {
        Self {
            code: -32041,
            message: msg.to_string(),
        }
    }

    pub fn nucleus_not_found() -> Self {
        Self {
            code: -32042,
            message: "Nucleus not found".to_string(),
        }
    }

    pub fn node(msg: impl ToString) -> Self {
        Self {
            code: -32043,
            message: msg.to_string(),
        }
    }

    pub fn params(msg: impl ToString) -> Self {
        Self {
            code: -32044,
            message: msg.to_string(),
        }
    }
}

impl Into<jsonrpsee::types::error::ErrorObjectOwned> for NucleusError {
    fn into(self) -> jsonrpsee::types::error::ErrorObjectOwned {
        jsonrpsee::types::error::ErrorObjectOwned::owned(self.code, self.message, None::<()>)
    }
}

impl Into<jsonrpc_core::types::Error> for NucleusError {
    fn into(self) -> jsonrpc_core::types::Error {
        jsonrpc_core::types::Error {
            code: jsonrpc_core::types::ErrorCode::ServerError(self.code.into()),
            message: self.message,
            data: None,
        }
    }
}
