//! Example: Keyboard typing with intervals and combos
//!
//! Demonstrates typing, key combinations, and the interval feature.
//!
//! Run with: cargo run --release -p usehid --example keyboard

use usehid::{AgentHID, Keyboard, Key, Modifiers, Device};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("=== useHID Keyboard Demo ===\n");
    println!("Focus a text editor within 3 seconds...\n");
    sleep(Duration::from_secs(3));
    
    let mut agent = AgentHID::new();
    
    // ========================================
    // 1. Basic Typing
    // ========================================
    println!("--- 1. Basic Typing ---");
    
    println!("Typing 'Hello, World!' (instant)...");
    let result = agent.execute_json(r#"{"action": "type", "text": "Hello, World!"}"#);
    println!("Result: {:?}\n", result);
    sleep(Duration::from_millis(500));
    
    // Press Enter
    agent.execute_json(r#"{"action": "key_press", "key": "enter"}"#);
    sleep(Duration::from_millis(200));
    
    // ========================================
    // 2. Typing with Interval (Typewriter)
    // ========================================
    println!("--- 2. Typewriter Effect ---");
    
    println!("Typing with 50ms interval...");
    let result = agent.execute_json(
        r#"{"action": "type", "text": "This is typed slowly...", "interval": 50}"#
    );
    println!("Result: {:?}\n", result);
    
    agent.execute_json(r#"{"action": "key_press", "key": "enter"}"#);
    sleep(Duration::from_millis(200));
    
    // Slower typing
    println!("Typing with 150ms interval...");
    agent.execute_json(
        r#"{"action": "type", "text": "Even slower...", "interval": 150}"#
    );
    
    agent.execute_json(r#"{"action": "key_press", "key": "enter"}"#);
    sleep(Duration::from_millis(300));
    
    // ========================================
    // 3. Key Combinations
    // ========================================
    println!("\n--- 3. Key Combinations ---");
    
    // Type some text to select
    agent.execute_json(r#"{"action": "type", "text": "Select this text"}"#);
    sleep(Duration::from_millis(200));
    
    // Cmd+A (Select All)
    println!("Pressing Cmd+A (Select All)...");
    let result = agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "a"}"#);
    println!("Result: {:?}", result);
    sleep(Duration::from_millis(500));
    
    // Cmd+C (Copy)
    println!("Pressing Cmd+C (Copy)...");
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "c"}"#);
    sleep(Duration::from_millis(300));
    
    // Move to end
    agent.execute_json(r#"{"action": "key_press", "key": "right"}"#);
    agent.execute_json(r#"{"action": "key_press", "key": "enter"}"#);
    
    // Cmd+V (Paste)
    println!("Pressing Cmd+V (Paste)...");
    agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "v"}"#);
    sleep(Duration::from_millis(300));
    
    agent.execute_json(r#"{"action": "key_press", "key": "enter"}"#);
    
    // ========================================
    // 4. Multiple Modifiers
    // ========================================
    println!("\n--- 4. Multiple Modifiers ---");
    
    // Cmd+Shift+Z (Redo on some apps)
    println!("Pressing Cmd+Shift+Z (Redo)...");
    let result = agent.execute_json(
        r#"{"action": "key_combo", "modifiers": ["cmd", "shift"], "key": "z"}"#
    );
    println!("Result: {:?}", result);
    sleep(Duration::from_millis(300));
    
    // ========================================
    // 5. Special Keys
    // ========================================
    println!("\n--- 5. Special Keys ---");
    
    println!("Arrow keys...");
    agent.execute_json(r#"{"action": "key_press", "key": "up"}"#);
    sleep(Duration::from_millis(100));
    agent.execute_json(r#"{"action": "key_press", "key": "down"}"#);
    sleep(Duration::from_millis(100));
    agent.execute_json(r#"{"action": "key_press", "key": "left"}"#);
    sleep(Duration::from_millis(100));
    agent.execute_json(r#"{"action": "key_press", "key": "right"}"#);
    sleep(Duration::from_millis(100));
    
    println!("Tab...");
    agent.execute_json(r#"{"action": "key_press", "key": "tab"}"#);
    sleep(Duration::from_millis(100));
    
    println!("Backspace (x3)...");
    for _ in 0..3 {
        agent.execute_json(r#"{"action": "key_press", "key": "backspace"}"#);
        sleep(Duration::from_millis(50));
    }
    
    // ========================================
    // 6. Key Down/Up (Hold)
    // ========================================
    println!("\n--- 6. Key Down/Up (Hold) ---");
    
    println!("Holding Shift and typing...");
    agent.execute_json(r#"{"action": "key_down", "key": "shift"}"#);
    sleep(Duration::from_millis(50));
    
    agent.execute_json(r#"{"action": "type", "text": "uppercase"}"#);
    
    agent.execute_json(r#"{"action": "key_up", "key": "shift"}"#);
    sleep(Duration::from_millis(50));
    
    agent.execute_json(r#"{"action": "type", "text": " lowercase"}"#);
    
    // ========================================
    // 7. Using Direct API
    // ========================================
    println!("\n--- 7. Direct Keyboard API ---");
    
    let mut keyboard = Keyboard::new();
    if let Err(e) = keyboard.create() {
        println!("Note: Direct API requires create(): {}", e);
    } else {
        println!("Using direct Keyboard API...");
        
        keyboard.type_text("\nDirect API test").ok();
        sleep(Duration::from_millis(200));
        
        keyboard.tap(Key::Enter).ok();
        
        // Combo
        keyboard.press_combo(Modifiers::CMD, Key::A).ok();
        sleep(Duration::from_millis(100));
        
        // Type single char
        keyboard.type_char('!').ok();
        
        keyboard.destroy().ok();
    }
    
    println!("\n=== Demo Complete ===");
}
