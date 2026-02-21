//! Example: Agent HID - Complete API Demo
//!
//! This example demonstrates all Agent API capabilities including
//! new features: screen queries, smooth movement, failsafe, and more.
//!
//! Run with: cargo run --release -p usehid --example agent

use usehid::AgentHID;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("=== useHID Agent API Demo ===\n");
    
    let mut agent = AgentHID::new();
    
    // ========================================
    // 1. Screen Query Actions
    // ========================================
    println!("--- 1. Screen Query ---");
    
    // Get screen size
    let result = agent.execute_json(r#"{"action": "size"}"#);
    println!("Screen size: {:?}", result);
    
    // Get mouse position
    let result = agent.execute_json(r#"{"action": "position"}"#);
    println!("Mouse position: {:?}", result);
    
    // ========================================
    // 2. Failsafe Status
    // ========================================
    println!("\n--- 2. Failsafe ---");
    
    let result = agent.execute_json(r#"{"action": "failsafe_status"}"#);
    println!("Failsafe status: {:?}", result);
    
    // ========================================
    // 3. Basic Mouse Actions
    // ========================================
    println!("\n--- 3. Basic Mouse Actions ---");
    
    // Move to absolute position
    println!("Moving to (400, 300)...");
    let result = agent.execute_json(r#"{"action": "mouse_move_to", "x": 400, "y": 300}"#);
    println!("Result: {:?}", result);
    sleep(Duration::from_millis(300));
    
    // Move by relative offset
    println!("Moving +100, +50...");
    let result = agent.execute_json(r#"{"action": "mouse_move", "x": 100, "y": 50}"#);
    println!("Result: {:?}", result);
    sleep(Duration::from_millis(300));
    
    // Click
    println!("Left click...");
    let result = agent.execute_json(r#"{"action": "mouse_click", "button": "left"}"#);
    println!("Result: {:?}", result);
    sleep(Duration::from_millis(300));
    
    // Double click
    println!("Double click...");
    let result = agent.execute_json(r#"{"action": "mouse_double_click"}"#);
    println!("Result: {:?}", result);
    sleep(Duration::from_millis(300));
    
    // Scroll
    println!("Scroll down...");
    let result = agent.execute_json(r#"{"action": "mouse_scroll", "delta": -3}"#);
    println!("Result: {:?}", result);
    
    // ========================================
    // 4. Smooth Movement with Tween
    // ========================================
    println!("\n--- 4. Smooth Movement ---");
    
    // Move to start position
    agent.execute_json(r#"{"action": "mouse_move_to", "x": 200, "y": 200}"#);
    sleep(Duration::from_millis(200));
    
    // Smooth move with duration and easing
    println!("Smooth move to (600, 400) over 1.5s with ease_out_cubic...");
    let result = agent.execute_json(
        r#"{"action": "mouse_move_to", "x": 600, "y": 400, "duration": 1500, "tween": "ease_out_cubic"}"#
    );
    println!("Result: {:?}", result);
    
    // Smooth relative move
    println!("Smooth move -200, +100 over 1s with ease_in_out_quad...");
    let result = agent.execute_json(
        r#"{"action": "mouse_move", "x": -200, "y": 100, "duration": 1000, "tween": "ease_in_out_quad"}"#
    );
    println!("Result: {:?}", result);
    
    // ========================================
    // 5. Drag Operations
    // ========================================
    println!("\n--- 5. Drag Operations ---");
    
    agent.execute_json(r#"{"action": "mouse_move_to", "x": 300, "y": 300}"#);
    sleep(Duration::from_millis(200));
    
    // Simple drag
    println!("Dragging +150, +50...");
    let result = agent.execute_json(r#"{"action": "mouse_drag", "x": 150, "y": 50}"#);
    println!("Result: {:?}", result);
    sleep(Duration::from_millis(300));
    
    // Smooth drag with duration
    println!("Smooth drag to (600, 500) over 1s...");
    let result = agent.execute_json(
        r#"{"action": "mouse_drag_to", "x": 600, "y": 500, "duration": 1000, "tween": "ease_in_out"}"#
    );
    println!("Result: {:?}", result);
    
    // ========================================
    // 6. Keyboard Actions
    // ========================================
    println!("\n--- 6. Keyboard Actions ---");
    
    // Type text (instant)
    println!("Typing 'Hello'...");
    let result = agent.execute_json(r#"{"action": "type", "text": "Hello"}"#);
    println!("Result: {:?}", result);
    sleep(Duration::from_millis(300));
    
    // Type with interval (typewriter effect)
    println!("Typing ' World!' with 100ms interval...");
    let result = agent.execute_json(r#"{"action": "type", "text": " World!", "interval": 100}"#);
    println!("Result: {:?}", result);
    sleep(Duration::from_millis(300));
    
    // Key press
    println!("Pressing Enter...");
    let result = agent.execute_json(r#"{"action": "key_press", "key": "enter"}"#);
    println!("Result: {:?}", result);
    sleep(Duration::from_millis(300));
    
    // Key combo
    println!("Pressing Cmd+A (select all)...");
    let result = agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "a"}"#);
    println!("Result: {:?}", result);
    
    // ========================================
    // 7. Available Tween Functions
    // ========================================
    println!("\n--- 7. Tween Functions ---");
    println!("Available tweens:");
    println!("  linear          - constant speed");
    println!("  ease_in         - slow start (quadratic)");
    println!("  ease_out        - slow end (quadratic)");
    println!("  ease_in_out     - slow start and end (quadratic)");
    println!("  ease_in_cubic   - cubic slow start");
    println!("  ease_out_cubic  - cubic slow end");
    println!("  ease_in_out_cubic - cubic slow start and end");
    println!("  ease_out_elastic - elastic bounce");
    println!("  ease_out_bounce  - bounce effect");
    
    // ========================================
    // 8. Final Position Check
    // ========================================
    println!("\n--- 8. Final State ---");
    let result = agent.execute_json(r#"{"action": "position"}"#);
    println!("Final mouse position: {:?}", result);
    
    let result = agent.execute_json(r#"{"action": "failsafe_status"}"#);
    println!("Failsafe status: {:?}", result);
    
    println!("\n=== Demo Complete ===");
}
