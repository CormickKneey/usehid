use usehid::agent::AgentResult;

pub fn print_result(result: &AgentResult, human: bool) {
    if human {
        print_human(result);
    } else {
        print_json(result);
    }

    if !result.success {
        std::process::exit(1);
    }
}

fn print_json(result: &AgentResult) {
    println!("{}", serde_json::to_string(result).unwrap());
}

fn print_human(result: &AgentResult) {
    if !result.success {
        if let Some(ref err) = result.error {
            eprintln!("Error: {}", err);
        }
        return;
    }

    if let (Some(w), Some(h)) = (result.width, result.height) {
        println!("{}x{}", w, h);
        return;
    }

    if let (Some(x), Some(y)) = (result.x, result.y) {
        println!("{},{}", x, y);
        return;
    }

    if let (Some(enabled), Some(triggered)) = (result.enabled, result.triggered) {
        println!("enabled={} triggered={}", enabled, triggered);
        return;
    }

    println!("OK");
}
