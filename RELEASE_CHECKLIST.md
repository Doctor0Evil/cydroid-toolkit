# Cydroid Release Checklist

## Pre-Release

-   [ ] **Version Bump:** Update `Cargo.toml` and ALN particle versions.
-   [ ] **Changelog:** Update `CHANGELOG.md` with all changes.
-   [ ] **Security Audit:** Run `cargo audit` and review dependencies.
-   [ ] **Soul Guardrail Check:** Verify all soul guardrail tests pass.
-   [ ] **ROW Ledger Test:** Verify ledger integrity tests pass.
-   [ ] **PQC Signer Test:** Verify quantum-resistant signing works.
-   [ ] **Documentation:** Ensure all docs are up-to-date.

## Release

-   [ ] **Tag:** Create Git tag (`v1.0.0`).
-   [ ] **Publish:** Publish crates to `crates.io` (if public).
-   [ ] **Particles:** Publish ALN particles to registry.
-   [ ] **Binaries:** Build and sign release binaries.
-   [ ] **Announcement:** Post release announcement to community channels.

## Post-Release

-   [ ] **Monitor:** Watch for crash reports or security issues.
-   [ ] **Patch:** Prepare patch releases for critical bugs.
-   [ ] **Retrospective:** Conduct release retrospective.

## Governance Approval

-   [ ] **EthicsBoard:** Co-signature obtained for release.
-   [ ] **NeurodataController:** Co-signature obtained for release.
-   [ ] **ROW Log:** Release approval logged to ROW Ledger.
