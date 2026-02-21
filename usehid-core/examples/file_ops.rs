//! Example: File operations
//!
//! Common file operations: New, Save, Save As, Open

use usehid::AgentHID;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("📁 File Operations Example");
    println!("==========================");
    println!("Focus on a text editor (VS Code, Notepad, etc.) in 3 seconds...");
    sleep(Duration::from_secs(3));
    
    let mut agent = AgentHID::new();
    
    // New file (Ctrl+N / Cmd+N)
    println!("📄 Creating new file (Ctrl/Cmd+N)...");
    #[cfg(target_os = "macos")]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "n"}"#);
    #[cfg(not(target_os = "macos"))]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["ctrl"], "key": "n"}"#);
    sleep(Duration::from_millis(500));
    
    // Type some content
    println!("📝 Writing content...");
    agent.execute_json(r#"{"action": "type", "text": "My Document - This is auto-generated content."}"#);
    sleep(Duration::from_millis(300));
    
    // Save (Ctrl+S / Cmd+S)
    println!("💾 Saving file (Ctrl/Cmd+S)...");
    #[cfg(target_os = "macos")]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "s"}"#);
    #[cfg(not(target_os = "macos"))]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["ctrl"], "key": "s"}"#);
    sleep(Duration::from_millis(500));
    
    // Type filename (if save dialog appears)
    println!("📝 Typing filename...");
    agent.execute_json(r#"{"action": "type", "text": "my_document.md"}"#);
    sleep(Duration::from_millis(200));
    
    // Confirm save
    agent.execute_json(r#"{"action": "key_press", "key": "enter"}"#);
    
    println!("\n✅ File saved!");
    
    // Open file dialog (Ctrl+O / Cmd+O)
    sleep(Duration::from_secs(1));
    println!("\n📂 Opening file dialog (Ctrl/Cmd+O)...");
    #[cfg(target_os = "macos")]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "o"}"#);
    #[cfg(not(target_os = "macos"))]
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["ctrl"], "key": "o"}"#);
    
    println!("📂 File dialog opened!");
}
