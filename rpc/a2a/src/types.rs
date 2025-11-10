// A2A Rust types
// Strictly corresponds to TypeScript definitions in a2a.ts

use codec::Encode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use vrs_primitives::{AccountId, NucleusId};

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
            organization: self.organization,
            url: self.url,
        }
    }
}

impl From<a2a_rs::AgentProvider> for AgentProvider {
    fn from(provider: a2a_rs::AgentProvider) -> Self {
        Self {
            organization: provider.organization,
            url: provider.url,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentExtension {
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

impl Encode for AgentExtension {
    fn encode(&self) -> Vec<u8> {
        let de: a2a_rs::AgentExtension = self.clone().into();
        de.encode()
    }
}

impl Into<a2a_rs::AgentExtension> for AgentExtension {
    fn into(self) -> a2a_rs::AgentExtension {
        a2a_rs::AgentExtension {
            uri: self.uri,
            description: self.description,
            required: self.required,
            params: self
                .params
                .map(|p| serde_json::to_string(&p).expect("must be json")),
        }
    }
}

impl From<a2a_rs::AgentExtension> for AgentExtension {
    fn from(ext: a2a_rs::AgentExtension) -> Self {
        Self {
            uri: ext.uri,
            description: ext.description,
            required: ext.required,
            params: ext
                .params
                .map(|p| serde_json::from_str(&p).expect("must be json")),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<AgentExtension>>,
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
            extensions: self
                .extensions
                .map(|exts| exts.into_iter().map(|ext| ext.into()).collect()),
        }
    }
}

impl From<a2a_rs::AgentCapabilities> for AgentCapabilities {
    fn from(capabilities: a2a_rs::AgentCapabilities) -> Self {
        Self {
            streaming: capabilities.streaming,
            push_notifications: capabilities.push_notifications,
            state_transition_history: capabilities.state_transition_history,
            extensions: capabilities.extensions.map(|exts| {
                exts.into_iter()
                    .map(|ext| AgentExtension::from(ext))
                    .collect()
            }),
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
            id: self.id,
            name: self.name,
            description: self.description,
            tags: self.tags,
            examples: self.examples,
            input_modes: self.input_modes,
            output_modes: self.output_modes,
        }
    }
}

impl From<a2a_rs::AgentSkill> for AgentSkill {
    fn from(skill: a2a_rs::AgentSkill) -> Self {
        Self {
            id: skill.id,
            name: skill.name,
            description: skill.description,
            tags: skill.tags,
            examples: skill.examples,
            input_modes: skill.input_modes,
            output_modes: skill.output_modes,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    pub agent_id: NucleusId,
    pub owner_id: AccountId,
    pub url_verified: bool,
    pub price_rate: u16,
    pub agent_card: AgentCard,
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
            name: self.name,
            description: self.description,
            url: self.url,
            icon_url: self.icon_url,
            provider: self.provider.map(|p| p.into()),
            version: self.version,
            documentation_url: self.documentation_url,
            capabilities: self.capabilities.into(),
            security_schemes: self.security_schemes.map(|schemes| {
                schemes
                    .into_iter()
                    .map(|(k, v)| (k, serde_json::to_string(&v).expect("must be json")))
                    .collect()
            }),
            security: self.security.map(|security| {
                security
                    .into_iter()
                    .map(|map| map.into_iter().map(|(k, v)| (k, v)).collect())
                    .collect()
            }),
            default_input_modes: self.default_input_modes,
            default_output_modes: self.default_output_modes,
            skills: self.skills.into_iter().map(|s| s.into()).collect(),
            supports_authenticated_extended_card: self.supports_authenticated_extended_card,
        }
    }
}

impl From<a2a_rs::AgentCard> for AgentCard {
    fn from(agent_card: a2a_rs::AgentCard) -> AgentCard {
        AgentCard {
            name: agent_card.name,
            description: agent_card.description,
            url: agent_card.url,
            icon_url: agent_card.icon_url,
            provider: agent_card.provider.map(|p| p.into()),
            version: agent_card.version,
            documentation_url: agent_card.documentation_url,
            capabilities: agent_card.capabilities.into(),
            security_schemes: agent_card.security_schemes.map(|schemes| {
                schemes
                    .into_iter()
                    .map(|(k, v)| (k, serde_json::from_str(&v).expect("must be json")))
                    .collect()
            }),
            security: agent_card.security.map(|security| {
                security
                    .into_iter()
                    .map(|map| map.into_iter().map(|(k, v)| (k, v)).collect())
                    .collect()
            }),
            default_input_modes: agent_card.default_input_modes,
            default_output_modes: agent_card.default_output_modes,
            skills: agent_card.skills.into_iter().map(|s| s.into()).collect(),
            supports_authenticated_extended_card: agent_card.supports_authenticated_extended_card,
        }
    }
}

impl From<a2a_rs::AgentInfo<AccountId>> for AgentInfo {
    fn from(agent_info: a2a_rs::AgentInfo<AccountId>) -> Self {
        let a2a_rs::AgentInfo {
            agent_id,
            owner_id,
            url_verified,
            price_rate,
            agent_card,
        } = agent_info;
        Self {
            agent_id,
            owner_id,
            url_verified,
            price_rate,
            agent_card: agent_card.into(),
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
