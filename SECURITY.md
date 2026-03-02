# Cydroid Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

We take the security of Cydroid seriously, especially given its involvement with biophysical safety, neurorights, and soul guardrails.

### Scope

-   **In Scope:**
    -   Cydroid ALN Schema validation logic.
    -   Rust crates (`cydroid-aln`, `cydroid-pqc-signer`, etc.).
    -   ROW Ledger integrity mechanisms.
    -   Soul Guardrail enforcement modules.
    -   Cross-chain bridge specifications.

-   **Out of Scope:**
    -   Third-party dependencies (report to upstream).
    -   Physical hardware vulnerabilities (report to hardware vendor).
    -   Social engineering attacks.

### Process

1.  **Do Not Disclose Publicly:** Please do not create public GitHub issues for security vulnerabilities.
2.  **Email:** Send details to `security@cydroid.org` (PGP Key ID: `0xCYD1A2`).
3.  **Response Time:** We aim to acknowledge receipt within 48 hours and provide a preliminary assessment within 5 business days.
4.  **Resolution:** We follow a coordinated disclosure policy. Once a fix is available, we will notify you before public announcement.

### Security Best Practices for Contributors

-   **No Unsafe Code:** All Rust crates must deny `unsafe_code`.
-   **PQC Signing:** All governance transactions must use quantum-resistant signatures.
-   **Soul Guardrails:** Never modify code that bypasses `soul.guardrail.spec` checks.
-   **Audit Logs:** All security-sensitive actions must log to the ROW Ledger.

## Bug Bounty Program

We offer bounties for critical vulnerabilities affecting soul safety or ledger integrity. Bounty amounts are denominated in CHAT tokens (non-monetary) and Care Access Credits.

| Severity | Description | Reward |
| -------- | ----------- | ------ |
| Critical | Soul guardrail bypass, ROW ledger corruption | 10,000 CAC |
| High     | Governance threshold manipulation, PQC signature forgery | 5,000 CAC |
| Medium   | Data quality flag bypass, non-critical logic error | 1,000 CAC |
| Low      | Documentation errors, minor UI bugs | 100 CAC |

## Contact

-   **Email:** security@cydroid.org
-   **PGP Key:** [Download](https://cydroid.org/pgp/security.asc)
-   **DID:** `did:ion:CydroidSecurityTeam`
