# 📋 Soroban Sentry protocol

This backlog contains issues specifically designed for the project. Each task is structured to be independently implementable by contributors.

---

## 🛡️ Smart Contracts (Rust/Soroban)
1.  [ ] Implement `pause` and `unpause` functionality for the Sentry Hub.
2.  [ ] Add `Ownable` pattern to the Sentry Contract using OpenZeppelin.
3.  [ ] Create a `VolumeGuard` that tracks 24h transfer volumes per address.
4.  [ ] Implement a `VelocityGuard` to prevent high-frequency transaction bursts.
5.  [ ] Add `blacklist_address` functionality with a `Reason` code.
6.  [ ] Create a `WhitelistGuard` for specific jurisdictional compliance.
7.  [ ] Implement `upgrade_contract` logic via Authority check.
8.  [ ] Add a `MetadataStore` to store external CID links for identities.
9.  [ ] Implement a `MultiSigAdmin` for critical compliance actions.
10. [ ] Create a `TimeLock` for adding/removing high-level Guards.
11. [ ] Add `FeeAbstraction` support for compliance checks.
12. [ ] Implement a `ClawbackGuard` authorized by Sentry.
13. [ ] Create a `CircuitBreaker` that pauses a protocol on large anomaly detection.
14. [ ] Add `BatchVerify` for processing multiple identities in one TX.
15. [ ] Implement `ExpirationLogic` for transient identities.
16. [ ] Create a `TieredCompliance` system (Level 1, 2, 3).
17. [ ] Add `HookRegistry` to allow 3rd party contracts to register custom guards.
18. [ ] Implement `StorageRent` management logic for persistent identity data.
19. [ ] Create a `SentryTokenBridge` hook for cross-chain compliance.
20. [ ] Add `EventCompression` logic to reduce on-chain data costs.

## 📡 Indexer (Rust/Systems)
21. [ ] Implement `getEvents` RPC call with proper pagination.
22. [ ] Add `XDR decoding` for `GuardTriggered` events.
23. [ ] Add `XDR decoding` for `IdentityVerified` events.
24. [ ] Add `XDR decoding` for `AnomalyFlagged` events.
25. [ ] Implement `LastLedger` persistence in DB to prevent data gaps.
26. [ ] Add `PostgreSQL Schema` migrations via `sqlx-cli`.
27. [ ] Implement `Re-org Handling` logic (detecting ledger branches).
28. [ ] Add `Prometheus Exporter` for indexing latency metrics.
29. [ ] Implement `Webhook Notifications` for high-severity anomalies.
30. [ ] Add `Rate Limiting` for RPC polling to avoid node bans.
31. [ ] Implement `GraphQL API` layer over the Audit logs.
32. [ ] Add `Redis Caching` for frequently queried compliance statuses.
33. [ ] Implement `Batch Insert` in SQLX for high-throughput ledgers.
34. [ ] Add `Multi-Contract Support` (one indexer, many Sentry Hubs).
35. [ ] Implement `Log Rotation` for indexer service logs.
36. [ ] Add `Health Check` endpoint for container orchestration.
37. [ ] Implement `Transaction Re-parsing` for historically missed ledgers.
38. [ ] Add `Audit Trail Hashing` to ensure DB integrity.
39. [ ] Implement `Export to CSV` functionality for regulators.
40. [ ] Add `Discord Integration` for real-time compliance alerts.

