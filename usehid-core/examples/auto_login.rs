//! Example: Auto-fill a login form
//!
//! Demonstrates using AgentHID to automatically fill login credentials.

use usehid::AgentHID;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("🔐 Auto-fill Login Form Example");
    println!("================================");
    println!("Focus on a login page username field in 3 seconds...");
    sleep(Duration::from_secs(3));
    
    let mut agent = AgentHID::new();
    
    // Type username
    println!("📝 Typing username...");
    agent.execute_json(r#"{"action": "type", "text": "demo@example.com"}"#);
    sleep(Duration::from_millis(300));
    
    // Tab to password field
    println!("➡️ Moving to password field...");
    agent.execute_json(r#"{"action": "key_press", "key": "tab"}"#);
    sleep(Duration::from_millis(200));
    
    // Type password
    println!("🔑 Typing password...");
    agent.execute_json(r#"{"action": "type", "text": "SecurePassword123!"}"#);
    sleep(Duration::from_millis(300));
    
    // Submit form (Enter)
    println!("✅ Submitting form...");
    agent.execute_json(r#"{"action": "key_press", "key": "enter"}"#);
    
    println!("\n🎉 Done! Form submitted.");
}
