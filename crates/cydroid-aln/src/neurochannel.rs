//! NeuroChannel definitions with biophysical safety limits

use crate::error::{CydroidError, Result};
use crate::validate::Validatable;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Modality types for neurochannels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModalityType {
    EEG,
    EMG,
    E_SKIN,
    IMU,
    SOIL_MOISTURE,
    WATER_QUALITY,
    AIR_QUALITY,
}

/// Calibration metadata for quantitative measurements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationMetadata {
    /// Date of last calibration
    pub calibration_date: DateTime<Utc>,
    /// Method used for calibration (e.g., "NIST_traceable")
    pub calibration_method: String,
    /// Uncertainty estimate (standard deviation or confidence interval)
    pub uncertainty_estimate: f64,
}

/// Safety limits for biophysical channels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyLimits {
    /// Maximum allowable value for this channel
    pub max_value: f64,
    /// Minimum allowable value for this channel
    pub min_value: f64,
    /// Maximum electrode current density (µA/cm²) - critical for EEG/EMG/E-skin
    pub max_current_density_uA_per_cm2: f64,
    /// Maximum skin contact duration (minutes) - optional for non-contact sensors
    pub max_skin_contact_duration_minutes: Option<u32>,
    /// Maximum device surface temperature (°C) - optional for non-wearable sensors
    pub max_device_temperature_celsius: Option<f64>,
}

/// NeuroChannel definition with safety and calibration metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuroChannel {
    /// Unique identifier for this channel
    pub channel_id: String,
    /// Sensing modality type
    pub modality_type: ModalityType,
    /// Unit of measurement (SI or domain-specific)
    pub unit: String,
    /// Calibration metadata
    pub calibration_metadata: CalibrationMetadata,
    /// Safety limits (machine-enforced)
    pub safety_limits: SafetyLimits,
    /// Semantic URI linking to external ontology (e.g., SWEET, OGC)
    pub semantic_uri: Option<String>,
}

impl NeuroChannel {
    /// Create a new NeuroChannel with validation
    pub fn new(
        channel_id: String,
        modality_type: ModalityType,
        unit: String,
        calibration_metadata: CalibrationMetadata,
        safety_limits: SafetyLimits,
        semantic_uri: Option<String>,
    ) -> Result<Self> {
        let channel = Self {
            channel_id,
            modality_type,
            unit,
            calibration_metadata,
            safety_limits,
            semantic_uri,
        };
        channel.validate()?;
        Ok(channel)
    }

    /// Check if a value is within safety limits
    pub fn is_within_limits(&self, value: f64) -> bool {
        value >= self.safety_limits.min_value && value <= self.safety_limits.max_value
    }

    /// Check if current density is safe (for contact modalities)
    pub fn is_current_density_safe(&self, current_density: f64) -> bool {
        current_density <= self.safety_limits.max_current_density_uA_per_cm2
    }

    /// Get the modality category (biophysical vs environmental)
    pub fn category(&self) -> ChannelCategory {
        match self.modality_type {
            ModalityType::EEG | ModalityType::EMG | ModalityType::E_SKIN | ModalityType::IMU => {
                ChannelCategory::Biophysical
            }
            ModalityType::SOIL_MOISTURE
            | ModalityType::WATER_QUALITY
            | ModalityType::AIR_QUALITY => ChannelCategory::Environmental,
        }
    }
}

/// Channel category for routing and policy decisions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelCategory {
    Biophysical,
    Environmental,
}

impl Validatable for NeuroChannel {
    fn validate(&self) -> Result<()> {
        if self.channel_id.is_empty() {
            return Err(CydroidError::ValidationError(
                "channel_id cannot be empty".to_string(),
            ));
        }

        if self.unit.is_empty() {
            return Err(CydroidError::ValidationError(
                "unit cannot be empty".to_string(),
            ));
        }

        if self.safety_limits.max_value < self.safety_limits.min_value {
            return Err(CydroidError::ValidationError(
                "max_value must be >= min_value".to_string(),
            ));
        }

        if self.safety_limits.max_current_density_uA_per_cm2 < 0.0 {
            return Err(CydroidError::ValidationError(
                "max_current_density cannot be negative".to_string(),
            ));
        }

        if self.calibration_metadata.uncertainty_estimate < 0.0 {
            return Err(CydroidError::ValidationError(
                "uncertainty_estimate cannot be negative".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_channel() -> NeuroChannel {
        NeuroChannel {
            channel_id: "EEG_alpha_frontal".to_string(),
            modality_type: ModalityType::EEG,
            unit: "µV".to_string(),
            calibration_metadata: CalibrationMetadata {
                calibration_date: Utc::now(),
                calibration_method: "NIST_traceable".to_string(),
                uncertainty_estimate: 0.02,
            },
            safety_limits: SafetyLimits {
                max_value: 100.0,
                min_value: -100.0,
                max_current_density_uA_per_cm2: 0.5,
                max_skin_contact_duration_minutes: Some(120),
                max_device_temperature_celsius: Some(40.0),
            },
            semantic_uri: None,
        }
    }

    #[test]
    fn test_channel_validation() {
        let channel = create_test_channel();
        assert!(channel.validate().is_ok());
    }

    #[test]
    fn test_within_limits() {
        let channel = create_test_channel();
        assert!(channel.is_within_limits(50.0));
        assert!(!channel.is_within_limits(150.0));
    }

    #[test]
    fn test_channel_category() {
        let bio_channel = create_test_channel();
        assert_eq!(bio_channel.category(), ChannelCategory::Biophysical);

        let env_channel = NeuroChannel {
            channel_id: "SoilMoisture_30cm".to_string(),
            modality_type: ModalityType::SOIL_MOISTURE,
            unit: "m³/m³".to_string(),
            calibration_metadata: CalibrationMetadata {
                calibration_date: Utc::now(),
                calibration_method: "TDR".to_string(),
                uncertainty_estimate: 0.01,
            },
            safety_limits: SafetyLimits {
                max_value: 1.0,
                min_value: 0.0,
                max_current_density_uA_per_cm2: 0.0,
                max_skin_contact_duration_minutes: None,
                max_device_temperature_celsius: None,
            },
            semantic_uri: None,
        };
        assert_eq!(env_channel.category(), ChannelCategory::Environmental);
    }
}
