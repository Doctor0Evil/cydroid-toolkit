//! Swarm Policy States and Biophysical Triggers
//!
//! This module defines policy states for robot swarm control,
//! including biophysical modulation rules, eco-floor enforcement,
//! and biome/hazard-aware policy selection.

use crate::error::{CydroidError, Result};
use crate::neuromorphic_event::NeuromorphicEvent;
use crate::validate::Validatable;
use crate::MINIMUM_ECOSCORE;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Intent class for swarm policies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IntentClass {
    CanalClean,
    RiparianPlanting,
    UrbanHeatRelief,
    PollutantInterception,
    MicrobialSeeding,
    BiodiversityMonitoring,
    EmergencyResponse,
}

/// Biome tag for policy context
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BiomeTag {
    UrbanCanal,
    CoastalWetland,
    SemiAridRiver,
    PeriUrbanAgriculture,
    DesertCorridor,
}

/// Hazard tag for policy context
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HazardTag {
    Heatwave,
    FloodEvent,
    WildfireSmoke,
    SevereStorm,
    None,
}

/// Biophysical trigger conditions for swarm modulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiophysicalTriggers {
    /// Maximum swarm tempo when human fatigue is high (0.0-1.0)
    pub high_fatigue_max_tempo: f32,
    /// Minimum safety radius when human stress is high (meters)
    pub high_stress_min_safety_radius: f32,
    /// Maximum intervention depth when cognitive load is high (meters)
    pub high_cognitive_load_max_depth: f32,
    /// Pause swarm if pain exceeds threshold (0.0-10.0 scale)
    pub pain_pause_threshold: Option<f32>,
}

impl Default for BiophysicalTriggers {
    fn default() -> Self {
        Self {
            high_fatigue_max_tempo: 0.5,
            high_stress_min_safety_radius: 5.0,
            high_cognitive_load_max_depth: 0.5,
            pain_pause_threshold: Some(3.0),
        }
    }
}

/// Safety constraints for swarm operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyConstraints {
    /// Maximum swarm tempo (operations per second)
    pub max_swarm_tempo: f32,
    /// Minimum safety radius (meters)
    pub min_safety_radius_meters: f32,
    /// Maximum intervention depth (meters)
    pub max_intervention_depth: f32,
    /// Maximum concurrent robots in swarm
    pub max_concurrent_robots: u32,
}

impl Default for SafetyConstraints {
    fn default() -> Self {
        Self {
            max_swarm_tempo: 10.0,
            min_safety_radius_meters: 2.0,
            max_intervention_depth: 2.0,
            max_concurrent_robots: 50,
        }
    }
}

/// Swarm policy state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmPolicy {
    /// Unique policy ID
    pub policy_id: String,
    /// Intent class
    pub intent_class: IntentClass,
    /// Biome tag
    pub biome_tag: BiomeTag,
    /// Hazard tag
    pub hazard_tag: HazardTag,
    /// Safety constraints
    pub safety_constraints: SafetyConstraints,
    /// Minimum eco-score required for continued operation
    pub eco_floor: f32,
    /// Biophysical triggers for human-robotics modulation
    pub biophysical_triggers: BiophysicalTriggers,
    /// Whether policy is currently active
    pub is_active: bool,
    /// ROW entry ID where policy was logged
    pub row_entry_id: Option<Uuid>,
    /// Evidence bundle IDs
    pub evidence_bundle_ids: Vec<String>,
}

impl SwarmPolicy {
    /// Create a new swarm policy
    pub fn new(
        policy_id: String,
        intent_class: IntentClass,
        biome_tag: BiomeTag,
        hazard_tag: HazardTag,
        safety_constraints: SafetyConstraints,
        eco_floor: f32,
        biophysical_triggers: BiophysicalTriggers,
    ) -> Result<Self> {
        let policy = Self {
            policy_id,
            intent_class,
            biome_tag,
            hazard_tag,
            safety_constraints,
            eco_floor,
            biophysical_triggers,
            is_active: false,
            row_entry_id: None,
            evidence_bundle_ids: Vec::new(),
        };
        policy.validate()?;
        Ok(policy)
    }

    /// Activate the policy
    pub fn activate(&mut self) -> Result<()> {
        if self.is_active {
            return Err(CydroidError::MissionError(
                "Policy is already active".to_string(),
            ));
        }
        self.is_active = true;
        Ok(())
    }

    /// Deactivate the policy
    pub fn deactivate(&mut self) -> Result<()> {
        self.is_active = false;
        Ok(())
    }

