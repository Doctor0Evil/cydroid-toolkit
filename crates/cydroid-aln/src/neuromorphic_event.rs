//! Neuromorphic Event Frame Definitions
//!
//! This module defines the event frames for neuromorphic spikes,
//! environmental snapshots, and eco-score events. All events carry
//! explicit data quality flags and are bound to NeuroChannel IDs.

use crate::error::{CydroidError, Result};
use crate::neurochannel::{NeuroChannel, ChannelCategory};
use crate::validate::Validatable;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Data quality flags for neuromorphic events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataQualityFlag {
    /// Sensor drift detected
    SensorDrift,
    /// Biofouling suspected (environmental sensors)
    Biofouling,
    /// Motion artifact present
    MotionArtifact,
    /// Low signal-to-noise ratio
    LowSNR,
    /// Partial occlusion
    PartialOcclusion,
    /// Calibration expired
    CalibrationExpired,
    /// Envelope violation (biophysical safety)
    EnvelopeViolation,
}

/// Event type enumeration for neuromorphic frames
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EventType {
    NeuSpike,
    NeuSpikeTrain,
    NeuSpikeBatch,
    EnvSnapshot,
    EcoScore,
}

/// Payload for spike events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpikePayload {
    /// Spike timestamp (relative to event start)
    pub timestamp_us: u64,
    /// Spike amplitude (in channel units)
    pub amplitude: f64,
    /// Spike width (microseconds)
    pub width_us: Option<u32>,
}

/// Payload for spike train events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpikeTrainPayload {
    /// Number of spikes in train
    pub spike_count: u32,
    /// Mean inter-spike interval (microseconds)
    pub mean_isi_us: f64,
    /// Standard deviation of ISI
    pub isi_std_us: f64,
    /// Train duration (microseconds)
    pub duration_us: u64,
}

/// Payload for spike batch events (multiple trains)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpikeBatchPayload {
    /// Number of spike trains in batch
    pub train_count: u32,
    /// Total spike count
    pub total_spikes: u32,
    /// Mean firing rate (Hz)
    pub mean_firing_rate_hz: f64,
    /// Burst index (0.0-1.0)
    pub burst_index: f32,
}

/// Payload for environmental snapshot events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvSnapshotPayload {
    /// Measured value
    pub value: f64,
    /// Unit (e.g., "m³/m³", "mg/L", "µg/m³")
    pub unit: String,
    /// Measurement uncertainty
    pub uncertainty: f64,
    /// Integration time (seconds)
    pub integration_time_s: Option<u32>,
}

/// Payload for eco-score events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcoScorePayload {
    /// Normalized eco-score (0.0-1.0)
    pub score: f32,
    /// Biome tag context
    pub biome_tag: String,
    /// Hazard tag context
    pub hazard_tag: String,
    /// Contributing metric ROW IDs
    pub contributing_metric_row_ids: Vec<String>,
    /// Eco-floor threshold (e.g., 0.86)
    pub eco_floor: f32,
}

/// Unified payload enum for all event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "payload_type", content = "payload")]
pub enum EventPayload {
    Spike(SpikePayload),
    SpikeTrain(SpikeTrainPayload),
    SpikeBatch(SpikeBatchPayload),
    EnvSnapshot(EnvSnapshotPayload),
    EcoScore(EcoScorePayload),
}

/// Neuromorphic event frame
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuromorphicEvent {
    /// Unique event ID
    pub event_id: Uuid,
    /// Event type
    pub event_type: EventType,
    /// Timestamp of event creation
    pub timestamp: DateTime<Utc>,
    /// Associated channel ID
    pub channel_id: String,
    /// Data quality flags (empty if all clear)
    pub data_quality_flags: Vec<DataQualityFlag>,
    /// Event payload
    pub payload: EventPayload,
    /// Optional reference to previous event (for chain validation)
    pub previous_event_id: Option<Uuid>,
    /// ROW entry ID (populated when logged)
    pub row_entry_id: Option<Uuid>,
}

impl NeuromorphicEvent {
    /// Create a new spike event
    pub fn new_spike(
        channel_id: String,
        timestamp_us: u64,
        amplitude: f64,
    ) -> Result<Self> {
        let event = Self {
            event_id: Uuid::new_v4(),
            event_type: EventType::NeuSpike,
            timestamp: Utc::now(),
            channel_id,
            data_quality_flags: Vec::new(),
            payload: EventPayload::Spike(SpikePayload {
                timestamp_us,
                amplitude,
                width_us: None,
            }),
            previous_event_id: None,
            row_entry_id: None,
        };
        event.validate()?;
        Ok(event)
    }

