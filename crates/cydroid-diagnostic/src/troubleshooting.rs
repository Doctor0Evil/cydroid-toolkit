//! Advanced Troubleshooting Module
//!
//! Provides diagnostic tools for Cydroid deployments, focusing on
//! Soul Guardrail violations, NEU budget exhaustion, and ROW ledger corruption.
//! Aligns with Cybercore-Brain stack diagnostic protocols.

#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]

use cydroid_aln::{RowLedger, RowEntry};
use cybercore_soul_guardrails::{SoulGuardrailAuditor, NeuBudget};

/// Diagnostic Result
#[derive(Debug)]
pub struct DiagnosticResult {
    pub status: Status,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
}

/// System Status
#[derive(Debug, PartialEq)]
pub enum Status {
    Healthy,
    Warning,
    Critical,
}

/// Diagnostic Engine
pub struct DiagnosticEngine {
    auditor: SoulGuardrailAuditor,
}

impl DiagnosticEngine {
    pub fn new() -> Self {
        Self {
            auditor: SoulGuardrailAuditor::new(),
        }
    }

    /// Run Full System Diagnostic
    pub fn run_full_diagnostic(&self, ledger: &RowLedger, neu_budget: &NeuBudget) -> DiagnosticResult {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        let mut status = Status::Healthy;

        // 1. Check ROW Ledger Integrity
        if !self.check_ledger_integrity(ledger) {
            issues.push("ROW Ledger integrity violation detected".to_string());
            recommendations.push("Restore from last known good backup".to_string());
            status = Status::Critical;
        }

        // 2. Check NEU Budget Status
        if neu_budget.is_exhausted() {
            issues.push("NEU Budget exhausted".to_string());
            recommendations.push("Initiate prosocial-clinical replenishment flow".to_string());
            status = Status::Critical;
        } else if neu_budget.is_low() {
            issues.push("NEU Budget low".to_string());
            recommendations.push("Reduce high-risk operations".to_string());
            status = Status::Warning;
        }

        // 3. Check Soul Guardrail Compliance
        let audit = self.auditor.audit_ledger(ledger).unwrap();
        if !audit.passed {
            issues.push(format!("Soul Guardrail violations: {:?}", audit.violations));
            recommendations.push("Rollback to pre-violation state".to_string());
            status = Status::Critical;
        }

        DiagnosticResult {
            status,
            issues,
            recommendations,
        }
    }

    /// Check ROW Ledger Integrity
    fn check_ledger_integrity(&self, ledger: &RowLedger) -> bool {
        // Verify chain links
        let entries = ledger.get_all_entries();
        for i in 1..entries.len() {
            let prev = &entries[i - 1];
            let curr = &entries[i];
            if curr.previous_entry_hash != prev.compute_hash().unwrap() {
                return false;
            }
        }
        true
    }
}

impl Default for DiagnosticEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_healthy() {
        let engine = DiagnosticEngine::new();
        let ledger = RowLedger::new(); // Empty ledger is valid
        let neu_budget = NeuBudget::new(100.0);
        let result = engine.run_full_diagnostic(&ledger, &neu_budget);
        assert_eq!(result.status, Status::Healthy);
    }
}
