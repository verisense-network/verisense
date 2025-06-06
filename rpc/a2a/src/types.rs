// A2A Rust types
// Strictly corresponds to TypeScript definitions in a2a.ts

use codec::Encode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the service provider of an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProvider {
    /// Agent provider's organization name.
    pub organization: String,
    /// Agent provider's URL.
    pub url: String,
}

impl Encode for AgentProvider {
    fn encode(&self) -> Vec<u8> {
        let de: a2a_rs::AgentProvider = self.clone().into();
        de.encode()
    }
}

impl Into<a2a_rs::AgentProvider> for AgentProvider {
    fn into(self) -> a2a_rs::AgentProvider {
        a2a_rs::AgentProvider {
            organization: self.organization.as_bytes().to_vec(),
            url: self.url.as_bytes().to_vec(),
        }
    }
}

impl From<a2a_rs::AgentProvider> for AgentProvider {
    fn from(provider: a2a_rs::AgentProvider) -> Self {
        Self {
            organization: String::from_utf8(provider.organization).unwrap_or_default(),
            url: String::from_utf8(provider.url).unwrap_or_default(),
        }
    }
}

/// Defines optional capabilities supported by an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCapabilities {
    /// true if the agent supports SSE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming: Option<bool>,
    /// true if the agent can notify updates to client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_notifications: Option<bool>,
    /// true if the agent exposes status change history for tasks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_transition_history: Option<bool>,
}

impl Encode for AgentCapabilities {
    fn encode(&self) -> Vec<u8> {
        let de: a2a_rs::AgentCapabilities = self.clone().into();
        de.encode()
    }
}

impl Into<a2a_rs::AgentCapabilities> for AgentCapabilities {
    fn into(self) -> a2a_rs::AgentCapabilities {
        a2a_rs::AgentCapabilities {
            streaming: self.streaming,
            push_notifications: self.push_notifications,
            state_transition_history: self.state_transition_history,
        }
    }
}

impl From<a2a_rs::AgentCapabilities> for AgentCapabilities {
    fn from(capabilities: a2a_rs::AgentCapabilities) -> Self {
        Self {
            streaming: capabilities.streaming,
            push_notifications: capabilities.push_notifications,
            state_transition_history: capabilities.state_transition_history,
        }
    }
}

/// Represents a unit of capability that an agent can perform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSkill {
    /// Unique identifier for the agent's skill.
    pub id: String,
    /// Human readable name of the skill.
    pub name: String,
    /// Description of the skill - will be used by the client or a human
    /// as a hint to understand what the skill does.
    pub description: String,
    /// Set of tagwords describing classes of capabilities for this specific skill.
    pub tags: Vec<String>,
    /// The set of example scenarios that the skill can perform.
    /// Will be used by the client as a hint to understand how the skill can be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<String>>,
    /// The set of interaction modes that the skill supports
    /// (if different than the default).
    /// Supported mime types for input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_modes: Option<Vec<String>>,
    /// Supported mime types for output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_modes: Option<Vec<String>>,
}

impl Into<a2a_rs::AgentSkill> for AgentSkill {
    fn into(self) -> a2a_rs::AgentSkill {
        a2a_rs::AgentSkill {
            id: self.id.as_bytes().to_vec(),
            name: self.name.as_bytes().to_vec(),
            description: self.description.as_bytes().to_vec(),
            tags: self
                .tags
                .into_iter()
                .map(|s| s.as_bytes().to_vec())
                .collect(),
            examples: self
                .examples
                .map(|ex| ex.into_iter().map(|s| s.as_bytes().to_vec()).collect()),
            input_modes: self
                .input_modes
                .map(|modes| modes.into_iter().map(|s| s.as_bytes().to_vec()).collect()),
            output_modes: self
                .output_modes
                .map(|modes| modes.into_iter().map(|s| s.as_bytes().to_vec()).collect()),
        }
    }
}

