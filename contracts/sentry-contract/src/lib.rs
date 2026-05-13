#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec,
};
use sentry_common::SentryEvent;

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Admin,
    Guard([u8; 32]),     // guard_id -> metadata
    AllowList(Address),  // subject -> status
    AuditLog(u64),       // timestamp/index -> summary
}

#[contract]
pub struct SentryContract;

#[contractimpl]
impl SentryContract {
    /// Initialize the contract with an admin
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Register or update a compliance guard
    pub fn set_guard(env: Env, admin: Address, guard_id: [u8; 32], metadata: String) {
        admin.require_auth();
        let current_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if admin != current_admin {
            panic!("unauthorized");
        }

        env.storage().persistent().set(&DataKey::Guard(guard_id), &metadata);
        
        env.events().publish(
            (symbol_short!("Sentry"), symbol_short!("Config")),
            symbol_short!("GuardSet"),
        );
    }

    /// Check compliance and emit events if guards are triggered
    pub fn check_compliance(env: Env, subject: Address, action: Symbol) -> bool {
        // In a full implementation, this would iterate through registered guards.
        // For this MVP, we check the internal AllowList.
        
        let is_verified = env.storage().persistent().has(&DataKey::AllowList(subject.clone()));
        
        if !is_verified {
            env.events().publish(
                (symbol_short!("Sentry"), symbol_short!("Guard")),
                SentryEvent::GuardTriggered {
                    guard_id: [0u8; 32], // Default guard
                    subject: subject.to_array(),
                    action,
                    reason: symbol_short!("NoIdent"),
                },
            );
            return false;
        }
        true
    }

    /// Verify an identity (KYC/AML)
    pub fn verify_identity(env: Env, admin: Address, subject: Address, jurisdiction: Symbol, duration: u64) {
        admin.require_auth();
        let current_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if admin != current_admin {
            panic!("unauthorized");
        }

        let expires_at = env.ledger().timestamp() + duration;
        env.storage().persistent().set(&DataKey::AllowList(subject.clone()), &expires_at);

        env.events().publish(
            (symbol_short!("Sentry"), symbol_short!("Identity")),
            SentryEvent::IdentityVerified {
                subject: subject.to_array(),
                jurisdiction,
                expires_at,
            },
        );
    }

    /// Flag an anomaly (Security monitoring)
    pub fn flag_anomaly(env: Env, reporter: Address, subject: Address, anomaly_type: Symbol, severity: u32) {
        reporter.require_auth();
        // Here we could add logic to only allow authorized reporters
        
        env.events().publish(
            (symbol_short!("Sentry"), symbol_short!("Anomaly")),
            SentryEvent::AnomalyFlagged {
                subject: subject.to_array(),
                severity,
                anomaly_type,
            },
        );
    }

    /// Read the admin address
    pub fn get_admin(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_initialization() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, SentryContract);
        let client = SentryContractClient::new(&env, &contract_id);

        client.init(&admin);
        assert_eq!(client.get_admin(), admin);
    }
}
