use std::io::{self, BufRead};
use usehid::AgentHID;

use crate::output::print_result;

pub fn run(json: Option<String>, human: bool) {
    let mut hid = AgentHID::new();

    match json {
        Some(ref s) if s != "-" => {
            let result = hid.execute_json(s);
            print_result(&result, human);
        }
        _ => {
            // Read from stdin (NDJSON: one JSON per line)
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                let line = match line {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!("Error reading stdin: {}", e);
                        std::process::exit(1);
                    }
                };
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                let result = hid.execute_json(trimmed);
                print_result(&result, human);
            }
        }
    }
}
