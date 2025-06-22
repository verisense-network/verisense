#![cfg_attr(not(feature = "std"), no_std)]

// A2A Rust types for no_std

use alloc::string::String;
use codec::{alloc, Decode, Encode};
use scale_info::TypeInfo;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::vec::Vec;

type Text = String;

/// Represents the service provider of an agent.
#[derive(Debug, Clone, Decode, Encode, TypeInfo, Eq, PartialEq)]
pub struct AgentProvider {
    /// Agent provider's organization name.
    pub organization: Text,
    /// Agent provider's URL.
    pub url: Text,
}

/// Defines optional capabilities supported by an agent.
#[derive(Debug, Clone, Decode, Encode, TypeInfo, Eq, PartialEq)]
pub struct AgentCapabilities {
    /// true if the agent supports SSE.
    pub streaming: Option<bool>,
    /// true if the agent can notify updates to client.
    pub push_notifications: Option<bool>,
    /// true if the agent exposes status change history for tasks.
    pub state_transition_history: Option<bool>,
    /// extentions supported by the agent.
    pub extensions: Option<Vec<AgentExtension>>,
}

#[derive(Debug, Clone, Decode, Encode, TypeInfo, Eq, PartialEq)]
pub struct AgentExtension {
    pub uri: Text,
    pub description: Option<Text>,
    pub required: Option<bool>,
    pub params: Option<Text>,
}

/// Represents a unit of capability that an agent can perform.
#[derive(Debug, Clone, Decode, Encode, TypeInfo, Eq, PartialEq)]
pub struct AgentSkill {
    /// Unique identifier for the agent's skill.
    pub id: Text,
    /// Human readable name of the skill.
    pub name: Text,
    /// Description of the skill - will be used by the client or a human
    /// as a hint to understand what the skill does.
    pub description: Text,
    /// Set of tagwords describing classes of capabilities for this specific skill.
    pub tags: Vec<Text>,
    /// The set of example scenarios that the skill can perform.
    /// Will be used by the client as a hint to understand how the skill can be used.
    pub examples: Option<Vec<Text>>,
    /// The set of interaction modes that the skill supports
    /// (if different than the default).
    /// Supported mime types for input.
    pub input_modes: Option<Vec<Text>>,
    /// Supported mime types for output.
    pub output_modes: Option<Vec<Text>>,
}

/// An AgentCard conveys key information:
/// - Overall details (version, name, description, uses)
/// - Skills: A set of capabilities the agent can perform
/// - Default modalities/content types supported by the agent.
/// - Authentication requirements
#[derive(Debug, Clone, Decode, Encode, TypeInfo, Eq, PartialEq)]
pub struct AgentCard {
    /// Human readable name of the agent.
    pub name: Text,
    /// A human-readable description of the agent. Used to assist users and
    /// other agents in understanding what the agent can do.
    pub description: Text,
    /// A URL to the address the agent is hosted at.
    pub url: Text,
    /// A URL to an icon for the agent.
    pub icon_url: Option<Text>,
    /// The service provider of the agent
    pub provider: Option<AgentProvider>,
    /// The version of the agent - format is up to the provider.
    pub version: Text,
    /// A URL to documentation for the agent.
    pub documentation_url: Option<Text>,
    /// Optional capabilities supported by the agent.
    pub capabilities: AgentCapabilities,
    /// Security scheme details used for authenticating with this agent.
    // it's difficult to store kv mappings into substrate runtime, so we use transparent Text
    pub security_schemes: Option<BTreeMap<Text, Text>>,
    /// Security requirements for contacting the agent.
    pub security: Option<Vec<BTreeMap<Text, Vec<Text>>>>,
    /// The set of interaction modes that the agent supports across all skills.
    /// Supported mime types for input.
    pub default_input_modes: Vec<Text>,
    /// Supported mime types for output.
    pub default_output_modes: Vec<Text>,
    /// Skills are a unit of capability that an agent can perform.
    pub skills: Vec<AgentSkill>,
    /// true if the agent supports providing an extended agent card when the user is authenticated.
    /// Defaults to false if not specified.
    pub supports_authenticated_extended_card: Option<bool>,
}

#[derive(Debug, Clone, Decode, Encode, TypeInfo, Eq, PartialEq)]
pub struct AgentInfo<AccountId> {
    pub agent_id: AccountId,
    pub owner_id: AccountId,
    pub agent_card: AgentCard,
}

#[derive(Debug, Clone, Decode, Encode, TypeInfo, Eq, PartialEq)]
pub enum Role {
    User,
    Agent,
}

#[derive(Debug, Clone, Decode, Encode, TypeInfo, Eq, PartialEq)]
pub struct TextPart {
    pub metadata: Vec<(Text, Text)>,
    pub text: Text,
}

#[derive(Debug, Clone, Decode, Encode, TypeInfo, Eq, PartialEq)]
pub struct DataPart {
    pub metadata: Vec<(Text, Text)>,
    pub data: Vec<(Text, Text)>,
}

#[derive(Debug, Clone, Decode, Encode, TypeInfo, Eq, PartialEq)]
pub struct FilePart {
    pub metadata: Vec<(Text, Text)>,
    pub name: Option<Text>,
    pub mime_type: Option<Text>,
    pub file_data: FileData,
}

#[derive(Debug, Clone, Decode, Encode, TypeInfo, Eq, PartialEq)]
pub enum FileData {
    FileWithBytes(Text),
    FileWithUri(Text),
}

#[derive(Debug, Clone, Decode, Encode, TypeInfo, Eq, PartialEq)]
pub enum Part {
    Text(TextPart),
    File(FilePart),
    Data(DataPart),
}

#[derive(Debug, Clone, Decode, Encode, TypeInfo, Eq, PartialEq)]
pub struct A2aMessage {
    pub role: Role,
    pub parts: Vec<Part>,
    pub metadata: Option<Vec<(Text, Text)>>,
    pub extensions: Option<Vec<Text>>,
    pub reference_task_ids: Option<Vec<Text>>,
    pub message_id: Text,
    pub task_id: Option<Text>,
    pub context_id: Option<Text>,
}

#[derive(Debug, Clone, Decode, Encode, TypeInfo, Eq, PartialEq)]
pub enum TaskState {
    Submitted,
    Working,
    InputRequired,
    Completed,
    Canceled,
    Failed,
    Rejected,
    AuthRequired,
    Unknown,
}

#[derive(Debug, Clone, Decode, Encode, TypeInfo, Eq, PartialEq)]
pub struct TaskStatus {
    pub state: TaskState,
    pub message: Option<A2aMessage>,
    pub timestamp: Option<Text>,
}

#[derive(Debug, Clone, Decode, Encode, TypeInfo, Eq, PartialEq)]
pub struct Artifact {
    pub artifact_id: Text,
    pub name: Option<Text>,
    pub description: Option<Text>,
    pub parts: Vec<Part>,
    pub metadata: Option<Vec<(Text, Text)>>,
    pub extensions: Option<Vec<Text>>,
}

#[derive(Debug, Clone, Decode, Encode, TypeInfo, Eq, PartialEq)]
pub struct A2aTask {
    pub id: Text,
    pub context_id: Text,
    pub status: TaskStatus,
    pub history: Option<Vec<A2aMessage>>,
    pub artifacts: Option<Vec<Artifact>>,
    pub metadata: Option<Vec<(Text, Text)>>,
}
