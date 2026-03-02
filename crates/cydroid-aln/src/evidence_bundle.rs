//! Evidence Bundle Definitions
//!
//! Aggregates verifiable artifacts (10-hex IDs) linked to ROW entries.
//! Supports CyberRank validation and down-ranking of invalid bundles.

use crate::error::{CydroidError, Result};
use crate::validate::{Validatable, helpers};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Type of evidence contained in the bundle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EvidenceType {
    BiophysicalSafety,
    EcoImpact,
    SwarmConsensus,
    ConsentAudit,
    NegativeResult,
    CalibrationRecord,
}

/// Evidence Bundle aggregating hex IDs and ROW references
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceBundle {
    /// Unique bundle ID
    pub bundle_id: Uuid,
    /// Type of evidence
    pub evidence_type: EvidenceType,
    /// 10-character hex identifiers for artifacts
    pub hex_ids: Vec<String>,
    /// Timestamp of bundle creation
    pub timestamp: DateTime<Utc>,
    /// ROW entry IDs linked to this evidence
    pub row_references: Vec<Uuid>,
    /// CyberRank score assigned to this bundle (0.0-1.0)
    pub cyberrank_score: f32,
    /// Validator DID who attested this bundle
    pub validator_did: String,
}

impl EvidenceBundle {
    /// Create a new evidence bundle
    pub fn new(
        evidence_type: EvidenceType,
        hex_ids: Vec<String>,
        row_references: Vec<Uuid>,
        validator_did: String,
    ) -> Result<Self> {
        let bundle = Self {
            bundle_id: Uuid::new_v4(),
            evidence_type,
            hex_ids,
            timestamp: Utc::now(),
            row_references,
            cyberrank_score: 1.0, // Default full trust
            validator_did,
        };
        bundle.validate()?;
        Ok(bundle)
    }

    /// Add a hex ID to the bundle
    pub fn add_hex_id(&mut self, hex_id: String) -> Result<()> {
        helpers::validate_hex_10(&hex_id)?;
        if !self.hex_ids.contains(&hex_id) {
            self.hex_ids.push(hex_id);
        }
        Ok(())
    }

    /// Down-rank bundle due to validation failure (CyberRank logic)
    pub fn downrank(&mut self, penalty: f32) {
        self.cyberrank_score = (self.cyberrank_score - penalty).max(0.0);
    }

    /// Check if bundle is trusted enough for governance (threshold 0.8)
    pub fn is_trusted(&self) -> bool {
        self.cyberrank_score >= 0.8
    }
}

impl Validatable for EvidenceBundle {
    fn validate(&self) -> Result<()> {
        if self.hex_ids.is_empty() {
            return Err(CydroidError::ValidationError(
                "Evidence bundle must contain at least one hex ID".to_string(),
            ));
        }

        for hex_id in &self.hex_ids {
            helpers::validate_hex_10(hex_id)?;
        }

        if self.validator_did.is_empty() || !self.validator_did.starts_with("did:") {
            return Err(CydroidError::ValidationError(
                "Validator DID must be valid".to_string(),
            ));
        }

        if self.cyberrank_score < 0.0 || self.cyberrank_score > 1.0 {
            return Err(CydroidError::ValidationError(
                "CyberRank score must be in [0.0, 1.0]".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bundle_creation() {
        let bundle = EvidenceBundle::new(
            EvidenceType::EcoImpact,
            vec!["a1f3c9b2d8".to_string()],
            vec![Uuid::new_v4()],
            "did:ion:validator".to_string(),
        );
        assert!(bundle.is_ok());
        assert!(bundle.unwrap().is_trusted());
    }

    #[test]
    fn test_downrank() {
        let mut bundle = EvidenceBundle::new(
            EvidenceType::BiophysicalSafety,
            vec!["1234567890".to_string()],
            vec![],
            "did:ion:validator".to_string(),
        )
        .unwrap();
        bundle.downrank(0.5);
        assert!(!bundle.is_trusted());
    }
}
