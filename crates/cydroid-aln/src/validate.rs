//! Validation trait for ALN types

use crate::error::Result;

/// Trait for validating ALN schema instances
pub trait Validatable {
    /// Validate the instance against schema constraints
    fn validate(&self) -> Result<()>;

    /// Validate and return detailed error messages
    fn validate_detailed(&self) -> Vec<String> {
        match self.validate() {
            Ok(()) => vec![],
            Err(e) => vec![e.to_string()],
        }
    }
}

/// Validation helpers for common patterns
pub mod helpers {
    use crate::error::{CydroidError, Result};

    /// Validate a hex string (10 characters, lowercase)
    pub fn validate_hex_10(hex: &str) -> Result<()> {
        if hex.len() != 10 {
            return Err(CydroidError::ValidationError(format!(
                "Hex string must be 10 characters, got {}",
                hex.len()
            )));
        }
        if !hex.chars().all(|c| c.is_ascii_hexdigit() && c.is_lowercase()) {
            return Err(CydroidError::ValidationError(
                "Hex string must be lowercase hex digits".to_string(),
            ));
        }
        Ok(())
    }

    /// Validate a DID string (basic format check)
    pub fn validate_did(did: &str) -> Result<()> {
        if !did.starts_with("did:") {
            return Err(CydroidError::ValidationError(
                "DID must start with 'did:'".to_string(),
            ));
        }
        if did.len() < 10 {
            return Err(CydroidError::ValidationError(
                "DID too short".to_string(),
            ));
        }
        Ok(())
    }

    /// Validate ecoscore is within [0.0, 1.0]
    pub fn validate_ecoscore(score: f32) -> Result<()> {
        if score < 0.0 || score > 1.0 {
            return Err(CydroidError::ValidationError(format!(
                "Ecoscore must be in [0.0, 1.0], got {}",
                score
            )));
        }
        Ok(())
    }

    /// Validate ecoscore meets minimum floor
    pub fn validate_ecoscore_floor(score: f32, floor: f32) -> Result<()> {
        if score < floor {
            return Err(CydroidError::EcoFloorNotMet {
                actual: score,
                required: floor,
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::helpers::*;

    #[test]
    fn test_hex_validation() {
        assert!(validate_hex_10("a1f3c9b2d8").is_ok());
        assert!(validate_hex_10("A1F3C9B2D8").is_err());
        assert!(validate_hex_10("a1f3c9b2").is_err());
    }

    #[test]
    fn test_did_validation() {
        assert!(validate_did("did:ion:EiD8J2b3K8k9Q8x9").is_ok());
        assert!(validate_did("ion:EiD8J2b3K8k9Q8x9").is_err());
    }

    #[test]
    fn test_ecoscore_validation() {
        assert!(validate_ecoscore(0.86).is_ok());
        assert!(validate_ecoscore(1.5).is_err());
        assert!(validate_ecoscore_floor(0.90, 0.86).is_ok());
        assert!(validate_ecoscore_floor(0.80, 0.86).is_err());
    }
}
