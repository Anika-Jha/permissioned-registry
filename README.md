## Permissioned Registry (CosmWasm Smart Contract)

## Contract Summary

This smart contract implements a **permissioned message registry**. Only approved writers can register a message on-chain, and each writer can have **at most one message** associated with them which is immutable.

Think of this like a **shared notice board**:

* The **owner** decides who is allowed to write.
*  **Writers** can post their message, but only once.
* Anyone can **read** messages and see the list of writers.

This project is built to demonstrate core smart contract fundamentals in **Rust + CosmWasm** with clear access control, explicit errors, and clean state management.

---

## Why This Design?

This contract was intentionally kept simple but realistic:

* Permissioned systems are common in real-world apps (admins, moderators, curators).
* Demonstrates ownership-based access control.
* Shows controlled state mutation (who can change what).
* Easy to reason about and audit.

The goal is **clarity over complexity**; especially for beginners learning smart contracts.

---

## State & Data Model

### Stored State

1. **Config**

```rust
Config {
  owner: Addr
}
```

* Stores the contract owner.
* Owner manages writer permissions.

2. **WRITERS**

```rust
Map<Addr, bool>
```

* Tracks which addresses are approved writers.

3. **MESSAGES**

```rust
Map<Addr, Message>
```

* Stores one message per writer address.

---

## Contract Flow (State Machine)

### 1. Instantiate

* Contract is deployed.
* Owner is set (either provided or defaults to deployer).
* No writers or messages exist yet.

### 2. Add Writer (Owner only)

* Owner authorizes a new writer address.
* Writer is added to the `WRITERS` map.

### 3. Remove Writer (Owner only)

* Owner removes a writer’s permission.
* Writer can no longer register messages.

### 4. Register Message (Writer only)

* Approved writer registers or updates their message.
* Message is stored under their address.

### 5. Query

* Anyone can:

  * Fetch a writer’s message
  * List all approved writers

---

## Security & Access Control

This contract enforces several security checks:

* **Owner-only actions**

  * Only the owner can add or remove writers.

* **Writer-only actions**

  * Only approved writers can register messages.

* **Explicit error handling**

  * Unauthorized access returns clear errors.
  * Missing data is handled safely.

* **No unchecked state mutation**

  * Every write is gated by permission checks.

---

## How to Build & Test

### Build

```bash
cargo build
```

### Run Checks

```bash
cargo check
```

### Tests (to be added)

```bash
cargo test
```

---

## Known Limitations & Improvements

* Messages are simple strings (could be structured data).
* No expiration or history tracking for messages.
* No pagination for large writer sets.
* Gas usage not optimized yet.

Possible improvements:

* Message versioning
* Role-based permissions (admin vs writer)
* Pagination in queries
* Integration tests

---

## Learning Takeaway

This contract shows how **small, well-structured Rust code** can enforce strong guarantees on-chain using:

* Explicit permissions
* Clear state ownership
* Predictable execution paths

It is designed to be easy to read, audit, and extend.

## Social & Build Week Post

I shared my learning journey and the Permissioned Registry smart contract challenge on LinkedIn:

[View my post]([https://www.linkedin.com/posts/your-link-here](https://www.linkedin.com/posts/anika-jha-2157b0228_blockchain-smartcontracts-rust-activity-7418614318503698432-H_iM?utm_source=share&utm_medium=member_desktop&rcm=ACoAADkSEDEBsl2WzpOdWcuX2SXmRDinxCrer9A)](https://www.linkedin.com/posts/anika-jha-2157b0228_blockchain-smartcontracts-rust-activity-7418614318503698432-H_iM?utm_source=share&utm_medium=member_desktop&rcm=ACoAADkSEDEBsl2WzpOdWcuX2SXmRDinxCrer9A))
