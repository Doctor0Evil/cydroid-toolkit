//! Consent Records and Formally Presented Statements (FPS)
//!
//! This module defines consent records bound to participant DIDs,
//! with FPS hashes, comprehension checks, and revocation semantics.
//! All consent is forward-only (no rollbacks) and logged to ROW.

use crate::error::{CydroidError, Result};
use crate::validate::Validatable;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Consent status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConsentStatus {
    Active,
    Revoked,
    Expired,
    Suspended,
}

/// Data use scope for consent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataUseScope {
    /// Scope identifier (e.g., "mission_data", "eco_metrics")
    pub scope_id: String,
    /// Description of permitted use
    pub description: String,
    /// Expiration timestamp (optional)
    pub expires_at: Option<DateTime<Utc>>,
}

/// Formally Presented Statement (FPS) metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FpsMetadata {
    /// FPS version
    pub version: String,
    /// Language code (e.g., "en", "es", "zh")
    pub language: String,
    /// SHA-256 hash of FPS content
    pub content_hash: String,
    /// URI to FPS document
    pub uri: Option<String>,
    /// Presentation format (text, audio, AR)
    pub presentation_format: String,
}

/// Comprehension check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensionCheck {
    /// Check timestamp
    pub timestamp: DateTime<Utc>,
    /// Check passed
    pub passed: bool,
    /// Number of attempts
    pub attempts: u32,
    /// Check method (e.g., "quiz", "read-back", "verbal")
    pub method: String,
}

/// Consent record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    /// Unique consent ID
    pub consent_id: Uuid,
    /// Participant DID
    pub participant_did: String,
    /// FPS hash
    pub fps_hash: String,
    /// Consent version
    pub consent_version: String,
    /// Comprehension check result
    pub comprehension_check: ComprehensionCheck,
    /// Timestamp of consent grant
    pub granted_at: DateTime<Utc>,
    /// Whether consent is revocable
    pub revocable: bool,
    /// Current consent status
    pub status: ConsentStatus,
    /// Data use scopes
    pub data_use_scopes: Vec<DataUseScope>,
    /// FPS metadata
    pub fps_metadata: FpsMetadata,
    /// Timestamp of revocation (if applicable)
    pub revoked_at: Option<DateTime<Utc>>,
    /// Revocation reason (if applicable)
    pub revocation_reason: Option<String>,
    /// ROW entry ID where this consent was logged
    pub row_entry_id: Option<Uuid>,
}

impl ConsentRecord {
    /// Create a new consent record
    pub fn new(
        participant_did: String,
        fps_hash: String,
        consent_version: String,
        comprehension_check: ComprehensionCheck,
        data_use_scopes: Vec<DataUseScope>,
        fps_metadata: FpsMetadata,
        revocable: bool,
    ) -> Result<Self> {
        let record = Self {
            consent_id: Uuid::new_v4(),
            participant_did,
            fps_hash,
            consent_version,
            comprehension_check,
            granted_at: Utc::now(),
            revocable,
            status: ConsentStatus::Active,
            data_use_scopes,
            fps_metadata,
            revoked_at: None,
            revocation_reason: None,
            row_entry_id: None,
        };
        record.validate()?;
        Ok(record)
    }

    /// Revoke consent (forward-only, cannot be undone)
    pub fn revoke(&mut self, reason: String) -> Result<()> {
        if !self.revocable {
            return Err(CydroidError::ConsentError(
                "Consent is not revocable".to_string(),
            ));
        }
        if self.status != ConsentStatus::Active {
            return Err(CydroidError::ConsentError(format!(
                "Cannot revoke consent in {:?} status",
                self.status
            )));
        }
        self.status = ConsentStatus::Revoked;
        self.revoked_at = Some(Utc::now());
        self.revocation_reason = Some(reason);
        Ok(())
    }

    /// Suspend consent (temporary, can be reactivated)
    pub fn suspend(&mut self, reason: String) -> Result<()> {
        if self.status != ConsentStatus::Active {
            return Err(CydroidError::ConsentError(format!(
                "Cannot suspend consent in {:?} status",
                self.status
            )));
        }
        self.status = ConsentStatus::Suspended;
        self.revocation_reason = Some(reason);
        Ok(())
    }

    /// Reactivate suspended consent
    pub fn reactivate(&mut self) -> Result<()> {
        if self.status != ConsentStatus::Suspended {
            return Err(CydroidError::ConsentError(format!(
                "Cannot reactivate consent in {:?} status",
                self.status
            )));
        }
        self.status = ConsentStatus::Active;
        self.revocation_reason = None;
        Ok(())
    }

    /// Check if consent is currently active
    pub fn is_active(&self) -> bool {
        self.status == ConsentStatus::Active
    }

    /// Check if consent has expired
    pub fn is_expired(&self) -> bool {
        for scope in &self.data_use_scopes {
            if let Some(expires_at) = scope.expires_at {
                if Utc::now() > expires_at {
                    return true;
                }
            }
        }
        false
    }

    /// Check if participant has consented to a specific scope
    pub fn has_scope(&self, scope_id: &str) -> bool {
        self.data_use_scopes
            .iter()
            .any(|s| s.scope_id == scope_id && s.expires_at.map_or(true, |exp| Utc::now() <= exp))
    }
}

