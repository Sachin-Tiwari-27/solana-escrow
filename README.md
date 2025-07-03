# 🧾 Solana Escrow Program

A Solana smart contract built with Anchor that models a basic escrow agreement between an initializer and a receiver. The contract tracks metadata such as involved parties, amount, and status, laying the groundwork for secure, programmable fund transfers.

## 📦 Features

- Escrow account creation with PDA
- State transitions: `Pending → Completed / Cancelled`
- Ownership validation for actions
- Modular structure for easy upgrade

> 💡 **Note:** This version handles only escrow metadata (participants, amount, and status). Token transactions (transfer, lock, release) will be added in the next version using Solana's SPL Token Program.

## 📁 Project Structure

```bash
solana-escrow/
├── Anchor.toml
├── Cargo.toml
├── programs/
│ └── solana-escrow/
│ ├── src/
│ │ ├── lib.rs
│ │ ├── state.rs
│ │ ├── error.rs
│ │ └── instructions/
│ │ ├── initialize_escrow.rs
│ │ ├── cancel.rs
│ │ ├── complete.rs
│ │ └── mod.rs
├── tests/
│ └── escrow_test.ts
```