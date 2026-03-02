//! Final Integration Test Suite: Soul-Safe Verification
//!
//! This test suite verifies that Cydroid operations (ROW, Missions, Eco-Scores)
//! do not violate Cybercore-Brain Soul Guardrails. It ensures that no soul
//! quantification occurs, NEU budgets are respected, and rollback pathways exist.
//!
//! Aligns with: soul.guardrail.spec.v1, bio.safety.envelope.citizen.v1

#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]

use cydroid_aln::{
    RowLedger, RowEntry, RowPayload, MissionManifest, SwarmPolicy, 
    SafetyConstraints, BiophysicalTriggers, IntentClass, BiomeTag, HazardTag
};
use cybercore_soul_guardrails::{SoulGuardrailAuditor, NeuBudget, SoulState};
use chrono::Utc;
use uuid::Uuid;

/// Test: Verify ROW Ledger does not contain Soul-Scoring Fields
#[test]
fn test_row_ledger_soul_non_quantification() {
    let mut ledger = RowLedger::new();
    
    // Create a standard Cydroid mission entry
    let payload = RowPayload::SwarmPolicyExecution {
        policy_id: "eco_restore_v1".to_string(),
        mission_id: "mission_001".to_string(),
        decision: "Approved".to_string(),
    };
    
    let entry = RowEntry::genesis("did:ion:operator".to_string(), payload).unwrap();
    ledger.append(entry).unwrap();
    
    // Audit for soul-scoring fields (should be empty)
    let auditor = SoulGuardrailAuditor::new();
    let audit_result = auditor.audit_row_entry(ledger.tip().unwrap()).unwrap();
    
    assert!(audit_result.passed, "ROW entry contained forbidden soul-scoring fields");
    assert!(audit_result.soul_score_found.is_none(), "Soul score detected in ROW");
}

/// Test: Verify NEU Budget is Checked Before Mission Deployment
#[test]
fn test_neu_budget_gates_mission_deployment() {
    let mut neu_budget = NeuBudget::new(100.0); // Start with 100 NEU
    let mission_cost = 20.0;
    
    // Simulate mission deployment cost
    let allowed = neu_budget.spend(mission_cost);
    assert!(allowed, "Mission should be allowed with sufficient NEU");
    
    // Exhaust budget
    for _ in 0..5 {
        neu_budget.spend(20.0);
    }
    
    // Attempt another mission (should fail)
    let allowed = neu_budget.spend(20.0);
    assert!(!allowed, "Mission should be denied with exhausted NEU");
    
    // Verify state transition (Active -> Quarantined)
    assert_eq!(neu_budget.state(), SoulState::Quarantined);
}

/// Test: Verify Rollback Pathway Exists for Eco-Missions
#[test]
fn test_mission_rollback_pathway_exists() {
    let manifest = MissionManifest::new(
        "corridor_12".to_string(),
        "policy_v1".to_string(),
        vec!["operator".to_string()],
        Utc::now(),
        Utc::now(),
        BiomeTag::UrbanCanal,
        HazardTag::None,
        vec!["turbidity".to_string()],
    ).unwrap();
    
    // Check if manifest includes rollback particle reference
    assert!(manifest.rollback_particle.is_some(), "Mission must define rollback particle");
    assert_eq!(manifest.rollback_particle.unwrap(), "audit.pqc.rollback.v1");
}

/// Test: Verify CyberRank Down-Ranking on Soul Violation
#[test]
fn test_cyberrank_downranks_soul_violations() {
    let mut policy = SwarmPolicy::new(
        "policy_violation".to_string(),
        IntentClass::CanalClean,
        BiomeTag::UrbanCanal,
        HazardTag::None,
        SafetyConstraints::default(),
        0.86,
        BiophysicalTriggers::default(),
    ).unwrap();
    
    // Simulate a soul violation (e.g., attempting to score a participant)
    policy.karma_score = -100.0; // High negative karma
    
    // CyberRank should down-rank this policy
    let rank = policy.compute_cyberrank();
    assert!(rank < 0.5, "Policy with soul violation should have low CyberRank");
}
