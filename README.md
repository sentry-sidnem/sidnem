# 🛡️ Soroban Sentry Protocol

**Programmable Compliance, Observability, and Audit Infrastructure for the Stellar Network.**

[![Stellar Wave](https://img.shields.io/badge/Stellar-Wave-blue.svg)](https://www.drips.network/wave/stellar)
[![License](https://img.shields.io/badge/License-Apache%202.0-green.svg)](https://opensource.org/licenses/Apache-2.0)
[![Built with Soroban](https://img.shields.io/badge/Built%20with-Soroban-purple.svg)](https://soroban.stellar.org/)

## 🌟 The Vision
As the Stellar ecosystem moves toward the mass adoption of **Real World Assets (RWA)** and **Institutional DeFi**, the industry faces a critical challenge: **How do we enforce complex regulatory compliance on-chain without compromising speed or decentralization?**

**Soroban Sentry** is the answer. It is a unified infrastructure layer that provides "Programmable Compliance" for any asset issued on the Stellar network. By decoupling compliance logic from the core asset contract, Sentry allows issuers to implement jurisdictional-specific rules, automated audit trails, and real-time security monitoring via a modular "Guard" architecture.

---

## 🏗️ Technical Architecture
Soroban Sentry is built as a high-performance Rust workspace, ensuring maximum type safety and execution speed across the entire compliance lifecycle.

### 1. The On-Chain Hub (`contracts/sentry-contract`)
The **Sentry Hub** acts as the gatekeeper for all compliance actions.
- **Modular Guards**: Register compliance "hooks" (e.g., Daily Volume Limits, KYC verification, Country-based restrictions).
- **Rich Event Schema**: Emits detailed, XDR-encoded events tailored for regulatory auditing.
- **Admin & Roles**: Built-in multi-sig and administrative roles for Compliance Officers.

### 2. The Observability Engine (`packages/sentry-indexer`)
A specialized Rust service designed for **Audit Persistence**.
- **XDR Parity**: Uses shared Rust types to decode on-chain events with 100% accuracy.
- **SQL Audit Trail**: Persists ledger events into an immutable, regulator-friendly PostgreSQL database.
- **Anomaly Detection**: Monitors event frequency and volume to flag suspicious on-chain activity in real-time.

### 3. The Shared Core (`packages/sentry-common`)
The source of truth for the entire protocol. It defines the schemas for events, identities, and audit records, ensuring that the contract, indexer, and CLI are always in perfect sync.

### 4. The Lens CLI (`packages/sentry-cli`)
A powerful terminal interface for developers and auditors to interact with the Sentry ecosystem.
- `sentry audit`: Query compliance history for any address.
- `sentry verify`: Manual identity verification for edge cases.
- `sentry status`: Monitor indexer and network health.

---

## 🔮 Future Use Cases & Potential
Soroban Sentry is designed to be the foundational layer for institutional trust on Stellar.

### 🌍 Real World Assets (RWA)
Issuers of Tokenized Bonds, Real Estate, or Equities can use Sentry to enforce secondary market compliance automatically. A token transfer only succeeds if the Sentry Hub verifies the buyer's KYC status and jurisdictional eligibility.

### 🏦 Institutional DeFi
Protocols can integrate the Sentry SDK to create "Permissioned Pools." This ensures that all liquidity providers are verified, meeting AML requirements while maintaining the benefits of decentralized finance.

### 📜 Automated Auditing
Regulators and auditors can run their own **Sentry Indexer Nodes**, receiving a real-time, cryptographically verified stream of all compliance-relevant transactions. This eliminates the need for manual, reactive audits.

### 🛡️ Smart Contract Security
By flagging anomalies (e.g., a sudden spike in high-value transfers), Sentry can trigger "Circuit Breakers" that automatically pause contracts, protecting user funds from potential exploits.

---

## 🌊 Stellar Wave: Fix. Merge. Earn.
This project is an official participant in the **Stellar Wave Program**. We are dedicated to building a sustainable, community-driven ecosystem.

**We have a backlog of 100+ tasks for contributors!** Whether you are a Rust expert, a Frontend developer, or a Documentation specialist, there is a place for you in the Sentry mission.

👉 **[View the Tasks Backlog](./TASKS_BACKLOG.md)**

### How to Contribute
1. Check the [Issues](https://github.com/sentry-sidnem/sidnem/issues) for `wave-ready` tasks.
2. Claim an issue by commenting.
3. Submit your PR and earn **Wave Points**.

---

## 🛠️ Getting Started
### Prerequisites
- [Rust & Cargo](https://rustup.rs/)
- [Stellar CLI](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup)
- [PostgreSQL](https://www.postgresql.org/)

### Quick Build
```bash
# Clone the repository
git clone https://github.com/sentry-sidnem/sidnem.git

# Build the entire Rust workspace
cargo build

# Run the Sentry Indexer
cd packages/sentry-indexer
cargo run
```

---

## ⚖️ License
Licensed under the Apache License, Version 2.0.

---
*Built to bring Institutional Trust to the Stellar Network.*
