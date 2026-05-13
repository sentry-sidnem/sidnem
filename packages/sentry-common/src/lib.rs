use serde::{Deserialize, Serialize};

#[cfg(feature = "contract")]
use soroban_sdk::{contracttype, Symbol};

// Use a type alias or a simple String for non-contract environments
#[cfg(not(feature = "contract"))]
type Symbol = String;

/// Core event types for Soroban Sentry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "contract", contracttype)]
pub enum SentryEvent {
    /// A new compliance guard has been triggered
    GuardTriggered {
        guard_id: [u8; 32],
        subject: [u8; 32],
        action: Symbol,
        reason: Symbol,
    },
    /// An address has been added to the allow-list
    IdentityVerified {
        subject: [u8; 32],
        jurisdiction: Symbol,
        expires_at: u64,
    },
    /// A suspicious activity was flagged
    AnomalyFlagged {
        subject: [u8; 32],
        severity: u32,
        anomaly_type: Symbol,
    },
}

/// Represents a compliance audit record in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRecord {
    pub ledger_sequence: u32,
    pub transaction_id: String,
    pub event: SentryEvent,
    pub timestamp: i64,
}
