//! Neuromorphic Encoder Kernel Wrapper
//!
//! Interfaces with the Mojo kernel or provides a pure Rust fallback.
//! Ensures all encoded events are validated against ALN NeuroChannel schemas.

use crate::error::{CydroidError, Result};
use crate::neurochannel::{NeuroChannel, SafetyLimits};
use crate::neuromorphic_event::{NeuromorphicEvent, DataQualityFlag, EventType, EventPayload, SpikePayload};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Encoder state for a single channel
pub struct EncoderState {
    pub channel: NeuroChannel,
    pub last_spike_time: DateTime<Utc>,
    pub last_value: f64,
    pub accumulator: f64,
    pub threshold: f64,
    pub refractory_period_ms: u64,
}

impl EncoderState {
    /// Create a new encoder state from a NeuroChannel
    pub fn new(channel: NeuroChannel, threshold: f64, refractory_period_ms: u64) -> Result<Self> {
        // Validate threshold against safety limits
        let range = channel.safety_limits.max_value - channel.safety_limits.min_value;
        if threshold > range {
            return Err(CydroidError::ValidationError(
                "Threshold exceeds channel range".to_string(),
            ));
        }

        Ok(Self {
            channel,
            last_spike_time: Utc::now(),
            last_value: 0.0,
            accumulator: 0.0,
            threshold,
            refractory_period_ms,
        })
    }

    /// Process a new sample and return an event if a spike is generated
    pub fn process_sample(&mut self, value: f64, timestamp: DateTime<Utc>) -> Result<Option<NeuromorphicEvent>> {
        // 1. Safety Envelope Check
        if !self.channel.is_within_limits(value) {
            // Emit safety violation event
            let mut event = NeuromorphicEvent::new_spike(
                self.channel.channel_id.clone(),
                0,
                value,
            )?;
            event.add_quality_flag(DataQualityFlag::EnvelopeViolation);
            return Ok(Some(event));
        }

        // 2. Delta Calculation
        let delta = value - self.last_value;
        self.accumulator += delta;

        // 3. Threshold Check
        if self.accumulator.abs() >= self.threshold {
            // 4. Refractory Check
            let elapsed = timestamp.signed_duration_since(self.last_spike_time).num_milliseconds() as u64;
            if elapsed >= self.refractory_period_ms {
                self.last_spike_time = timestamp;
                self.accumulator = 0.0;

                // 5. Emit Spike
                let mut event = NeuromorphicEvent::new_spike(
                    self.channel.channel_id.clone(),
                    elapsed as u64,
                    value,
                )?;
                // Add calibration metadata reference
                return Ok(Some(event));
            }
        }

        self.last_value = value;
        Ok(None)
    }
}

/// Batch encoder for multiple channels
pub struct ChannelEncoder {
    pub states: Vec<EncoderState>,
}

impl ChannelEncoder {
    pub fn new(channels: Vec<NeuroChannel>) -> Result<Self> {
        let states = channels
            .into_iter()
            .map(|c| EncoderState::new(c, 0.5, 10)) // Default threshold/refractory
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { states })
    }

    pub fn process_batch(&mut self, values: Vec<f64>, timestamp: DateTime<Utc>) -> Result<Vec<NeuromorphicEvent>> {
        let mut events = Vec::new();
        for (i, state) in self.states.iter_mut().enumerate() {
            if let Some(val) = values.get(i) {
                if let Some(event) = state.process_sample(*val, timestamp)? {
                    events.push(event);
                }
            }
        }
        Ok(events)
    }
}