impl Validatable for ConsentRecord {
    fn validate(&self) -> Result<()> {
        if self.participant_did.is_empty() {
            return Err(CydroidError::ValidationError(
                "participant_did cannot be empty".to_string(),
            ));
        }

        if !self.participant_did.starts_with("did:") {
            return Err(CydroidError::ValidationError(
                "participant_did must start with 'did:'".to_string(),
            ));
        }

        if self.fps_hash.is_empty() {
            return Err(CydroidError::ValidationError(
                "fps_hash cannot be empty".to_string(),
            ));
        }

        if self.fps_metadata.content_hash != self.fps_hash {
            return Err(CydroidError::ValidationError(
                "fps_hash must match fps_metadata.content_hash".to_string(),
            ));
        }

        if !self.comprehension_check.passed {
            return Err(CydroidError::ValidationError(
                "Consent cannot be granted without passing comprehension check".to_string(),
            ));
        }

        if self.data_use_scopes.is_empty() {
            return Err(CydroidError::ValidationError(
                "At least one data use scope is required".to_string(),
            ));
        }

        Ok(())
    }
}

/// Consent registry (manages multiple consent records)
#[derive(Debug, Clone)]
pub struct ConsentRegistry {
    records: std::collections::HashMap<Uuid, ConsentRecord>,
    /// Map from participant_did to consent IDs
    participant_index: std::collections::HashMap<String, Vec<Uuid>>,
}

impl ConsentRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            records: std::collections::HashMap::new(),
            participant_index: std::collections::HashMap::new(),
        }
    }

    /// Add a consent record to the registry
    pub fn add(&mut self, record: ConsentRecord) -> Result<()> {
        let consent_id = record.consent_id;
        let participant_did = record.participant_did.clone();

        self.records.insert(consent_id, record);
        self.participant_index
            .entry(participant_did)
            .or_insert_with(Vec::new)
            .push(consent_id);

        Ok(())
    }

    /// Get consent record by ID
    pub fn get(&self, consent_id: Uuid) -> Option<&ConsentRecord> {
        self.records.get(&consent_id)
    }

    /// Get all consent records for a participant
    pub fn get_by_participant(&self, participant_did: &str) -> Vec<&ConsentRecord> {
        self.participant_index
            .get(participant_did)
            .map(|ids| ids.iter().filter_map(|id| self.records.get(id)).collect())
            .unwrap_or_default()
    }

    /// Get all active consent records
    pub fn get_active(&self) -> Vec<&ConsentRecord> {
        self.records
            .values()
            .filter(|r| r.is_active())
            .collect()
    }

    /// Revoke a consent record
    pub fn revoke(&mut self, consent_id: Uuid, reason: String) -> Result<()> {
        let record = self
            .records
            .get_mut(&consent_id)
            .ok_or_else(|| CydroidError::ConsentError("Consent record not found".to_string()))?;
        record.revoke(reason)?;
        Ok(())
    }

    /// Check if participant has active consent for a scope
    pub fn has_active_consent_for_scope(
        &self,
        participant_did: &str,
        scope_id: &str,
    ) -> bool {
        self.get_by_participant(participant_did)
            .iter()
            .any(|r| r.is_active() && r.has_scope(scope_id))
    }
}

impl Default for ConsentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_comprehension_check() -> ComprehensionCheck {
        ComprehensionCheck {
            timestamp: Utc::now(),
            passed: true,
            attempts: 1,
            method: "quiz".to_string(),
        }
    }

    fn create_test_fps_metadata() -> FpsMetadata {
        FpsMetadata {
            version: "1.0".to_string(),
            language: "en".to_string(),
            content_hash: "abc123def456".to_string(),
            uri: Some("https://cydroid.org/fps/v1".to_string()),
            presentation_format: "text".to_string(),
        }
    }

    fn create_test_consent() -> ConsentRecord {
        ConsentRecord::new(
            "did:ion:participant".to_string(),
            "abc123def456".to_string(),
            "1.0".to_string(),
            create_test_comprehension_check(),
            vec![DataUseScope {
                scope_id: "mission_data".to_string(),
                description: "Mission telemetry and eco-metrics".to_string(),
                expires_at: None,
            }],
            create_test_fps_metadata(),
            true,
        )
        .unwrap()
    }

    #[test]
    fn test_consent_creation() {
        let consent = create_test_consent();
        assert!(consent.is_active());
        assert!(consent.has_scope("mission_data"));
    }

    #[test]
    fn test_consent_revocation() {
        let mut consent = create_test_consent();
        assert!(consent.is_active());
        consent.revoke("User requested".to_string()).unwrap();
        assert_eq!(consent.status, ConsentStatus::Revoked);
        assert!(!consent.is_active());
    }

    #[test]
    fn test_consent_registry() {
        let mut registry = ConsentRegistry::new();
        let consent = create_test_consent();
        let consent_id = consent.consent_id;
        registry.add(consent).unwrap();

        assert!(registry.get(consent_id).is_some());
        assert!(registry
            .has_active_consent_for_scope("did:ion:participant", "mission_data"));
    }

    #[test]
    fn test_comprehension_check_required() {
        let mut bad_check = create_test_comprehension_check();
        bad_check.passed = false;

        let result = ConsentRecord::new(
            "did:ion:participant".to_string(),
            "abc123".to_string(),
            "1.0".to_string(),
            bad_check,
            vec![DataUseScope {
                scope_id: "test".to_string(),
                description: "Test".to_string(),
                expires_at: None,
            }],
            create_test_fps_metadata(),
            true,
        );
        assert!(result.is_err());
    }
}
