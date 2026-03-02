//! Cydroid CLI Tool
//!
//! Introspection for ROW ledgers, ALN schemas, and consent states.

use cydroid_aln::{RowLedger, RowEntryType};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cydroid-cli <command> [args]");
        eprintln!("Commands: inspect-row, verify-consent, eco-stats");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "inspect-row" => {
            if args.len() < 3 {
                eprintln!("Usage: cydroid-cli inspect-row <row_file.json>");
                std::process::exit(1);
            }
            let data = fs::read_to_string(&args[2]).expect("Failed to read ROW file");
            let ledger: RowLedger = serde_json::from_str(&data).expect("Invalid ROW JSON");
            println!("ROW Ledger Length: {}", ledger.len());
            println!("Safety Violations: {}", ledger.get_safety_violations().len());
            println!("Consent Events: {}", ledger.get_consent_events().len());
        }
        "verify-consent" => {
            println!("Consent verification requires DID input (not implemented in CLI stub)");
        }
        "eco-stats" => {
            println!("Eco-stats requires mission data (not implemented in CLI stub)");
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            std::process::exit(1);
        }
    }
}
