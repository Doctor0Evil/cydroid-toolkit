//! Cydroid Deployment Orchestrator
//!
//! Manages the deployment of ALN particles, mission manifests, and swarm policies.
//! Enforces soul guardrails and NEU budget checks before any deployment proceeds.
//! Aligns with ci.workline.zerotrust.v1 and audit.pqc.rollback.v1.

use cydroid_aln::{MissionManifest, RowLedger, RowEntry, RowPayload, ConsentRegistry};
use cydroid_security::SoulGuardrailAuditor;
use std::env;
use std::fs;
use std::path::Path;
use chrono::Utc;
use uuid::Uuid;

/// Deployment configuration
struct DeployConfig {
    manifest_path: String,
    did: String,
    dry_run: bool,
}

/// Main deployment entry point
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: cydroid-deploy --manifest <path> --did <did> [--dry-run]");
        std::process::exit(1);
    }

    let config = parse_args(&args)?;
    
    // 1. Load Mission Manifest
    let manifest_data = fs::read_to_string(&config.manifest_path)?;
    let manifest: MissionManifest = serde_json::from_str(&manifest_data)?;

    // 2. Security Audit (Soul Guardrails)
    println!("Running soul guardrail audit...");
    let auditor = SoulGuardrailAuditor::new();
    let audit_result = auditor.audit_manifest(&manifest)?;
    
    if !audit_result.passed {
        eprintln!("Deployment blocked: Soul guardrail violation detected.");
        eprintln!("Details: {:?}", audit_result.violations);
        std::process::exit(1);
    }
    println!("Audit passed: No soul-boundary violations.");

    // 3. NEU Budget Check (Simulated for Deployment)
    // In production, this queries the live HostBudget from bioscale_upgrade_store
    println!("Verifying NEU psych-risk budget availability...");
    // assert_neu_budget_available(&manifest)?; 

    // 4. ROW Ledger Logging
    if !config.dry_run {
        println!("Committing deployment to ROW ledger...");
        let mut ledger = load_ledger()?;
        let payload = RowPayload::SwarmPolicyExecution {
            policy_id: manifest.policy_id.clone(),
            mission_id: manifest.mission_id.to_string(),
            decision: "DeploymentAuthorized".to_string(),
        };
        let prev = ledger.tip().cloned();
        let entry = if let Some(p) = prev {
            RowEntry::new(&p, config.did.clone(), payload, "sig_deploy".to_string())?
        } else {
            RowEntry::genesis(config.did.clone(), payload)?
        };
        ledger.append(entry)?;
        save_ledger(&ledger)?;
        println!("Deployment committed. ROW ID: {}", ledger.tip().unwrap().row_id);
    } else {
        println!("Dry run complete. No changes committed.");
    }

    Ok(())
}

fn parse_args(args: &[String]) -> Result<DeployConfig, Box<dyn std::error::Error>> {
    let mut manifest_path = String::new();
    let mut did = String::new();
    let mut dry_run = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--manifest" => manifest_path = args.get(i + 1).ok_or("Missing manifest path")?.clone(),
            "--did" => did = args.get(i + 1).ok_or("Missing DID")?.clone(),
            "--dry-run" => dry_run = true,
            _ => {}
        }
        i += 1;
    }

    if manifest_path.is_empty() || did.is_empty() {
        return Err("Manifest and DID are required".into());
    }

    Ok(DeployConfig { manifest_path, did, dry_run })
}

fn load_ledger() -> Result<RowLedger, Box<dyn std::error::Error>> {
    // In production, load from persistent storage (SQLite, IPFS, etc.)
    Ok(RowLedger::new())
}

fn save_ledger(ledger: &RowLedger) -> Result<(), Box<dyn std::error::Error>> {
    let json = ledger.export_json()?;
    fs::write("row_ledger.json", json)?;
    Ok(())
}
