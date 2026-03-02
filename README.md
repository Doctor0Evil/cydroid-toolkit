# Cydroid Toolkit: Verifiable Human-Robotics & Ecological Restoration

**Version:** 1.0.0  
**Schema:** ALN v1.0 (`cydroid-spec.schema.yaml`)  
**Governance:** Bostrom/ALN/DID Forward-Only Evolution  
**Safety Envelope:** `bio.safety.envelope.citizen.v1` + `soul.guardrail.spec.v1`

## Overview

The Cydroid Toolkit provides a production-grade, cryptographically auditable framework for integrating neuromorphic sensing, robot swarm control, and ecological restoration. It is designed to operate within the **Cybercore-Brain** ecosystem, adhering to strict neurorights constraints, soul-boundary protections, and non-monetary incentive structures.

## Core Architecture

1.  **ALN Schema Layer:** Canonical definitions for neurochannels, eco-indicators, and ROW ledger entries.
2.  **Neuromorphic Edge:** Mojo/Rust kernels for asynchronous delta modulation and spike encoding.
3.  **Swarm Policy:** Lua-based orchestration enforcing eco-floors (≥0.86) and biophysical safety.
4.  **ROW Ledger:** Append-only, DID-anchored audit trail for all actions, consent, and evidence.
5.  **Soul Guardrails:** Hard-coded constraints preventing soul quantification, ownership transfer, or neuroright violations.

## Safety & Compliance

-   **Soul Non-Quantification:** No mechanism exists to score, trade, or collateralize a soul. Karma attaches only to actions/particles (`karma.metric.spec`), never to persons.
-   **NEU Psych-Risk Budgets:** Bounded monotonic counters gate high-risk operations. Exhaustion triggers deterministic rollback, not soul penalty.
-   **Forward-Only Governance:** Schema evolution requires multisig approval; no rollbacks on ledger history.
-   **Ecological Floor:** Missions must maintain `ecoscore ≥ 0.86` to continue operation.

## Getting Started

### Prerequisites

-   Rust 1.75+ (Edition 2021)
-   Mojo SDK (for neuromorphic kernels)
-   Lua 5.4+ (for swarm policy bus)
-   DID Key Pair (Bostrom/ION compatible)

### Build Instructions

```bash
# Clone the repository
git clone https://github.com/Doctor0Evil/Cydroid.git
cd Cydroid/cydroid-toolkit

# Build the core ALN crate
cargo build --release -p cydroid-aln

# Run integration tests (verifies soul guardrails)
cargo test --release -p cydroid-aln --test integration

# Deploy a mission manifest (requires DID signing)
cargo run --release -p cydroid-deploy -- --manifest mission_manifest.json --did did:ion:...

Security & Auditing
All code paths touching augmentation, neuromodulation, or identity are wrapped with #[soul_guarded] macros. The cydroid-security crate performs static analysis to ensure no particle violates soul.guardrail.spec.v1.
License
MIT OR Apache-2.0
Jurisdiction: US-AZ-Maricopa-Phoenix (Global Node Placement Allowed via The Globe)

