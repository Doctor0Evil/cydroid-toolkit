//! Mission Execution Logic
//!
//! Validates and executes mission manifests against consent, eco-floor, and safety envelopes.

use crate::error::{CydroidError, Result};
use crate::swarm_policy::{MissionManifest, SwarmPolicy, ConsciousnessState};
use crate::consent::ConsentRegistry;
use crate::row_ledger::{RowLedger, RowEntry, RowPayload};
use crate::validate::Validatable;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Mission execution state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissionState {
    Pending,
    Running,
    Completed,
    Failed,
    RolledBack,
}

/// Mission executor context
pub struct MissionExecutor {
    pub manifest: MissionManifest,
    pub policy: SwarmPolicy,
    pub state: MissionState,
    pub consent_registry: ConsentRegistry,
    pub ledger: RowLedger,
}

impl MissionExecutor {
    /// Create a new executor
    pub fn new(
        manifest: MissionManifest,
        policy: SwarmPolicy,
        consent_registry: ConsentRegistry,
        ledger: RowLedger,
    ) -> Self {
        Self {
            manifest,
            policy,
            state: MissionState::Pending,
            consent_registry,
            ledger,
        }
    }

    /// Validate mission pre-conditions (consent, eco-floor, safety)
    pub fn validate_preconditions(&self, operator_did: &str, current_ecoscore: f32) -> Result<()> {
        // Check consent
        if !self.consent_registry.has_active_consent_for_scope(operator_did, "mission_data") {
            return Err(CydroidError::ConsentError(
                "Operator consent missing or expired".to_string(),
            ));
        }

        // Check eco-floor
        if !self.policy.check_eco_floor(current_ecoscore)? {
            return Err(CydroidError::EcoFloorNotMet {
                actual: current_ecoscore,
                required: self.policy.eco_floor,
            });
        }

        // Check mission time window
        let now = Utc::now();
        if now < self.manifest.start_time || now > self.manifest.end_time {
            return Err(CydroidError::MissionError(
                "Mission outside valid time window".to_string(),
            ));
        }

        Ok(())
    }

    /// Start mission execution
    pub fn start(&mut self, operator_did: &str, current_ecoscore: f32) -> Result<()> {
        self.validate_preconditions(operator_did, current_ecoscore)?;
        self.state = MissionState::Running;

        // Log to ROW
        let payload = RowPayload::SwarmPolicyExecution {
            policy_id: self.policy.policy_id.clone(),
            mission_id: self.manifest.mission_id.to_string(),
            decision: "Started".to_string(),
        };
        let entry = RowEntry::genesis(operator_did.to_string(), payload)?;
        self.ledger.append(entry)?;

        Ok(())
    }

    /// Complete mission
    pub fn complete(&mut self, operator_did: &str, final_ecoscore: f32) -> Result<()> {
        if self.state != MissionState::Running {
            return Err(CydroidError::MissionError(
                "Mission not running".to_string(),
            ));
        }

        if !self.policy.check_eco_floor(final_ecoscore)? {
            self.state = MissionState::Failed;
            return Err(CydroidError::EcoFloorNotMet {
                actual: final_ecoscore,
                required: self.policy.eco_floor,
            });
        }

        self.state = MissionState::Completed;

        // Log to ROW
        let payload = RowPayload::EcoMetricObservation {
            metric_id: "mission_ecoscore".to_string(),
            value: final_ecoscore as f64,
            unit: "normalized".to_string(),
        };
        let prev = self.ledger.tip().cloned().unwrap();
        let entry = RowEntry::new(&prev, operator_did.to_string(), payload, "sig".to_string())?;
        self.ledger.append(entry)?;

        Ok(())
    }
}
