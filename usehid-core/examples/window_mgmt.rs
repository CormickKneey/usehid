//! Example: Window management
//!
//! Switch windows, minimize, maximize, close

use usehid_core::AgentHID;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("🪟 Window Management Example");
    println!("============================");
    
    let mut agent = AgentHID::new();
    
    sleep(Duration::from_secs(2));
    
    // Switch between windows (Alt+Tab / Cmd+Tab)
    println!("🔄 Switching windows (Alt/Cmd+Tab)...");
    #[cfg(target_os = "macos")]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "tab"}"#);
    #[cfg(not(target_os = "macos"))]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["alt"], "key": "tab"}"#);
    sleep(Duration::from_secs(1));
    
    // Switch back
    println!("🔄 Switching back...");
    #[cfg(target_os = "macos")]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "tab"}"#);
    #[cfg(not(target_os = "macos"))]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["alt"], "key": "tab"}"#);
    sleep(Duration::from_secs(1));
    
    // Minimize window
    println!("➖ Minimizing window...");
    #[cfg(target_os = "macos")]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "m"}"#);
    #[cfg(target_os = "windows")]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["win"], "key": "down"}"#);
    sleep(Duration::from_secs(1));
    
    // Show desktop
    println!("🖥️ Showing desktop...");
    #[cfg(target_os = "macos")]
    agent.execute_json(r#"{"action": "key_press", "key": "f11"}"#);
    #[cfg(target_os = "windows")]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["win"], "key": "d"}"#);
    sleep(Duration::from_secs(1));
    
    // Mission Control / Task View
    println!("🚀 Opening Mission Control / Task View...");
    #[cfg(target_os = "macos")]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["ctrl"], "key": "up"}"#);
    #[cfg(target_os = "windows")]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["win"], "key": "tab"}"#);
    sleep(Duration::from_secs(2));
    
    // Exit
    agent.execute_json(r#"{"action": "key_press", "key": "escape"}"#);
    
    println!("\n✅ Window management demo complete!");
}
