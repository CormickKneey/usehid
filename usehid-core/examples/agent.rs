//! Example: Agent HID
//!
//! This example demonstrates the JSON-based interface for LLM agents.

use usehid_core::AgentHID;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Creating AgentHID controller...");
    println!("You have 3 seconds to focus a window...");
    sleep(Duration::from_secs(3));
    
    let mut agent = AgentHID::new();
    
    // Mouse move
    println!("Moving mouse...");
    let result = agent.execute_json(r#"{"action": "mouse_move", "x": 100, "y": 50}"#);
    println!("Result: {:?}", result);
    sleep(Duration::from_millis(500));
    
    // Mouse click
    println!("Clicking...");
    let result = agent.execute_json(r#"{"action": "mouse_click", "button": "left"}"#);
    println!("Result: {:?}", result);
    sleep(Duration::from_millis(500));
    
    // Type text
    println!("Typing...");
    let result = agent.execute_json(r#"{"action": "type", "text": "Hello from AI Agent!"}"#);
    println!("Result: {:?}", result);
    sleep(Duration::from_millis(500));
    
    // Key press
    println!("Pressing Enter...");
    let result = agent.execute_json(r#"{"action": "key_press", "key": "enter"}"#);
    println!("Result: {:?}", result);
    sleep(Duration::from_millis(500));
    
    // Key combo
    println!("Pressing Ctrl+S...");
    let result = agent.execute_json(r#"{"action": "key_combo", "modifiers": ["ctrl"], "key": "s"}"#);
    println!("Result: {:?}", result);
    
    // Scroll
    println!("Scrolling...");
    let result = agent.execute_json(r#"{"action": "mouse_scroll", "delta": -5}"#);
    println!("Result: {:?}", result);
    
    println!("Done!");
}
