//! Care Access Credits (CAC)
//!
//! Non-monetary, non-transferable incentives for ecological contribution.
//! Bound to DID and redeemable for healthcare/rehabilitation services.

use crate::error::{CydroidError, Result};
use crate::validate::Validatable;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Care Access Credit record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CareAccessCredit {
    /// Unique CAC ID
    pub cac_id: Uuid,
    /// Recipient DID
    pub recipient_did: String,
    /// Amount of credits
    pub amount: f64,
    /// Mission ID that earned this credit
    pub mission_id: Uuid,
    /// Conversion rule (e.g., "1000 ecoscore = 1 CAC")
    pub conversion_rule: String,
    /// Redeemable services
    pub redeemable_services: Vec<String>,
    /// Non-transferable flag (always true)
    pub non_transferable: bool,
    /// Issuance timestamp
    pub issued_at: DateTime<Utc>,
    /// Expiration timestamp (optional)
    pub expires_at: Option<DateTime<Utc>>,
    /// ROW entry ID logging this grant
    pub row_entry_id: Option<Uuid>,
}

impl CareAccessCredit {
    /// Create a new CAC
    pub fn new(
        recipient_did: String,
        amount: f64,
        mission_id: Uuid,
        conversion_rule: String,
        redeemable_services: Vec<String>,
    ) -> Result<Self> {
        let credit = Self {
            cac_id: Uuid::new_v4(),
            recipient_did,
            amount,
            mission_id,
            conversion_rule,
            redeemable_services,
            non_transferable: true,
            issued_at: Utc::now(),
            expires_at: None,
            row_entry_id: None,
        };
        credit.validate()?;
        Ok(credit)
    }

    /// Redeem credits (reduces amount)
    pub fn redeem(&mut self, amount: f64) -> Result<()> {
        if amount > self.amount {
            return Err(CydroidError::ConsentError(
                "Insufficient CAC balance".to_string(),
            ));
        }
        self.amount -= amount;
        Ok(())
    }
}

impl Validatable for CareAccessCredit {
    fn validate(&self) -> Result<()> {
        if self.recipient_did.is_empty() || !self.recipient_did.starts_with("did:") {
            return Err(CydroidError::ValidationError(
                "Recipient DID must be valid".to_string(),
            ));
        }

        if self.amount < 0.0 {
            return Err(CydroidError::ValidationError(
                "CAC amount cannot be negative".to_string(),
            ));
        }

        if !self.non_transferable {
            return Err(CydroidError::ValidationError(
                "CAC must be non-transferable".to_string(),
            ));
        }

        if self.redeemable_services.is_empty() {
            return Err(CydroidError::ValidationError(
                "At least one redeemable service required".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cac_creation() {
        let cac = CareAccessCredit::new(
            "did:ion:user".to_string(),
            100.0,
            Uuid::new_v4(),
            "1000 ecoscore = 1 CAC".to_string(),
            vec!["telehealth".to_string()],
        );
        assert!(cac.is_ok());
        assert!(cac.unwrap().non_transferable);
    }
}
