use cosmwasm_schema::QueryResponses; // Helps generate schema for queries
use schemars::JsonSchema;             // Enables JSON schema generation
use serde::{Deserialize, Serialize};  // Enables JSON (de)serialization for messages

/// -------------------------
/// Instantiate Message
/// -------------------------
/// Message sent when the contract is first deployed.
/// Optionally allows setting a custom owner.
/// If `owner` is None, the deployer will be the owner.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: Option<String>, // Optional owner address
}

/// -------------------------
/// Execute Messages
/// -------------------------
/// Messages used to perform **state-changing actions** on the contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    /// Add a writer to the approved list (owner-only action)
    AddWriter { writer: String },

    /// Remove a writer from the approved list (owner-only action)
    RemoveWriter { writer: String },

    /// Register a message (writer-only action)
    /// Each writer can register **one message only**.
    RegisterMessage { content: String },
}

/// -------------------------
/// Query Messages
/// -------------------------
/// Messages used to perform **read-only actions** on the contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, QueryResponses)]
pub enum QueryMsg {
    /// Fetch the message of a specific writer
    /// Returns `Option<Message>` — None if no message exists
    #[returns(Option<crate::state::Message>)]
    GetMessage { writer: String },

    /// Fetch the list of all approved writers
    /// Returns a vector of writer addresses as strings
    #[returns(Vec<String>)]
    GetWriters {},
}
