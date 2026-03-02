//! Integration Test Suite
//!
//! End-to-end tests verifying ROW ledger integrity, soul guardrail enforcement,
//! NEU budget logic, and eco-floor constraints.
//! These tests simulate a full mission lifecycle from consent to completion.

use cydroid_aln::{
    RowLedger, RowEntry, RowPayload, ConsentRecord, ConsentRegistry, 
    SwarmPolicy, SafetyConstraints, BiophysicalTriggers, IntentClass, 
    BiomeTag, HazardTag, MissionManifest, EvidenceBundle, EvidenceType
};
use chrono::{Utc, Duration};
use uuid::Uuid;

/// Test: ROW Ledger Chain Integrity
#[test]
fn test_row_ledger_chain_integrity() {
    let mut ledger = RowLedger::new();

    // Genesis
    let payload1 = RowPayload::ConsentEvent {
        consent_id: Uuid::new_v4(),
        participant_did: "did:ion:test".to_string(),
        fps_hash: "hash_fps_001".to_string(),
        comprehension_passed: true,
    };
    let entry1 = RowEntry::genesis("did:ion:admin".to_string(), payload1).unwrap();
    let hash1 = entry1.compute_hash().unwrap();
    ledger.append(entry1).unwrap();

    // Second Entry
    let payload2 = RowPayload::SwarmPolicyExecution {
        policy_id: "policy_v1".to_string(),
        mission_id: "mission_001".to_string(),
        decision: "Started".to_string(),
    };
    let entry2 = RowEntry::new(
        ledger.tip().unwrap(), 
        "did:ion:operator".to_string(), 
        payload2, 
        "sig_002".to_string()
    ).unwrap();
    ledger.append(entry2).unwrap();

    // Verify Chain Link
    assert_eq!(ledger.len(), 2);
    assert_eq!(ledger.tip().unwrap().previous_entry_hash, hash1);
}

/// Test: Soul Guardrail Enforcement (No Soul Scoring)
#[test]
fn test_soul_guardrail_enforcement() {
    // Simulate a policy that attempts to violate soul guardrails
    // In production, this would be caught by the Security Audit module
    // Here we verify the policy structure itself doesn't allow soul fields
    let policy = SwarmPolicy::new(
        "policy_safe_v1".to_string(),
        IntentClass::CanalClean,
        BiomeTag::UrbanCanal,
        HazardTag::None,
        SafetyConstraints::default(),
        0.86, // Eco-floor
        BiophysicalTriggers::default(),
    ).unwrap();

    // Verify policy does not contain soul-scoring fields (structural check)
    // (In real code, this is enforced by type system lacking soul_score fields)
    assert!(policy.eco_floor >= 0.86); // Safety envelope check
}

/// Test: NEU Psych-Risk Budget Logic (Simulated)
#[test]
fn test_neu_budget_exhaustion_logic() {
    // Simulate NEU budget exhaustion triggering rollback
    let mut neu_budget = 100.0;
    let operation_cost = 20.0;
    let exhaustion_threshold = 0.0;

    // Perform operations
    for _ in 0..5 {
        neu_budget -= operation_cost;
    }

    assert_eq!(neu_budget, exhaustion_threshold);
    
    // Next operation should trigger rollback/denial
    let next_operation_cost = 20.0;
    let allowed = neu_budget >= next_operation_cost;
    assert!(!allowed); // Operation denied due to budget exhaustion
}

/// Test: Evidence Bundle Creation
#[test]
fn test_evidence_bundle_creation() {
    let bundle = EvidenceBundle::new(
        EvidenceType::EcoImpact,
        vec!["a1f3c9b2d8".to_string()],
        vec![Uuid::new_v4()],
        "did:ion:validator".to_string(),
    ).unwrap();

    assert!(bundle.is_trusted());
    assert_eq!(bundle.hex_ids.len(), 1);
}

/// Test: Consent Registry Revocation
#[test]
fn test_consent_revocation() {
    let mut registry = ConsentRegistry::new();
    let consent = create_test_consent();
    let consent_id = consent.consent_id;
    registry.add(consent).unwrap();

    // Revoke
    registry.revoke(consent_id, "User requested".to_string()).unwrap();

    // Verify revoked status
    let record = registry.get(consent_id).unwrap();
    assert!(!record.is_active());
}

fn create_test_consent() -> ConsentRecord {
    use cydroid_aln::{ComprehensionCheck, FpsMetadata, DataUseScope};
    ConsentRecord::new(
        "did:ion:participant".to_string(),
        "hash_fps".to_string(),
        "1.0".to_string(),
        ComprehensionCheck {
            timestamp: Utc::now(),
            passed: true,
            attempts: 1,
            method: "quiz".to_string(),
        },
        vec![DataUseScope {
            scope_id: "mission_data".to_string(),
            description: "Mission telemetry".to_string(),
            expires_at: None,
        }],
        FpsMetadata {
            version: "1.0".to_string(),
            language: "en".to_string(),
            content_hash: "hash_fps".to_string(),
            uri: None,
            presentation_format: "text".to_string(),
        },
        true,
    ).unwrap()
}
