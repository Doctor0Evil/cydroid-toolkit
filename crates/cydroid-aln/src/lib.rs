//! # Cydroid ALN Schema Bindings
//!
//! This crate provides the canonical ALN schema bindings for the Cydroid ecosystem,
//! enabling cross-language interoperability between Rust, Kotlin, Lua, JavaScript, and Mojo.
//!
//! ## Core Components
//!
//! - **NeuroChannel**: Biophysical sensing modalities with safety limits and calibration metadata
//! - **NeuromorphicEvent**: Event frames for spike trains and environmental snapshots
//! - **EcoIndicator**: Strongly-typed ecological metrics with units and uncertainty
//! - **RowEntry**: Append-only ledger entries with DID-anchored signatures
//! - **ConsentRecord**: Formally Presented Statements (FPS) bound to participant DIDs
//! - **SwarmPolicy**: Policy states for robot swarm control with biophysical triggers
//! - **MissionManifest**: Parameterized mission templates with biome/hazard tags
//! - **EvidenceBundle**: Verifiable evidence chains for audit and governance
//! - **CareAccessCredit**: Non-monetary incentives for ecological contribution
//!
//! ## Safety Guarantees
//!
//! - All safety limits are encoded as machine-enforced predicates
//! - No magic constants; all thresholds come from ALN schema
//! - Forward-only schema evolution with multisig governance
//! - Consent is always revocable and logged to ROW
//! - Eco-floor enforcement (minimum ecoscore ≥ 0.86)
//!
//! ## Example
//!
//! ```rust
//! use cydroid_aln::{NeuroChannel, ModalityType, CalibrationMetadata};
//! use chrono::Utc;
//!
//! let channel = NeuroChannel {
//!     channel_id: "EEG_alpha_frontal".to_string(),
//!     modality_type: ModalityType::EEG,
//!     unit: "µV".to_string(),
//!     calibration_metadata: CalibrationMetadata {
//!         calibration_date: Utc::now(),
//!         calibration_method: "NIST_traceable".to_string(),
//!         uncertainty_estimate: 0.02,
//!     },
//!     safety_limits: SafetyLimits {
//!         max_value: 100.0,
//!         min_value: -100.0,
//!         max_current_density_uA_per_cm2: 0.5,
//!         max_skin_contact_duration_minutes: Some(120),
//!         max_device_temperature_celsius: Some(40.0),
//!     },
//!     semantic_uri: Some("https://sweet.jpl.nasa.gov/2.3/phenomena.owl#Electroencephalography".to_string()),
//! };
//!
//! assert!(channel.validate().is_ok());
//! ```

#![deny(rust_2018_idioms)]
#![deny(missing_debug_implementations)]
#![deny(unsafe_code)]
#![warn(missing_docs)]

mod neurochannel;
mod neuromorphic_event;
mod eco_indicator;
mod row_ledger;
mod consent;
mod swarm_policy;
mod mission_manifest;
mod evidence_bundle;
mod care_credit;
mod error;
mod validate;

pub use neurochannel::*;
pub use neuromorphic_event::*;
pub use eco_indicator::*;
pub use row_ledger::*;
pub use consent::*;
pub use swarm_policy::*;
pub use mission_manifest::*;
pub use evidence_bundle::*;
pub use care_credit::*;
pub use error::*;
pub use validate::*;

/// Schema version for this release
pub const SCHEMA_VERSION: &str = "1.0.0";

/// Schema ID for cross-reference
pub const SCHEMA_ID: &str = "cydroid.canonical.schema.v1";

/// Minimum ecoscore required for continued mission operation
pub const MINIMUM_ECOSCORE: f32 = 0.86;

/// Re-export serde for convenience
#[cfg(feature = "serde")]
pub use serde;

/// Re-export chrono for timestamp handling
pub use chrono;

/// Re-export uuid for identifier generation
pub use uuid;
