# Tech stack used 
---

## Programming Language: Rust

Rust is a memory-safe, high-performance language.

- Ideal for writing smart contracts due to protection against common bugs such as null pointers and data races  
- Idiomatic Rust patterns improve readability, maintainability, and security  
- Rust’s `Result` type enables explicit and safe error handling  

---

CosmWasm is a WASM-based smart contract framework for Cosmos SDK chains.

- Enables WebAssembly (WASM) execution on-chain  
- Provides high-level abstractions for:
  - Storage (`Item`, `Map`)
  - Entry points (`instantiate`, `execute`, `query`)
  - Serialization (JSON / Binary)
- Well-suited for permissioned state management patterns  

---

## Storage Library: cw-storage-plus

- Provides `Item` and `Map` for structured state management  
- `Item` stores single values (e.g., owner configuration)  
- `Map` stores key-value pairs (e.g., approved writers, messages)  
- Automatically handles serialization and deserialization  

---

## Serialization: serde + cosmwasm-schema

- `serde` is used to serialize Rust structs into JSON and Binary formats  
- `cosmwasm-schema` generates type-safe schemas for:
  - Instantiate messages  
  - Execute messages  
  - Query messages  
- Ensures consistent communication between front-end (Web3) applications and the contract  

---

## Error Handling: thiserror + cosmwasm_std::StdError

- Custom errors are defined using `thiserror::Error`  
- Standard CosmWasm errors (`StdError`) are wrapped into `ContractError`  
- Improves debugging, clarity, and auditability  

---

## Blockchain & Web3 Integration

- Deployable on Cosmos-based blockchains that support CosmWasm  
- Compatible with Web3 tools for:
  - Adding and removing writers  
  - Registering messages  
  - Reading messages and approved writers  

Demonstrates real-world Web3 patterns:

- Role-based access control  
- Immutable user-generated data  

---

## Testing & Local Development

- Local testing is performed using Rust unit tests  
- Contracts can be compiled to WASM and deployed on local or testnet chains  

### Cargo Commands

```bash
cargo build   # build the contract
cargo check   # compile-time checks
cargo test    # run unit tests
```

## Why This Stack?

- Rust → Safety and readability
- CosmWasm → WASM execution on Cosmos chains
- cw-storage-plus → Simple, structured storage
- serde + cosmwasm-schema → Easy serialization for queries
- thiserror → Clear and explicit error handling

This stack ensures the contract is secure, readable, and beginner-friendly, while clearly demonstrating core blockchain and Web3 concepts.





