//! ROW (Read-Only World) Ledger Core
//!
//! Append-only, cryptographically-anchored ledger for all Cydroid events.
//! Every entry is signed, hashed, and linked to the previous entry,
//! creating an immutable audit trail for governance and compliance.

use crate::error::{CydroidError, Result};
use crate::validate::Validatable;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Entry type enumeration for ROW ledger
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RowEntryType {
    NeuromorphicUpdate,
    SwarmPolicyExecution,
    EcoMetricObservation,
    ConsentEvent,
    SchemaEvolutionProposal,
    CommunityDecision,
    CareAccessCreditGrant,
    SafetyViolation,
    RollbackEvent,
}

/// ROW ledger entry payload (unified enum)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "entry_type", content = "payload")]
pub enum RowPayload {
    NeuromorphicUpdate {
        event_id: Uuid,
        channel_id: String,
        event_type: String,
    },
    SwarmPolicyExecution {
        policy_id: String,
        mission_id: String,
        decision: String,
    },
    EcoMetricObservation {
        metric_id: String,
        value: f64,
        unit: String,
    },
    ConsentEvent {
        consent_id: Uuid,
        participant_did: String,
        fps_hash: String,
        comprehension_passed: bool,
    },
    SchemaEvolutionProposal {
        proposal_id: Uuid,
        schema_version: String,
        multisig_signatures: Vec<String>,
    },
    CommunityDecision {
        decision_id: Uuid,
        council_id: String,
        vote_result: String,
    },
    CareAccessCreditGrant {
        cac_id: Uuid,
        recipient_did: String,
        amount: f64,
        mission_id: String,
    },
    SafetyViolation {
        violation_type: String,
        channel_id: String,
        severity: String,
    },
    RollbackEvent {
        rollback_id: Uuid,
        target_state_hash: String,
        reason: String,
    },
}

/// ROW ledger entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RowEntry {
    /// Unique ROW entry ID
    pub row_id: Uuid,
    /// SHA-256 hash of previous entry (empty for genesis)
    pub previous_entry_hash: String,
    /// Entry type
    pub entry_type: RowEntryType,
    /// Timestamp of entry creation
    pub timestamp: DateTime<Utc>,
    /// DID of entity creating this entry
    pub signer_did: String,
    /// Cryptographic signature (hex-encoded)
    pub signature: String,
    /// Entry payload
    pub payload: RowPayload,
    /// Evidence bundle IDs (10-hex strings)
    pub evidence_bundle_ids: Vec<String>,
    /// Optional reference to related ROW entries
    pub related_row_ids: Vec<Uuid>,
}

impl RowEntry {
    /// Create a genesis entry (first in chain)
    pub fn genesis(signer_did: String, payload: RowPayload) -> Result<Self> {
        let entry = Self {
            row_id: Uuid::new_v4(),
            previous_entry_hash: String::new(),
            entry_type: Self::payload_to_entry_type(&payload),
            timestamp: Utc::now(),
            signer_did,
            signature: String::new(), // Would be populated by signing service
            payload,
            evidence_bundle_ids: Vec::new(),
            related_row_ids: Vec::new(),
        };
        entry.validate()?;
        Ok(entry)
    }

    /// Create a new entry linked to a previous entry
    pub fn new(
        previous_entry: &RowEntry,
        signer_did: String,
        payload: RowPayload,
        signature: String,
    ) -> Result<Self> {
        let previous_hash = previous_entry.compute_hash()?;
        let entry = Self {
            row_id: Uuid::new_v4(),
            previous_entry_hash: previous_hash,
            entry_type: Self::payload_to_entry_type(&payload),
            timestamp: Utc::now(),
            signer_did,
            signature,
            payload,
            evidence_bundle_ids: Vec::new(),
            related_row_ids: Vec::new(),
        };
        entry.validate()?;
        Ok(entry)
    }