## 💻 CLI Tool (Rust)
41. [ ] Implement `sentry status` to check indexer and node health.
42. [ ] Add `sentry audit --address [ID]` to view full compliance history.
43. [ ] Implement `sentry guards list` to view all active guards on-chain.
44. [ ] Add `sentry verify --subject [ID] --jurisdiction [CODE]` command.
45. [ ] Implement `sentry config set-contract [ID]` for local setup.
46. [ ] Add `sentry log --follow` for live event tailing.
47. [ ] Implement `sentry bench` to test contract compliance latency.
48. [ ] Add `sentry export --format [json|csv]` for audit data.
49. [ ] Implement `sentry debug xdr [HEX]` to decode raw Stellar data.
50. [ ] Add `Colorized Output` support for terminal readability.
51. [ ] Implement `Interactive Prompts` for complex configuration flows.
52. [ ] Add `sentry-cli completion` for bash/zsh/fish.
53. [ ] Implement `sentry scan` to find contracts lacking Sentry integration.
54. [ ] Add `sentry update` self-update logic via GitHub releases.
55. [ ] Implement `Mock Server` mode for frontend development.
56. [ ] Add `sentry diff` to compare compliance states between ledgers.
57. [ ] Implement `sentry identity revoke [ID]` command.
58. [ ] Add `Multiple Profiles` support (Testnet, Mainnet, Futurenet).
59. [ ] Implement `sentry help` with detailed examples for each command.
60. [ ] Add `sentry doctor` to diagnose environment/dependency issues.

## 📖 Documentation & DevRel
61. [ ] Create a `Quick Start Guide` for Asset Issuers.
62. [ ] Write a `Regulatory Mapping` doc (GDPR/MiCA to Sentry Hooks).
63. [ ] Create a `Sentry Hook Integration` tutorial for DeFi devs.
64. [ ] Implement a `Docusaurus` site for full API documentation.
65. [ ] Write a `Best Practices` guide for storage management on Soroban.
66. [ ] Create `Example Contracts` demonstrating Sentry in a Lending protocol.
67. [ ] Write a `Sentry Indexer Deployment` guide (Docker/Kubernetes).
68. [ ] Create a `Video Tutorial` series on YouTube/Loom.
69. [ ] Implement `Interactive API Explorer` for the Indexer.
70. [ ] Write a `Whitepaper` describing the Programmable Compliance vision.
71. [ ] Create a `Contribution Guide` specifically for Wave participants.
72. [ ] Translate documentation into Spanish (Stellar has a large LatAm community).
73. [ ] Translate documentation into Portuguese.
74. [ ] Create a `F.A.Q.` section for common compliance questions.
75. [ ] Write a blog post: "Why Your Stellar Asset Needs a Sentry."
76. [ ] Design a `Cheat Sheet` for Sentry CLI commands.
77. [ ] Create a `Security Policy` and vulnerability disclosure guide.
78. [ ] Build a `Live Demo` site showing real-time testnet audits.
79. [ ] Write `Success Stories` of protocols using Sentry.
80. [ ] Create `Mintlify` docs as an alternative to Docusaurus.

## 🧪 Testing & Security
81. [ ] Implement `Fuzz Testing` for the Sentry Hub contract.
82. [ ] Add `Property-Based Testing` for compliance guard logic.
83. [ ] Create a `Mock RPC Server` for indexer integration tests.
84. [ ] Implement `Gas Profiling` suite for Sentry Hooks.
85. [ ] Add `Code Coverage` reporting in GitHub Actions.
86. [ ] Conduct a `Security Audit` checklist for the Hook pattern.
87. [ ] Create `Exploit Scenarios` to test guard effectiveness.
88. [ ] Add `Linting Rules` specific to Soroban storage patterns.
89. [ ] Implement `Snapshot Testing` for DB schemas.
90. [ ] Add `Static Analysis` (Semgrep/Slither-equivalent) for Soroban.

## 🏗️ Infrastructure & Misc
91. [ ] Create a `Docker Compose` for one-click local development.
92. [ ] Implement `Terraform Scripts` for AWS/GCP deployment.
93. [ ] Add `GitHub Actions` for automated WASM releases.
94. [ ] Create an `npm package` for the Sentry Frontend SDK.
95. [ ] Implement `OpenAPI / Swagger` for the Indexer backend.
96. [ ] Add `Sentry.io Error Tracking` to the indexer service.
97. [ ] Create a `Stellar Dashboard` widget for Sentry status.
98. [ ] Implement `Multi-arch builds` (x86, ARM) for the CLI.
99. [ ] Add `Auto-generated ChangeLog` via Conventional Commits.
100. [ ] Create a `Sentry Badges` system for verified "Compliant" protocols.

---

