use usehid::agent::AgentAction;
use usehid::AgentHID;

use crate::output::print_result;

pub fn run(file: &str, delay: Option<u64>, human: bool) {
    let content = match std::fs::read_to_string(file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file, e);
            std::process::exit(1);
        }
    };

    let actions: Vec<AgentAction> = match serde_json::from_str(&content) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error parsing JSON from '{}': {}", file, e);
            std::process::exit(1);
        }
    };

    let mut hid = AgentHID::new();
    let delay_duration = delay.map(std::time::Duration::from_millis);
    let total = actions.len();

    for (i, action) in actions.into_iter().enumerate() {
        let result = hid.execute(action);
        print_result(&result, human);

        if !result.success {
            std::process::exit(1);
        }

        if let Some(d) = delay_duration {
            if i + 1 < total {
                std::thread::sleep(d);
            }
        }
    }
}