    /// Map payload to entry type
    fn payload_to_entry_type(payload: &RowPayload) -> RowEntryType {
        match payload {
            RowPayload::NeuromorphicUpdate { .. } => RowEntryType::NeuromorphicUpdate,
            RowPayload::SwarmPolicyExecution { .. } => RowEntryType::SwarmPolicyExecution,
            RowPayload::EcoMetricObservation { .. } => RowEntryType::EcoMetricObservation,
            RowPayload::ConsentEvent { .. } => RowEntryType::ConsentEvent,
            RowPayload::SchemaEvolutionProposal { .. } => RowEntryType::SchemaEvolutionProposal,
            RowPayload::CommunityDecision { .. } => RowEntryType::CommunityDecision,
            RowPayload::CareAccessCreditGrant { .. } => RowEntryType::CareAccessCreditGrant,
            RowPayload::SafetyViolation { .. } => RowEntryType::SafetyViolation,
            RowPayload::RollbackEvent { .. } => RowEntryType::RollbackEvent,
        }
    }

    /// Compute SHA-256 hash of this entry (for chaining)
    pub fn compute_hash(&self) -> Result<String> {
        let serialized = serde_json::to_vec(self).map_err(|e| {
            CydroidError::SerializationError(format!("Failed to serialize entry: {}", e))
        })?;
        let hash = Sha256::digest(&serialized);
        Ok(hex::encode(hash))
    }

    /// Verify chain integrity (previous hash matches)
    pub fn verify_chain_link(&self, previous_entry: &RowEntry) -> Result<bool> {
        let previous_hash = previous_entry.compute_hash()?;
        Ok(self.previous_entry_hash == previous_hash)
    }

    /// Add evidence bundle ID to entry
    pub fn add_evidence_bundle(&mut self, bundle_id: String) -> Result<()> {
        crate::validate::helpers::validate_hex_10(&bundle_id)?;
        if !self.evidence_bundle_ids.contains(&bundle_id) {
            self.evidence_bundle_ids.push(bundle_id);
        }
        Ok(())
    }

    /// Check if entry is a consent event
    pub fn is_consent_event(&self) -> bool {
        matches!(self.entry_type, RowEntryType::ConsentEvent)
    }

    /// Check if entry is a safety violation
    pub fn is_safety_violation(&self) -> bool {
        matches!(self.entry_type, RowEntryType::SafetyViolation)
    }
}

impl Validatable for RowEntry {
    fn validate(&self) -> Result<()> {
        if self.signer_did.is_empty() {
            return Err(CydroidError::ValidationError(
                "signer_did cannot be empty".to_string(),
            ));
        }

        if !self.signer_did.starts_with("did:") {
            return Err(CydroidError::ValidationError(
                "signer_did must start with 'did:'".to_string(),
            ));
        }

        // Validate evidence bundle IDs (10-hex format)
        for bundle_id in &self.evidence_bundle_ids {
            crate::validate::helpers::validate_hex_10(bundle_id)?;
        }

        // Genesis entry has no previous hash
        if !self.previous_entry_hash.is_empty() && self.previous_entry_hash.len() != 64 {
            return Err(CydroidError::ValidationError(
                "previous_entry_hash must be 64 characters (SHA-256 hex)".to_string(),
            ));
        }

        Ok(())
    }
}

/// ROW ledger (in-memory for now, can be extended to persistent storage)
#[derive(Debug, Clone)]
pub struct RowLedger {
    entries: Vec<RowEntry>,
    /// Map from row_id to index for fast lookup
    entry_index: std::collections::HashMap<Uuid, usize>,
}

