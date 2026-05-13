# 📐 Architecture Overview

Soroban Sentry follows a **Decoupled Infrastructure Pattern**. This ensures that the high-cost operations (indexing, auditing, reporting) happen off-chain while the critical enforcement logic stays on-chain.

## 🔄 The Sentry Loop

```mermaid
sequenceDiagram
    participant User as User/Address
    participant Protocol as DeFi Protocol
    participant Contract as Sentry Contract
    participant Indexer as Sentry Indexer
    participant DB as Audit Database
    participant Admin as Compliance Dashboard

    User->>Protocol: Request Transaction
    Protocol->>Contract: check_compliance(User)
    Contract->>Contract: Run Registered Guards
    alt Compliance Passed
        Contract-->>Protocol: OK
        Protocol->>User: Execute TX
    else Compliance Failed
        Contract-->>Protocol: REJECT
        Contract->>Contract: Emit GuardTriggered Event
    end
    
    Indexer->>Contract: Poll for Events (XDR)
    Indexer->>Indexer: Decode XDR via sentry-common
    Indexer->>DB: Store AuditRecord
    Admin->>DB: Query Audit History
    Admin->>User: Flag/Action (Off-chain)
```

## 📦 Workspace Layout

### `contracts/sentry-contract` (The Gatekeeper)
- **State**: Storage of Guard configurations.
- **Events**: `GuardTriggered`, `IdentityVerified`, `AnomalyFlagged`.

### `packages/sentry-indexer` (The Observer)
- **Ingestion**: `tokio` based loop polling the `getEvents` RPC.
- **Transformation**: Maps raw XDR bytes to human-readable structs.

### `packages/sentry-common` (The Blueprint)
- **Shared Enums**: Standardizes what an "Anomaly" or "Reason" looks like across the entire stack.

### `packages/sentry-cli` (The Lens)
- **Querying**: Direct interface to the Sentry Indexer's database.

---
*Built for Scale. Built for Trust.*