    /// Create a new environmental snapshot event
    pub fn new_env_snapshot(
        channel_id: String,
        value: f64,
        unit: String,
        uncertainty: f64,
    ) -> Result<Self> {
        let event = Self {
            event_id: Uuid::new_v4(),
            event_type: EventType::EnvSnapshot,
            timestamp: Utc::now(),
            channel_id,
            data_quality_flags: Vec::new(),
            payload: EventPayload::EnvSnapshot(EnvSnapshotPayload {
                value,
                unit,
                uncertainty,
                integration_time_s: None,
            }),
            previous_event_id: None,
            row_entry_id: None,
        };
        event.validate()?;
        Ok(event)
    }

    /// Create a new eco-score event
    pub fn new_eco_score(
        channel_id: String,
        score: f32,
        biome_tag: String,
        hazard_tag: String,
        contributing_metric_row_ids: Vec<String>,
        eco_floor: f32,
    ) -> Result<Self> {
        let event = Self {
            event_id: Uuid::new_v4(),
            event_type: EventType::EcoScore,
            timestamp: Utc::now(),
            channel_id,
            data_quality_flags: Vec::new(),
            payload: EventPayload::EcoScore(EcoScorePayload {
                score,
                biome_tag,
                hazard_tag,
                contributing_metric_row_ids,
                eco_floor,
            }),
            previous_event_id: None,
            row_entry_id: None,
        };
        event.validate()?;
        Ok(event)
    }

    /// Add a data quality flag to the event
    pub fn add_quality_flag(&mut self, flag: DataQualityFlag) {
        if !self.data_quality_flags.contains(&flag) {
            self.data_quality_flags.push(flag);
        }
    }

    /// Check if event has any quality issues
    pub fn has_quality_issues(&self) -> bool {
        !self.data_quality_flags.is_empty()
    }

    /// Check if eco-score meets floor (for EcoScore events)
    pub fn meets_eco_floor(&self) -> Result<bool> {
        match &self.payload {
            EventPayload::EcoScore(eco) => Ok(eco.score >= eco.eco_floor),
            _ => Err(CydroidError::ValidationError(
                "Not an EcoScore event".to_string(),
            )),
        }
    }

    /// Get the channel category from payload type
    pub fn channel_category(&self) -> ChannelCategory {
        match self.event_type {
            EventType::NeuSpike | EventType::NeuSpikeTrain | EventType::NeuSpikeBatch => {
                ChannelCategory::Biophysical
            }
            EventType::EnvSnapshot | EventType::EcoScore => ChannelCategory::Environmental,
        }
    }
}

impl Validatable for NeuromorphicEvent {
    fn validate(&self) -> Result<()> {
        if self.channel_id.is_empty() {
            return Err(CydroidError::ValidationError(
                "channel_id cannot be empty".to_string(),
            ));
        }

        // Validate eco-score payload if present
        if let EventPayload::EcoScore(eco) = &self.payload {
            if eco.score < 0.0 || eco.score > 1.0 {
                return Err(CydroidError::ValidationError(format!(
                    "EcoScore must be in [0.0, 1.0], got {}",
                    eco.score
                )));
            }
            if eco.eco_floor < 0.0 || eco.eco_floor > 1.0 {
                return Err(CydroidError::ValidationError(format!(
                    "EcoFloor must be in [0.0, 1.0], got {}",
                    eco.eco_floor
                )));
            }
        }

        // Validate env snapshot payload
        if let EventPayload::EnvSnapshot(env) = &self.payload {
            if env.uncertainty < 0.0 {
                return Err(CydroidError::ValidationError(
                    "Uncertainty cannot be negative".to_string(),
                ));
            }
            if env.unit.is_empty() {
                return Err(CydroidError::ValidationError(
                    "Unit cannot be empty".to_string(),
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spike_event_creation() {
        let event = NeuromorphicEvent::new_spike("EEG_alpha_frontal".to_string(), 1000, 50.0);
        assert!(event.is_ok());
        let event = event.unwrap();
        assert_eq!(event.event_type, EventType::NeuSpike);
        assert!(!event.has_quality_issues());
    }

    #[test]
    fn test_eco_score_validation() {
        let event = NeuromorphicEvent::new_eco_score(
            "Canal_Segment_12".to_string(),
            0.92,
            "URBAN_CANAL".to_string(),
            "NONE".to_string(),
            vec!["row_abc123".to_string()],
            0.86,
        );
        assert!(event.is_ok());
        let event = event.unwrap();
        assert!(event.meets_eco_floor().unwrap());
    }

    #[test]
    fn test_eco_floor_violation() {
        let event = NeuromorphicEvent::new_eco_score(
            "Canal_Segment_12".to_string(),
            0.75,
            "URBAN_CANAL".to_string(),
            "NONE".to_string(),
            vec![],
            0.86,
        )
        .unwrap();
        assert!(!event.meets_eco_floor().unwrap());
    }

    #[test]
    fn test_quality_flag_addition() {
        let mut event = NeuromorphicEvent::new_spike("EMG_forearm".to_string(), 500, 30.0).unwrap();
        assert!(!event.has_quality_issues());
        event.add_quality_flag(DataQualityFlag::MotionArtifact);
        assert!(event.has_quality_issues());
    }
}