impl RowLedger {
    /// Create a new empty ledger
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            entry_index: std::collections::HashMap::new(),
        }
    }

    /// Append an entry to the ledger
    pub fn append(&mut self, entry: RowEntry) -> Result<()> {
        // Verify chain link if not genesis
        if !entry.previous_entry_hash.is_empty() {
            if let Some(last_entry) = self.entries.last() {
                if !entry.verify_chain_link(last_entry)? {
                    return Err(CydroidError::RowLedgerError(
                        "Chain link verification failed".to_string(),
                    ));
                }
            }
        }

        let row_id = entry.row_id;
        let index = self.entries.len();
        self.entries.push(entry);
        self.entry_index.insert(row_id, index);
        Ok(())
    }

    /// Get entry by row_id
    pub fn get_entry(&self, row_id: Uuid) -> Option<&RowEntry> {
        self.entry_index.get(&row_id).map(|&idx| &self.entries[idx])
    }

    /// Get all entries of a specific type
    pub fn get_entries_by_type(&self, entry_type: RowEntryType) -> Vec<&RowEntry> {
        self.entries
            .iter()
            .filter(|e| e.entry_type == entry_type)
            .collect()
    }

    /// Get all consent events
    pub fn get_consent_events(&self) -> Vec<&RowEntry> {
        self.get_entries_by_type(RowEntryType::ConsentEvent)
    }

    /// Get all safety violations
    pub fn get_safety_violations(&self) -> Vec<&RowEntry> {
        self.get_entries_by_type(RowEntryType::SafetyViolation)
    }

    /// Get ledger length
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if ledger is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get the latest entry (tip of chain)
    pub fn tip(&self) -> Option<&RowEntry> {
        self.entries.last()
    }

    /// Export ledger to JSON (for audit/backup)
    pub fn export_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.entries).map_err(|e| {
            CydroidError::SerializationError(format!("Failed to export ledger: {}", e))
        })
    }
}

impl Default for RowLedger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_entry() {
        let payload = RowPayload::ConsentEvent {
            consent_id: Uuid::new_v4(),
            participant_did: "did:ion:test".to_string(),
            fps_hash: "abc123def456".to_string(),
            comprehension_passed: true,
        };
        let entry = RowEntry::genesis("did:ion:creator".to_string(), payload);
        assert!(entry.is_ok());
        let entry = entry.unwrap();
        assert!(entry.previous_entry_hash.is_empty());
    }

    #[test]
    fn test_chain_linking() {
        let mut ledger = RowLedger::new();

        // Genesis entry
        let payload1 = RowPayload::ConsentEvent {
            consent_id: Uuid::new_v4(),
            participant_did: "did:ion:user1".to_string(),
            fps_hash: "hash1".to_string(),
            comprehension_passed: true,
        };
        let entry1 = RowEntry::genesis("did:ion:creator".to_string(), payload1).unwrap();
        let hash1 = entry1.compute_hash().unwrap();
        ledger.append(entry1).unwrap();

        // Second entry
        let payload2 = RowPayload::NeuromorphicUpdate {
            event_id: Uuid::new_v4(),
            channel_id: "EEG_alpha".to_string(),
            event_type: "NeuSpike".to_string(),
        };
        let entry2 = RowEntry::new(
            ledger.tip().unwrap(),
            "did:ion:operator".to_string(),
            payload2,
            "signature2".to_string(),
        )
        .unwrap();
        ledger.append(entry2).unwrap();

        assert_eq!(ledger.len(), 2);
        assert_eq!(ledger.tip().unwrap().previous_entry_hash, hash1);
    }

    #[test]
    fn test_evidence_bundle_validation() {
        let payload = RowPayload::EcoMetricObservation {
            metric_id: "turbidity_01".to_string(),
            value: 5.2,
            unit: "NTU".to_string(),
        };
        let mut entry = RowEntry::genesis("did:ion:creator".to_string(), payload).unwrap();

        // Valid 10-hex bundle ID
        assert!(entry.add_evidence_bundle("a1f3c9b2d8".to_string()).is_ok());
        // Invalid (too short)
        assert!(entry.add_evidence_bundle("abc123".to_string()).is_err());
        // Invalid (uppercase)
        assert!(entry.add_evidence_bundle("A1F3C9B2D8".to_string()).is_err());
    }
}
