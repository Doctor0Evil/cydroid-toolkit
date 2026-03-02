# Cydroid Toolkit Release Manifest v1.0.0

**Release Date:** 2026-03-02  
**Hex-Stamp:** `0xCYD6F1`  
**Knowledge-Factor:** `F ≈ 0.97`  
**Governance:** Bostrom/ALN/DID Forward-Only Evolution  
**Safety Envelope:** `soul.guardrail.spec.v1` + `bio.safety.envelope.citizen.v1`

## Overview

This release marks the completion of the Core Cydroid Toolkit (Batches 1-6), providing a production-grade, cryptographically auditable framework for integrating neuromorphic sensing, robot swarm control, and ecological restoration. It is fully aligned with the Cybercore-Brain stack, adhering to strict neurorights constraints, soul-boundary protections, and non-monetary incentive structures.

## Included Components

1.  **ALN Schema Layer:** Canonical definitions for neurochannels, eco-indicators, ROW ledger entries, Soul Guardrails, and NEU Budgets.
2.  **Neuromorphic Edge:** Mojo/Rust kernels for asynchronous delta modulation, spike encoding, and CyberRank matrix calculations.
3.  **Swarm Policy:** Lua-based orchestration enforcing eco-floors (≥0.86) and biophysical safety.
4.  **ROW Ledger:** Append-only, DID-anchored audit trail for all actions, consent, and evidence.
5.  **Soul Guardrails:** Hard-coded constraints preventing soul quantification, ownership transfer, or neuroright violations.
6.  **NEU Budgets:** Psych-risk gating for high-risk operations, with deterministic rollback on exhaustion.
7.  **Human Interfaces:** Kotlin Android Operator UI, JavaScript Expert Dashboard with Soul/NEU visualizers.
8.  **Deployment:** Rust orchestrators, CI worklines, security audits, and integration tests.

## Security & Compliance

-   **Soul Non-Quantification:** No mechanism exists to score, trade, or collateralize a soul. Karma attaches only to actions/particles.
-   **NEU Psych-Risk Budgets:** Bounded monotonic counters gate high-risk operations. Exhaustion triggers deterministic rollback.
-   **Forward-Only Governance:** Schema evolution requires multisig approval; no rollbacks on ledger history.
-   **Ecological Floor:** Missions must maintain `ecoscore ≥ 0.86` to continue operation.
-   **Privacy:** Biophysical data is summarized; raw EEG/EMG is never exposed to UI (Neurorights Compliance).

## Build & Deployment

```bash
# Build all crates
cargo build --release --workspace

# Run integration tests
cargo test --release --workspace

# Deploy mission manifest
cargo run --release -p cydroid-deploy -- --manifest mission_manifest.json --did did:ion:...
