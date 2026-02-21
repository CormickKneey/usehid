//! Example: Web browsing shortcuts
//!
//! Common browser shortcuts for navigation

use usehid::AgentHID;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("🌐 Browser Shortcuts Example");
    println!("============================");
    println!("Make sure a browser is focused!");
    sleep(Duration::from_secs(2));
    
    let mut agent = AgentHID::new();
    
    // New tab
    println!("➕ Opening new tab (Ctrl/Cmd+T)...");
    #[cfg(target_os = "macos")]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "t"}"#);
    #[cfg(not(target_os = "macos"))]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["ctrl"], "key": "t"}"#);
    sleep(Duration::from_millis(500));
    
    // Navigate to URL
    println!("🔗 Navigating to github.com...");
    agent.execute_json(r#"{"action": "type", "text": "github.com"}"#);
    agent.execute_json(r#"{"action": "key_press", "key": "enter"}"#);
    sleep(Duration::from_secs(2));
    
    // Scroll down
    println!("⬇️ Scrolling down...");
    for _ in 0..3 {
        agent.execute_json(r#"{"action": "mouse_scroll", "delta": -3}"#);
        sleep(Duration::from_millis(200));
    }
    sleep(Duration::from_millis(500));
    
    // Back
    println!("⬅️ Going back (Alt/Cmd+Left)...");
    #[cfg(target_os = "macos")]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "left"}"#);
    #[cfg(not(target_os = "macos"))]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["alt"], "key": "left"}"#);
    sleep(Duration::from_secs(1));
    
    // Forward
    println!("➡️ Going forward (Alt/Cmd+Right)...");
    #[cfg(target_os = "macos")]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "right"}"#);
    #[cfg(not(target_os = "macos"))]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["alt"], "key": "right"}"#);
    sleep(Duration::from_secs(1));
    
    // Refresh
    println!("🔄 Refreshing page (Ctrl/Cmd+R)...");
    #[cfg(target_os = "macos")]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "r"}"#);
    #[cfg(not(target_os = "macos"))]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["ctrl"], "key": "r"}"#);
    sleep(Duration::from_secs(1));
    
    // Find in page
    println!("🔍 Opening find (Ctrl/Cmd+F)...");
    #[cfg(target_os = "macos")]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "f"}"#);
    #[cfg(not(target_os = "macos"))]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["ctrl"], "key": "f"}"#);
    sleep(Duration::from_millis(300));
    agent.execute_json(r#"{"action": "type", "text": "repository"}"#);
    sleep(Duration::from_millis(500));
    agent.execute_json(r#"{"action": "key_press", "key": "escape"}"#);
    
    // Close tab
    println!("❌ Closing tab (Ctrl/Cmd+W)...");
    #[cfg(target_os = "macos")]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "w"}"#);
    #[cfg(not(target_os = "macos"))]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["ctrl"], "key": "w"}"#);
    
    println!("\n✅ Browser shortcuts demo complete!");
}