    /// Check if eco-score meets floor
    pub fn check_eco_floor(&self, current_score: f32) -> Result<bool> {
        if current_score < self.eco_floor {
            return Ok(false);
        }
        Ok(true)
    }

    /// Get adjusted safety constraints based on biophysical state
    pub fn get_adjusted_constraints(
        &self,
        fatigue_level: f32,
        stress_level: f32,
        cognitive_load: f32,
        pain_level: Option<f32>,
    ) -> SafetyConstraints {
        let mut constraints = self.safety_constraints.clone();

        // Apply fatigue modulation
        if fatigue_level > 0.7 {
            constraints.max_swarm_tempo = constraints
                .max_swarm_tempo
                .min(self.biophysical_triggers.high_fatigue_max_tempo);
        }

        // Apply stress modulation
        if stress_level > 0.7 {
            constraints.min_safety_radius_meters = constraints
                .min_safety_radius_meters
                .max(self.biophysical_triggers.high_stress_min_safety_radius);
        }

        // Apply cognitive load modulation
        if cognitive_load > 0.7 {
            constraints.max_intervention_depth = constraints
                .max_intervention_depth
                .min(self.biophysical_triggers.high_cognitive_load_max_depth);
        }

        // Apply pain-based pause
        if let Some(pain) = pain_level {
            if let Some(threshold) = self.biophysical_triggers.pain_pause_threshold {
                if pain > threshold {
                    constraints.max_swarm_tempo = 0.0; // Pause all operations
                }
            }
        }

        constraints
    }

    /// Check if policy is compatible with hazard conditions
    pub fn is_hazard_compatible(&self, current_hazard: HazardTag) -> bool {
        match (self.hazard_tag, current_hazard) {
            (HazardTag::None, _) => true,
            (HazardTag::Heatwave, HazardTag::Heatwave) => true,
            (HazardTag::FloodEvent, HazardTag::FloodEvent) => true,
            (HazardTag::WildfireSmoke, HazardTag::WildfireSmoke) => true,
            (HazardTag::SevereStorm, HazardTag::SevereStorm) => true,
            _ => false,
        }
    }
}

impl Validatable for SwarmPolicy {
    fn validate(&self) -> Result<()> {
        if self.policy_id.is_empty() {
            return Err(CydroidError::ValidationError(
                "policy_id cannot be empty".to_string(),
            ));
        }

        if self.eco_floor < 0.0 || self.eco_floor > 1.0 {
            return Err(CydroidError::ValidationError(format!(
                "eco_floor must be in [0.0, 1.0], got {}",
                self.eco_floor
            )));
        }

        // Enforce minimum eco-floor (0.86 default)
        if self.eco_floor < MINIMUM_ECOSCORE {
            return Err(CydroidError::EcoFloorNotMet {
                actual: self.eco_floor,
                required: MINIMUM_ECOSCORE,
            });
        }

        if self.safety_constraints.max_swarm_tempo < 0.0 {
            return Err(CydroidError::ValidationError(
                "max_swarm_tempo cannot be negative".to_string(),
            ));
        }

        if self.safety_constraints.min_safety_radius_meters < 0.0 {
            return Err(CydroidError::ValidationError(
                "min_safety_radius cannot be negative".to_string(),
            ));
        }

        // Validate evidence bundle IDs
        for bundle_id in &self.evidence_bundle_ids {
            crate::validate::helpers::validate_hex_10(bundle_id)?;
        }

        Ok(())
    }
}

/// Mission manifest (parameterized policy instance)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionManifest {
    /// Unique mission ID
    pub mission_id: Uuid,
    /// Corridor ID
    pub corridor_id: String,
    /// Policy ID
    pub policy_id: String,
    /// Required consent roles
    pub consent_requirements: Vec<String>,
    /// Mission start time
    pub start_time: DateTime<Utc>,
    /// Mission end time
    pub end_time: DateTime<Utc>,
    /// Biome tag
    pub biome_tag: BiomeTag,
    /// Hazard tag
    pub hazard_tag: HazardTag,
    /// Required eco-metrics
    pub required_eco_metrics: Vec<String>,
    /// ROW logging enabled
    pub row_logging_enabled: bool,
}

