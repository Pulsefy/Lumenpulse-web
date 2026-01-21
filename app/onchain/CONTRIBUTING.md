# Contributing to On-Chain Contracts

Welcome to the on-chain contracts workspace! This document outlines the development standards, testing conventions, and contribution workflow for Soroban smart contracts on Stellar.

## 📋 Development Standards

### Code Style
- **Rustfmt**: All code must be formatted with `cargo fmt`
- **Clippy**: No warnings allowed (`cargo clippy -- -D warnings`)
- **Naming Conventions**:
  - Structs: `PascalCase` (e.g., `HelloContract`)
  - Functions: `snake_case` (e.g., `ping`, `enable_privacy`)
  - Constants: `SCREAMING_SNAKE_CASE` (e.g., `MAX_PRIVACY_LEVEL`)
  - Variables: `snake_case` (e.g., `account_address`)

### Import Order
```rust
// 1. External crates
use soroban_sdk::{contract, contractimpl, Env};

// 2. Internal modules (if any)
// use crate::types::*;

// 3. Module declarations
mod test;