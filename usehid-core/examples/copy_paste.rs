//! Example: Copy and Paste text
//!
//! Demonstrates clipboard operations using keyboard shortcuts.

use usehid::AgentHID;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("📋 Copy & Paste Example");
    println!("========================");
    println!("Focus on a text editor in 3 seconds...");
    sleep(Duration::from_secs(3));
    
    let mut agent = AgentHID::new();
    
    // Type some text
    println!("📝 Typing text...");
    agent.execute_json(r#"{"action": "type", "text": "Hello, this text will be copied!"}"#);
    sleep(Duration::from_millis(300));
    
    // Select all (Ctrl+A / Cmd+A)
    println!("🔤 Selecting all (Ctrl/Cmd+A)...");
    #[cfg(target_os = "macos")]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "a"}"#);
    #[cfg(not(target_os = "macos"))]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["ctrl"], "key": "a"}"#);
    sleep(Duration::from_millis(200));
    
    // Copy (Ctrl+C / Cmd+C)
    println!("📋 Copying (Ctrl/Cmd+C)...");
    #[cfg(target_os = "macos")]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "c"}"#);
    #[cfg(not(target_os = "macos"))]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["ctrl"], "key": "c"}"#);
    sleep(Duration::from_millis(200));
    
    // Move to end
    println!("➡️ Moving to end...");
    agent.execute_json(r#"{"action": "key_press", "key": "end"}"#);
    sleep(Duration::from_millis(100));
    
    // New line
    agent.execute_json(r#"{"action": "key_press", "key": "enter"}"#);
    agent.execute_json(r#"{"action": "key_press", "key": "enter"}"#);
    sleep(Duration::from_millis(100));
    
    // Paste (Ctrl+V / Cmd+V)
    println!("📄 Pasting (Ctrl/Cmd+V)...");
    #[cfg(target_os = "macos")]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "v"}"#);
    #[cfg(not(target_os = "macos"))]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["ctrl"], "key": "v"}"#);
    
    println!("\n✅ Text copied and pasted!");
}