impl MissionManifest {
    /// Create a new mission manifest
    pub fn new(
        corridor_id: String,
        policy_id: String,
        consent_requirements: Vec<String>,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        biome_tag: BiomeTag,
        hazard_tag: HazardTag,
        required_eco_metrics: Vec<String>,
    ) -> Result<Self> {
        let manifest = Self {
            mission_id: Uuid::new_v4(),
            corridor_id,
            policy_id,
            consent_requirements,
            start_time,
            end_time,
            biome_tag,
            hazard_tag,
            required_eco_metrics,
            row_logging_enabled: true,
        };
        manifest.validate()?;
        Ok(manifest)
    }

    /// Check if mission duration is valid
    pub fn is_duration_valid(&self) -> bool {
        self.end_time > self.start_time
    }

    /// Get mission duration in hours
    pub fn duration_hours(&self) -> f64 {
        (self.end_time - self.start_time).num_seconds() as f64 / 3600.0
    }
}

impl Validatable for MissionManifest {
    fn validate(&self) -> Result<()> {
        if self.corridor_id.is_empty() {
            return Err(CydroidError::ValidationError(
                "corridor_id cannot be empty".to_string(),
            ));
        }

        if self.policy_id.is_empty() {
            return Err(CydroidError::ValidationError(
                "policy_id cannot be empty".to_string(),
            ));
        }

        if !self.is_duration_valid() {
            return Err(CydroidError::ValidationError(
                "end_time must be after start_time".to_string(),
            ));
        }

        if self.consent_requirements.is_empty() {
            return Err(CydroidError::ValidationError(
                "At least one consent requirement is required".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_policy_creation() {
        let policy = SwarmPolicy::new(
            "canal_clean_v1".to_string(),
            IntentClass::CanalClean,
            BiomeTag::UrbanCanal,
            HazardTag::None,
            SafetyConstraints::default(),
            0.86,
            BiophysicalTriggers::default(),
        );
        assert!(policy.is_ok());
        let policy = policy.unwrap();
        assert!(!policy.is_active);
    }

    #[test]
    fn test_eco_floor_enforcement() {
        let policy = SwarmPolicy::new(
            "test_policy".to_string(),
            IntentClass::CanalClean,
            BiomeTag::UrbanCanal,
            HazardTag::None,
            SafetyConstraints::default(),
            0.86,
            BiophysicalTriggers::default(),
        )
        .unwrap();

        assert!(policy.check_eco_floor(0.92).unwrap());
        assert!(!policy.check_eco_floor(0.75).unwrap());
    }

    #[test]
    fn test_biophysical_modulation() {
        let policy = SwarmPolicy::new(
            "test_policy".to_string(),
            IntentClass::CanalClean,
            BiomeTag::UrbanCanal,
            HazardTag::None,
            SafetyConstraints::default(),
            0.86,
            BiophysicalTriggers::default(),
        )
        .unwrap();

        let base_tempo = policy.safety_constraints.max_swarm_tempo;

        // High fatigue should reduce tempo
        let adjusted = policy.get_adjusted_constraints(0.9, 0.3, 0.3, None);
        assert!(adjusted.max_swarm_tempo < base_tempo);

        // High stress should increase safety radius
        let adjusted = policy.get_adjusted_constraints(0.3, 0.9, 0.3, None);
        assert!(adjusted.min_safety_radius_meters > policy.safety_constraints.min_safety_radius_meters);

        // High pain should pause operations
        let adjusted = policy.get_adjusted_constraints(0.3, 0.3, 0.3, Some(5.0));
        assert_eq!(adjusted.max_swarm_tempo, 0.0);
    }

    #[test]
    fn test_mission_manifest() {
        let now = Utc::now();
        let manifest = MissionManifest::new(
            "corridor_12".to_string(),
            "policy_v1".to_string(),
            vec!["operator".to_string()],
            now,
            now + Duration::hours(4),
            BiomeTag::UrbanCanal,
            HazardTag::None,
            vec!["turbidity".to_string(), "dissolved_oxygen".to_string()],
        );
        assert!(manifest.is_ok());
        let manifest = manifest.unwrap();
        assert_eq!(manifest.duration_hours(), 4.0);
    }

    #[test]
    fn test_eco_floor_minimum() {
        // Attempting to create policy with eco_floor below minimum should fail
        let policy = SwarmPolicy::new(
            "bad_policy".to_string(),
            IntentClass::CanalClean,
            BiomeTag::UrbanCanal,
            HazardTag::None,
            SafetyConstraints::default(),
            0.50, // Below MINIMUM_ECOSCORE (0.86)
            BiophysicalTriggers::default(),
        );
        assert!(policy.is_err());
    }
}
