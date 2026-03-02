# Contributing to Cydroid

Thank you for your interest in contributing to Cydroid. This project is built on principles of safety, verifiability, and neurorights protection.

## Code of Conduct

-   **Respect Soul Guardrails:** Never propose code that quantifies, transfers, or violates soul boundaries.
-   **Safety First:** All biophysical safety checks must remain enforced.
-   **Transparency:** All governance changes must be logged to the ROW Ledger.

## Getting Started

1.  **Fork the Repo:** Create a fork on GitHub.
2.  **Clone:** `git clone https://github.com/your-username/Cydroid.git`
3.  **Build:** `cargo build --release --workspace`
4.  **Test:** `cargo test --release --workspace`

## Development Guidelines

### Rust Code

-   **Edition:** Rust 2021.
-   **Safety:** `#![deny(unsafe_code)]` is mandatory.
-   **Formatting:** Use `rustfmt`.
-   **Linting:** Use `clippy`.
-   **Documentation:** All public APIs must have docstrings.

### ALN Particles

-   **Schema:** Follow `cydroid-spec.schema.yaml`.
-   **Validation:** All particles must validate against the schema.
-   **Cyberlinks:** Include appropriate cyberlinks to related particles.

### Testing

-   **Unit Tests:** Required for all new logic.
-   **Integration Tests:** Required for ROW ledger and governance flows.
-   **Safety Tests:** Specific tests for soul guardrail enforcement.

## Pull Request Process

1.  **Branch:** Create a feature branch (`feature/my-feature`).
2.  **Commit:** Sign commits with your DID-bound key.
3.  **Test:** Ensure all tests pass.
4.  **Review:** Request review from maintainers.
5.  **Merge:** Once approved, a maintainer will merge.

## Governance Contributions

Changes to governance thresholds or soul guardrails require a formal proposal via the `bostrom.governance.threshold.v1` particle and multi-sig approval from EthicsBoard and NeurodataController roles.

## License

By contributing, you agree that your contributions will be licensed under MIT OR Apache-2.0.