impl From<a2a_rs::AgentSkill> for AgentSkill {
    fn from(skill: a2a_rs::AgentSkill) -> Self {
        Self {
            id: String::from_utf8(skill.id).unwrap_or_default(),
            name: String::from_utf8(skill.name).unwrap_or_default(),
            description: String::from_utf8(skill.description).unwrap_or_default(),
            tags: skill
                .tags
                .into_iter()
                .map(|s| String::from_utf8(s).unwrap_or_default())
                .collect(),
            examples: skill.examples.map(|ex| {
                ex.into_iter()
                    .map(|s| String::from_utf8(s).unwrap_or_default())
                    .collect()
            }),
            input_modes: skill.input_modes.map(|modes| {
                modes
                    .into_iter()
                    .map(|s| String::from_utf8(s).unwrap_or_default())
                    .collect()
            }),
            output_modes: skill.output_modes.map(|modes| {
                modes
                    .into_iter()
                    .map(|s| String::from_utf8(s).unwrap_or_default())
                    .collect()
            }),
        }
    }
}

/// An AgentCard conveys key information:
/// - Overall details (version, name, description, uses)
/// - Skills: A set of capabilities the agent can perform
/// - Default modalities/content types supported by the agent.
/// - Authentication requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCard {
    /// Human readable name of the agent.
    pub name: String,
    /// A human-readable description of the agent. Used to assist users and
    /// other agents in understanding what the agent can do.
    pub description: String,
    /// A URL to the address the agent is hosted at.
    pub url: String,
    /// A URL to an icon for the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// The service provider of the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<AgentProvider>,
    /// The version of the agent - format is up to the provider.
    pub version: String,
    /// A URL to documentation for the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
    /// Optional capabilities supported by the agent.
    pub capabilities: AgentCapabilities,
    /// Security scheme details used for authenticating with this agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_schemes: Option<HashMap<String, SecurityScheme>>,
    /// Security requirements for contacting the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<HashMap<String, Vec<String>>>>,
    /// The set of interaction modes that the agent supports across all skills.
    /// Supported mime types for input.
    pub default_input_modes: Vec<String>,
    /// Supported mime types for output.
    pub default_output_modes: Vec<String>,
    /// Skills are a unit of capability that an agent can perform.
    pub skills: Vec<AgentSkill>,
    /// true if the agent supports providing an extended agent card when the user is authenticated.
    /// Defaults to false if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_authenticated_extended_card: Option<bool>,
}

impl Into<a2a_rs::AgentCard> for AgentCard {
    fn into(self) -> a2a_rs::AgentCard {
        a2a_rs::AgentCard {
            name: self.name.as_bytes().to_vec(),
            description: self.description.as_bytes().to_vec(),
            url: self.url.as_bytes().to_vec(),
            icon_url: self.icon_url.map(|v| v.as_bytes().to_vec()),
            provider: self.provider.map(|p| p.into()),
            version: self.version.as_bytes().to_vec(),
            documentation_url: self.documentation_url.map(|v| v.as_bytes().to_vec()),
            capabilities: self.capabilities.into(),
            security_schemes: self.security_schemes.map(|schemes| {
                schemes
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            k.as_bytes().to_vec(),
                            serde_json::to_vec(&v).expect("must be json"),
                        )
                    })
                    .collect()
            }),
            security: self.security.map(|security| {
                security
                    .into_iter()
                    .map(|map| {
                        map.into_iter()
                            .map(|(k, v)| {
                                (
                                    k.as_bytes().to_vec(),
                                    v.into_iter().map(|s| s.as_bytes().to_vec()).collect(),
                                )
                            })
                            .collect()
                    })
                    .collect()
            }),
            default_input_modes: self
                .default_input_modes
                .into_iter()
                .map(|s| s.as_bytes().to_vec())
                .collect(),
            default_output_modes: self
                .default_output_modes
                .into_iter()
                .map(|s| s.as_bytes().to_vec())
                .collect(),
            skills: self.skills.into_iter().map(|s| s.into()).collect(),
            supports_authenticated_extended_card: self.supports_authenticated_extended_card,
        }
    }
}

