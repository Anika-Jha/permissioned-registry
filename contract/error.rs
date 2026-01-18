use cosmwasm_std::StdError; // Standard errors provided by CosmWasm
use thiserror::Error;        // Derive macro for easy custom error creation

/// -------------------------
/// Custom errors for the Permissioned Registry contract
/// -------------------------
#[derive(Error, Debug)]
pub enum ContractError {
    /// Wraps standard CosmWasm errors.
    /// This allows functions that return `StdResult` to automatically convert errors into our custom error type.
    #[error("{0}")]
    Std(#[from] StdError),

    /// Thrown when a user tries to perform an action reserved for the contract owner.
    /// Example: a non-owner trying to add or remove a writer.
    #[error("Unauthorized")]
    Unauthorized {},

    /// Thrown when someone who is not an approved writer tries to register a message.
    #[error("Address is not an approved writer")]
    NotAWriter {},

    /// Thrown when a writer tries to register a message, but they already have one.
    /// Ensures **immutability** of messages — once written, they cannot be changed.
    #[error("Message already exists for this writer")]
    MessageAlreadyExists {},
}
