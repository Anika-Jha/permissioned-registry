use cosmwasm_schema::cw_serde;       // Provides serde support with CosmWasm helpers
use cosmwasm_std::Addr;              // Standard type for blockchain addresses
use cw_storage_plus::{Item, Map};    // Storage abstractions: Item = single value, Map = key-value

/// -------------------------
/// Global Contract Configuration
/// -------------------------
/// This struct stores the **owner** of the contract.
/// Set once during instantiation and **never changes**.
#[cw_serde]
pub struct Config {
    pub owner: Addr, // The account that controls adding/removing writers
}

/// -------------------------
/// Writer Message
/// -------------------------
/// Represents a message registered by an approved writer.
/// Once stored, messages are **immutable**.
#[cw_serde]
pub struct Message {
    pub author: Addr,   // The writer who created the message
    pub content: String, // The actual message content
}

/// -------------------------
/// Storage Items
/// -------------------------

/// Stores the contract's configuration (owner)
pub const CONFIG: Item<Config> = Item::new("config");

/// Stores the list of approved writers
/// Key: writer's address  
/// Value: empty tuple `()` (we only care about the key)
pub const WRITERS: Map<&Addr, ()> = Map::new("writers");

/// Stores messages registered by writers
/// Key: writer's address  
/// Value: `Message` struct
pub const MESSAGES: Map<&Addr, Message> = Map::new("messages");
