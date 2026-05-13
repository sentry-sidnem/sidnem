# 🛡️ Soroban Sentry

**Soroban Sentry** is an institutional-grade infrastructure layer for the Stellar network, designed to provide **Programmable Compliance**, **Security Monitoring**, and **Audit Trails** for Soroban smart contracts.

## 🌟 The Vision
As the Stellar ecosystem moves toward regulated assets and institutional DeFi, there is a critical need for standardized compliance middleware. Soroban Sentry addresses this by providing a "Hook" based architecture where any contract can enforce transfer rules, KYC requirements, and volume limits without rebuilding the logic from scratch.

## 🏗️ Core Components

### 1. Sentry Contract (`contracts/sentry-contract`)
A modular on-chain hub that acts as a gatekeeper.
- **Hook Registry**: Allows other contracts to register compliance "Guards."
- **Event Emission**: Emits rich, XDR-encoded events specifically designed for regulator-friendly audit trails.
- **Roles**: Built-in support for Compliance Officers, Auditors, and Asset Issuers.

### 2. Sentry Indexer (`packages/sentry-indexer`)
A high-performance Rust service built on `tokio` and `sqlx`.
- **Event Streaming**: Polls Soroban RPC for Sentry-specific events.
- **XDR Decoding**: Decodes binary data into human-readable compliance logs.
- **Immutability**: Ensures every compliance check is logged in a persistent PostgreSQL audit trail.

### 3. Sentry SDK (`packages/sentry-common`)
A shared Rust library that ensures the contract and the backend speak the same language. It prevents "Schema Drift" and makes it easy for third-party developers to integrate Sentry into their own apps.

### 4. Sentry CLI (`packages/sentry-cli`)
A terminal-based tool for:
- **Auditing**: Querying the indexer for specific contract compliance history.
- **Monitoring**: Real-time tracking of "Guard Triggers" (e.g., failed KYC attempts).
- **Setup**: Deploying and configuring Sentry Guard modules.

## 🛠️ Technical Stack
- **Languages**: Rust (Soroban SDK, Tokio, SQLX).
- **Infrastructure**: PostgreSQL, Stellar RPC.
- **Architecture**: Workspace-based Monorepo.

---
*Built for the Stellar Wave Program.*
