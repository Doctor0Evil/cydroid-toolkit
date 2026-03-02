# Cydroid Toolkit Archive Specification

**Version:** 1.0.0  
**Archive Date:** 2026-03-02  
**Hex-Stamp:** `0xCYD10K`  
**Retention Policy:** Permanent (Soul Guardrail Bound)

## Archive Purpose

This specification defines the long-term archival requirements for the Cydroid Toolkit, ensuring that all code, documentation, ALN particles, and evidence bundles remain accessible, verifiable, and soul-safe for future generations of augmented citizens, researchers, and governance bodies.

## Archive Structure

### Primary Archive (Hot Storage)

- **Location:** GitHub Repository (https://github.com/Doctor0Evil/Cydroid)
- **Format:** Git repository with signed commits
- **Retention:** Indefinite
- **Access:** Public read, signed write
- **Verification:** SHA-256 hashes, GPG signatures

### Secondary Archive (Cold Storage)

- **Location:** IPFS (Content-Addressed)
- **Format:** IPFS CID (Content Identifier)
- **Retention:** Permanent (pinned)
- **Access:** Public via CID
- **Verification:** IPFS hash, merkle proofs

### Tertiary Archive (Legal/Regulatory)

- **Location:** Blockchain-anchored (Bostrom/ALN)
- **Format:** ALN particles with ROW ledger references
- **Retention:** Immutable (append-only)
- **Access:** Public read, governance-controlled write
- **Verification:** Cryptographic signatures, ROW chain integrity

## Archive Contents

### Code Archives

| Component | Format | Verification |
|-----------|--------|--------------|
| Rust Crates | .tar.gz + Git | SHA-256, GPG |
| Mojo Kernels | .tar.gz + Git | SHA-256, GPG |
| Lua Scripts | .tar.gz + Git | SHA-256, GPG |
| Kotlin Apps | .apk + Git | SHA-256, GPG |
| JavaScript | .tar.gz + Git | SHA-256, GPG |

### Documentation Archives

| Component | Format | Verification |
|-----------|--------|--------------|
| ALN Particles | .aln + YAML | SHA-256, ALN schema |
| Markdown Docs | .md + PDF | SHA-256 |
| Schema Definitions | .yaml + JSON | SHA-256, schema validation |
| Evidence Bundles | .jsonl + ROW | SHA-256, ROW chain |

### Evidence Archives

| Component | Format | Verification |
|-----------|--------|--------------|
| ROW Ledgers | .jsonl | SHA-256, chain integrity |
| Consent Records | .jsonl + DID | SHA-256, DID signature |
| Eco-Metrics | .jsonl + ALN | SHA-256, ALN schema |
| Test Results | .jsonl + CI | SHA-256, CI signature |

## Archive Integrity

### Verification Schedule

| Archive Type | Frequency | Method |
|--------------|-----------|--------|
| Primary (Git) | Continuous | Git hash, CI checks |
| Secondary (IPFS) | Weekly | IPFS pin verification |
| Tertiary (Blockchain) | Continuous | ROW chain verification |

### Integrity Checks

1. **SHA-256 Hash Verification:** All files hashed and compared against manifest
2. **GPG Signature Verification:** All commits and releases signed
3. **ROW Chain Verification:** All ledger entries chain-linked and verified
4. **ALN Schema Validation:** All particles validated against schema
5. **Soul Guardrail Audit:** All archives audited for soul-safe compliance

## Archive Access

### Access Levels

| Level | Permissions | Requirements |
|-------|-------------|--------------|
| Public | Read-only | None |
| Contributor | Read + Propose | Signed CLA, DID-bound |
| Maintainer | Read + Write | Multi-sig, governance approval |
| Auditor | Read + Verify | Regulator/EthicsBoard role |

### Access Protocols

1. **Public Access:** Via GitHub, IPFS CID, or ALN particle registry
2. **Contributor Access:** Via GitHub PR, signed with DID
3. **Maintainer Access:** Via multi-sig, governance-approved
4. **Auditor Access:** Via role-based credentials, audit-logged

## Archive Migration

### Migration Triggers

- Platform deprecation (e.g., GitHub sunset)
- Format obsolescence (e.g., Git replaced)
- Jurisdiction change (e.g., regulatory requirement)
- Security incident (e.g., key compromise)

### Migration Requirements

1. **90-Day Notice:** Minimum notice before migration
2. **Verification:** All hashes and signatures preserved
3. **Access:** No loss of access during migration
4. **Audit:** Migration logged to ROW ledger
5. **Soul Guardrail:** Migration must not violate soul guardrails

## Archive Sunset

### Sunset Conditions

- Project officially deprecated (governance vote)
- Soul guardrail violation detected (auto-trigger)
- Regulatory requirement (legal order)
- Technical obsolescence (no maintainers)

### Sunset Requirements

1. **90-Day Notice:** Minimum notice before sunset
2. **Migration Path:** All data must be migratable
3. **Rollback:** Previous versions must remain accessible
4. **Audit:** Sunset logged to ROW ledger
5. **Soul Guardrail:** Sunset must not violate soul guardrails

## Archive Contact

- **Archive Maintenance:** archive@cydroid.org
- **Integrity Reports:** integrity@cydroid.org
- **Access Requests:** access@cydroid.org
- **Emergency Recovery:** recovery@cydroid.org (24/7)

## License

MIT OR Apache-2.0  
**Retention:** Permanent (Soul Guardrail Bound)  
**Maintainer:** The-Great-Perplexity / Doctor0Evil