impl From<a2a_rs::AgentCard> for AgentCard {
    fn from(card: a2a_rs::AgentCard) -> Self {
        Self {
            name: String::from_utf8(card.name).unwrap_or_default(),
            description: String::from_utf8(card.description).unwrap_or_default(),
            url: String::from_utf8(card.url).unwrap_or_default(),
            icon_url: card
                .icon_url
                .map(|v| String::from_utf8(v).unwrap_or_default()),
            provider: card.provider.map(Into::into),
            version: String::from_utf8(card.version).unwrap_or_default(),
            documentation_url: card
                .documentation_url
                .map(|v| String::from_utf8(v).unwrap_or_default()),
            capabilities: card.capabilities.into(),
            security_schemes: card.security_schemes.map(|schemes| {
                schemes
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            String::from_utf8(k).unwrap_or_default(),
                            serde_json::from_slice(&v).expect("must be json"),
                        )
                    })
                    .collect()
            }),
            security: card.security.map(|security| {
                security
                    .into_iter()
                    .map(|map| {
                        map.into_iter()
                            .map(|(k, v)| {
                                (
                                    String::from_utf8(k).unwrap_or_default(),
                                    v.into_iter()
                                        .map(|s| String::from_utf8(s).unwrap_or_default())
                                        .collect(),
                                )
                            })
                            .collect()
                    })
                    .collect()
            }),
            default_input_modes: card
                .default_input_modes
                .into_iter()
                .map(|s| String::from_utf8(s).unwrap_or_default())
                .collect(),
            default_output_modes: card
                .default_output_modes
                .into_iter()
                .map(|s| String::from_utf8(s).unwrap_or_default())
                .collect(),
            skills: card.skills.into_iter().map(Into::into).collect(),
            supports_authenticated_extended_card: card.supports_authenticated_extended_card,
        }
    }
}

/// Base properties shared by all security schemes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySchemeBase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// API Key security scheme.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeySecurityScheme {
    pub type_: String,
    pub in_: String,
    pub name: String,
    #[serde(flatten)]
    pub base: SecuritySchemeBase,
}

/// HTTP Authentication security scheme.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpAuthSecurityScheme {
    pub type_: String,
    pub scheme: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearer_format: Option<String>,
    #[serde(flatten)]
    pub base: SecuritySchemeBase,
}

/// Configuration details for a supported OAuth Flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationCodeOAuthFlow {
    pub authorization_url: String,
    pub token_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    pub scopes: HashMap<String, String>,
}

/// Configuration details for a supported OAuth Flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCredentialsOAuthFlow {
    pub token_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    pub scopes: HashMap<String, String>,
}

/// Configuration details for a supported OAuth Flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplicitOAuthFlow {
    pub authorization_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    pub scopes: HashMap<String, String>,
}

/// Configuration details for a supported OAuth Flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordOAuthFlow {
    pub token_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    pub scopes: HashMap<String, String>,
}

/// Allows configuration of the supported OAuth Flows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthFlows {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<AuthorizationCodeOAuthFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<ClientCredentialsOAuthFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit: Option<ImplicitOAuthFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<PasswordOAuthFlow>,
}

/// OAuth2.0 security scheme configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2SecurityScheme {
    pub type_: String,
    pub flows: OAuthFlows,
    #[serde(flatten)]
    pub base: SecuritySchemeBase,
}

/// OpenID Connect security scheme configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenIdConnectSecurityScheme {
    pub type_: String,
    pub open_id_connect_url: String,
    #[serde(flatten)]
    pub base: SecuritySchemeBase,
}

/// Mirrors the OpenAPI Security Scheme Object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SecurityScheme {
    #[serde(rename = "apiKey")]
    ApiKey(ApiKeySecurityScheme),
    #[serde(rename = "http")]
    Http(HttpAuthSecurityScheme),
    #[serde(rename = "oauth2")]
    OAuth2(OAuth2SecurityScheme),
    #[serde(rename = "openIdConnect")]
    OpenIdConnect(OpenIdConnectSecurityScheme),
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct JsonRpcError {
//     pub code: i32,
//     pub message: String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub data: Option<serde_json::Value>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum A2aError {
//     JsonParseError(JsonRpcError),
//     InvalidRequestError(JsonRpcError),
//     MethodNotFoundError(JsonRpcError),
//     InvalidParamsError(JsonRpcError),
//     InternalError(JsonRpcError),
//     TaskNotFoundError(JsonRpcError),
//     TaskNotCancelableError(JsonRpcError),
//     PushNotificationNotSupportedError(JsonRpcError),
//     UnsupportedOperationError(JsonRpcError),
//     ContentTypeNotSupportedError(JsonRpcError),
//     InvalidAgentResponseError(JsonRpcError),
// }
