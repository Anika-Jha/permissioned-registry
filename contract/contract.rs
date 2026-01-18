use cosmwasm_std::{
    entry_point,         // Marks functions as callable from the blockchain
    to_json_binary,      // Converts Rust data to binary JSON for queries
    Addr, Binary, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult, // Standard types for responses and query results
};

use crate::error::ContractError; // Custom errors for our contract
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg}; // Messages for executing and querying
use crate::state::{Config, Message, CONFIG, MESSAGES, WRITERS}; // Contract state

/// -------------------------
/// Instantiate: called when contract is first deployed
/// -------------------------
#[entry_point]
pub fn instantiate(
    deps: DepsMut,      // Mutable reference to blockchain storage
    _env: Env,          // Environment info: block height, time, etc.
    info: MessageInfo,  // Information about who is sending this transaction
    msg: InstantiateMsg // Parameters for contract initialization
) -> Result<Response, ContractError> {

    // Determine contract owner:
    // 1. If the deployer specified an owner, validate that address
    // 2. Otherwise, use the sender (deployer) as owner
    let owner = match msg.owner {
        Some(owner) => deps.api.addr_validate(&owner)?,
        None => info.sender,
    };

    // Create a Config struct to store the owner
    let config = Config { owner };
    // Save the config to blockchain storage
    CONFIG.save(deps.storage, &config)?;

    // Return a simple response with an attribute for tracking
    Ok(Response::new().add_attribute("action", "instantiate"))
}

/// -------------------------
/// Execute: handles all state-changing messages
/// -------------------------
#[entry_point]
pub fn execute(
    deps: DepsMut,       // Mutable access to blockchain storage
    _env: Env,           // Environment info
    info: MessageInfo,   // Who is sending the message
    msg: ExecuteMsg      // Type of execution requested
) -> Result<Response, ContractError> {
    // Match the type of ExecuteMsg and call the appropriate handler
    match msg {
        ExecuteMsg::AddWriter { writer } => execute_add_writer(deps, info, writer),
        ExecuteMsg::RemoveWriter { writer } => execute_remove_writer(deps, info, writer),
        ExecuteMsg::RegisterMessage { content } => {
            execute_register_message(deps, info, content)
        }
    }
}

/// -------------------------
/// Query: handles read-only requests
/// -------------------------
#[entry_point]
pub fn query(
    deps: Deps,       // Read-only access to storage
    _env: Env,
    msg: QueryMsg     // Type of query requested
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMessage { writer } => {
            // Return a writer's message as JSON binary
            to_json_binary(&query_message(deps, writer)?)
        }
        QueryMsg::GetWriters {} => {
            // Return all approved writers as JSON binary
            to_json_binary(&query_writers(deps)?)
        }
    }
}

/// -------------------------
/// Helper: access control
/// -------------------------
fn assert_owner(deps: Deps, sender: &Addr) -> Result<(), ContractError> {
    // Load the stored contract config
    let config = CONFIG.load(deps.storage)?;
    // Only allow the stored owner to proceed
    if config.owner != *sender {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

/// -------------------------
/// Execute Handlers
/// -------------------------

// Adds a new writer to the approved list
fn execute_add_writer(
    mut deps: DepsMut,
    info: MessageInfo,
    writer: String,
) -> Result<Response, ContractError> {
    // Ensure only owner can add a writer
    assert_owner(deps.as_ref(), &info.sender)?;

    // Validate the writer's address
    let writer_addr = deps.api.addr_validate(&writer)?;
    // Save the writer in the WRITERS map
    WRITERS.save(deps.storage, &writer_addr, &())?;

    Ok(Response::new()
        .add_attribute("action", "add_writer")
        .add_attribute("writer", writer))
}

// Removes a writer from the approved list
fn execute_remove_writer(
    mut deps: DepsMut,
    info: MessageInfo,
    writer: String,
) -> Result<Response, ContractError> {
    // Ensure only owner can remove a writer
    assert_owner(deps.as_ref(), &info.sender)?;

    // Validate the writer's address and remove them
    let writer_addr = deps.api.addr_validate(&writer)?;
    WRITERS.remove(deps.storage, &writer_addr);

    Ok(Response::new()
        .add_attribute("action", "remove_writer")
        .add_attribute("writer", writer))
}

// Registers a new message for a writer
fn execute_register_message(
    deps: DepsMut,
    info: MessageInfo,
    content: String,
) -> Result<Response, ContractError> {
    // Check that sender is an approved writer
    if !WRITERS.has(deps.storage, &info.sender) {
        return Err(ContractError::NotAWriter {});
    }

    // Check that writer hasn't already registered a message (immutability)
    if MESSAGES.has(deps.storage, &info.sender) {
        return Err(ContractError::MessageAlreadyExists {});
    }

    // Create the message struct
    let message = Message {
        author: info.sender.clone(),
        content,
    };

    // Save message to blockchain storage
    MESSAGES.save(deps.storage, &info.sender, &message)?;

    Ok(Response::new().add_attribute("action", "register_message"))
}

/// -------------------------
/// Query Handlers
/// -------------------------

// Get a message for a specific writer
fn query_message(deps: Deps, writer: String) -> StdResult<Option<Message>> {
    let writer_addr = deps.api.addr_validate(&writer)?;
    MESSAGES.may_load(deps.storage, &writer_addr)
}

// Get all approved writers
fn query_writers(deps: Deps) -> StdResult<Vec<String>> {
    let writers: Vec<String> = WRITERS
        .keys(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|addr| addr.unwrap().to_string())
        .collect();

    Ok(writers)
}
