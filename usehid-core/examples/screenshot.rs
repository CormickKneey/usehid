//! Example: Screenshot and save
//!
//! Takes a screenshot using system shortcuts.

use usehid::AgentHID;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("📸 Screenshot Example");
    println!("=====================");
    
    let mut agent = AgentHID::new();
    
    sleep(Duration::from_secs(2));
    
    // macOS: Cmd+Shift+3 for full screen screenshot
    // Windows: Win+PrintScreen
    // Linux: PrintScreen (depends on DE)
    
    #[cfg(target_os = "macos")]
    {
        println!("Taking full screen screenshot (Cmd+Shift+3)...");
        agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd", "shift"], "key": "3"}"#);
    }
    
    #[cfg(target_os = "windows")]
    {
        println!("Taking screenshot (Win+PrintScreen)...");
        agent.execute_json(r#"{"action": "key_combo", "modifiers": ["win"], "key": "printscreen"}"#);
    }
    
    #[cfg(target_os = "linux")]
    {
        println!("Taking screenshot (PrintScreen)...");
        agent.execute_json(r#"{"action": "key_press", "key": "printscreen"}"#);
    }
    
    sleep(Duration::from_millis(500));
    
    println!("✅ Screenshot saved!");
    
    // macOS: Cmd+Shift+4 for selection screenshot
    #[cfg(target_os = "macos")]
    {
        sleep(Duration::from_secs(1));
        println!("\nTaking selection screenshot (Cmd+Shift+4)...");
        println!("Draw a rectangle to capture...");
        agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd", "shift"], "key": "4"}"#);
    }
}
