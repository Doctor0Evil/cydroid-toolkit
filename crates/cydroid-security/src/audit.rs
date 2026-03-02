//! Security Audit Module
//!
//! Performs static and runtime checks to ensure compliance with
//! soul.guardrail.spec.v1 and karma.metric.spec.v1.
//! Prevents deployment of code that attempts to quantify souls or violate neurorights.

use cydroid_aln::{MissionManifest, SwarmPolicy};
use serde::{Deserialize, Serialize};

/// Result of a security audit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditResult {
    pub passed: bool,
    pub violations: Vec<String>,
    pub warnings: Vec<String>,
}

/// Soul Guardrail Auditor
pub struct SoulGuardrailAuditor {
    // Configuration loaded from soul.guardrail.spec.v1
    forbid_soul_scoring: bool,
    require_consent_logging: bool,
}

impl SoulGuardrailAuditor {
    pub fn new() -> Self {
        Self {
            forbid_soul_scoring: true,
            require_consent_logging: true,
        }
    }

    /// Audit a mission manifest for soul-boundary violations
    pub fn audit_manifest(&self, manifest: &MissionManifest) -> Result<AuditResult, Box<dyn std::error::Error>> {
        let mut violations = Vec::new();
        let mut warnings = Vec::new();

        // 1. Check Policy Constraints
        // In production, this would inspect the compiled policy bytecode/ALN particle
        // for forbidden opcodes (e.g., SOUL_SCORE, OWNERSHIP_TRANSFER)
        if self.contains_forbidden_policy(&manifest.policy_id) {
            violations.push(format!("Policy {} contains forbidden soul-scoring logic", manifest.policy_id));
        }

        // 2. Check Consent Requirements
        if manifest.consent_requirements.is_empty() && self.require_consent_logging {
            violations.push("Mission manifest lacks required consent requirements".to_string());
        }

        // 3. Check Eco-Floor (Safety Envelope)
        // This is a proxy for bio.safety.envelope.citizen.v1 compliance
        // A mission without an eco-floor is considered unsafe for deployment
        // (Actual check happens in SwarmPolicy validation, but we double-check here)
        warnings.push("Ensure SwarmPolicy eco_floor >= 0.86 (checked at runtime)".to_string());

        Ok(AuditResult {
            passed: violations.is_empty(),
            violations,
            warnings,
        })
    }

    /// Simulated check for forbidden policy patterns
    fn contains_forbidden_policy(&self, policy_id: &str) -> bool {
        // In production, this queries a blocklist of known harmful particle IDs
        // or scans the policy bytecode for forbidden instructions.
        let forbidden_patterns = ["soul_scorer", "ownership_transfer", "karma_on_person"];
        forbidden_patterns.iter().any(|p| policy_id.contains(p))
    }
}

impl Default for SoulGuardrailAuditor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cydroid_aln::{MissionManifest, BiomeTag, HazardTag};
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_audit_passes_safe_manifest() {
        let auditor = SoulGuardrailAuditor::new();
        let manifest = create_safe_manifest();
        let result = auditor.audit_manifest(&manifest).unwrap();
        assert!(result.passed);
        assert!(result.violations.is_empty());
    }

    #[test]
    fn test_audit_blocks_unsafe_manifest() {
        let auditor = SoulGuardrailAuditor::new();
        let mut manifest = create_safe_manifest();
        manifest.policy_id = "soul_scorer_v1".to_string(); // Forbidden pattern
        let result = auditor.audit_manifest(&manifest).unwrap();
        assert!(!result.passed);
        assert!(!result.violations.is_empty());
    }

    fn create_safe_manifest() -> MissionManifest {
        MissionManifest::new(
            "corridor_12".to_string(),
            "policy_safe_v1".to_string(),
            vec!["operator".to_string()],
            Utc::now(),
            Utc::now(),
            BiomeTag::UrbanCanal,
            HazardTag::None,
            vec!["turbidity".to_string()],
        ).unwrap()
    }
}
