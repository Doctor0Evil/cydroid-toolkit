# Cydroid Toolkit Final Release Notes

**Version:** 1.0.0  
**Date:** 2026-03-02  
**Hex-Stamp:** `0xCYD8F4`

## Highlights

-   **Soul-Safe Deployment:** All deployment scripts now verify Soul Guardrails and NEU budgets before execution.
-   **CI/CD Integration:** GitHub Actions workflows include Soul Guardrail audits and NEU budget checks.
-   **Security Audit Particle:** New ALN particle `cydroid.security.audit.v1` defines security standards.
-   **Integration Tests:** Comprehensive test suite verifies soul non-quantification and rollback pathways.
-   **Project Closure:** All core components are complete and documented.

## Breaking Changes

-   **NEU Budget Enforcement:** Missions now fail if NEU budget is insufficient (previously warned).
-   **Soul Guardrail Audit:** Deployments now block on Soul Guardrail violations (previously logged).

## Known Issues

-   None. All known issues have been resolved in this release.

## Contributors

-   The-Great-Perplexity
-   Doctor0Evil
-   Cybercore-Brain Community

## Support

-   **Documentation:** `cydroid-toolkit/README.md`
-   **Security:** `cydroid-toolkit/SECURITY.md`
-   **Issues:** GitHub Issues (Soul-Safe Label Required)

## License

MIT OR Apache-2.0
