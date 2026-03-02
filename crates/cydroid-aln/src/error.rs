//! Error types for Cydroid ALN operations

use thiserror::Error;

/// Result type alias for Cydroid operations
pub type Result<T> = std::result::Result<T, CydroidError>;

/// Comprehensive error types for the Cydroid ecosystem
#[derive(Error, Debug)]
pub enum CydroidError {
    /// Validation error for schema violations
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// ROW ledger error (append-only violations, hash mismatches)
    #[error("ROW ledger error: {0}")]
    RowLedgerError(String),

    /// Consent error (revoked, missing, comprehension failed)
    #[error("Consent error: {0}")]
    ConsentError(String),

    /// Safety limit violation (biophysical envelope breach)
    #[error("Safety limit violation: {0}")]
    SafetyViolation(String),

    /// Eco-floor not met (ecoscore below threshold)
    #[error("Eco-floor not met: ecoscore {actual} < {required}")]
    EcoFloorNotMet { actual: f32, required: f32 },

    /// Schema evolution error (forward-only violation)
    #[error("Schema evolution error: {0}")]
    SchemaEvolutionError(String),

    /// Cryptographic error (signature verification, DID resolution)
    #[error("Cryptographic error: {0}")]
    CryptoError(String),

    /// Serialization error (ALN encoding/decoding)
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Mission execution error (corridor violation, policy breach)
    #[error("Mission execution error: {0}")]
    MissionError(String),

    /// Data quality error (sensor drift, biofouling, low SNR)
    #[error("Data quality error: {0}")]
    DataQualityError(String),
}

impl From<serde_json::Error> for CydroidError {
    fn from(err: serde_json::Error) -> Self {
        CydroidError::SerializationError(err.to_string())
    }
}

impl From<uuid::Error> for CydroidError {
    fn from(err: uuid::Error) -> Self {
        CydroidError::ValidationError(err.to_string())
    }
}
