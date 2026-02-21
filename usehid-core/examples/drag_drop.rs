//! Example: Drag and Drop operations
//!
//! Demonstrates various drag operations with and without smooth animation.
//!
//! Run with: cargo run --release -p usehid --example drag_drop

use usehid::{AgentHID, size, position, move_to};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("=== useHID Drag & Drop Demo ===\n");
    
    let mut agent = AgentHID::new();
    let screen = size().expect("Failed to get screen size");
    
    println!("Screen: {}x{}\n", screen.width, screen.height);
    
    // ========================================
    // 1. Basic Drag (Relative)
    // ========================================
    println!("--- 1. Basic Relative Drag ---");
    
    // Start position
    let start_x = 300;
    let start_y = 300;
    move_to(start_x, start_y).ok();
    println!("Starting at ({}, {})", start_x, start_y);
    sleep(Duration::from_millis(300));
    
    // Drag right and down
    println!("Dragging +200, +100 (instant)...");
    let result = agent.execute_json(r#"{"action": "mouse_drag", "x": 200, "y": 100}"#);
    println!("Result: {:?}", result);
    
    let pos = position().unwrap();
    println!("Now at ({}, {})\n", pos.x, pos.y);
    sleep(Duration::from_millis(500));
    
    // ========================================
    // 2. Drag To (Absolute)
    // ========================================
    println!("--- 2. Absolute Drag To ---");
    
    let target_x = 600;
    let target_y = 200;
    println!("Dragging to ({}, {})...", target_x, target_y);
    
    let json = format!(r#"{{"action": "mouse_drag_to", "x": {}, "y": {}}}"#, target_x, target_y);
    let result = agent.execute_json(&json);
    println!("Result: {:?}", result);
    
    let pos = position().unwrap();
    println!("Now at ({}, {})\n", pos.x, pos.y);
    sleep(Duration::from_millis(500));
    
    // ========================================
    // 3. Smooth Drag with Duration
    // ========================================
    println!("--- 3. Smooth Drag (1 second, ease_in_out) ---");
    
    move_to(200, 400).ok();
    sleep(Duration::from_millis(200));
    
    println!("Smooth dragging +400, -100 over 1 second...");
    let result = agent.execute_json(
        r#"{"action": "mouse_drag", "x": 400, "y": -100, "duration": 1000, "tween": "ease_in_out"}"#
    );
    println!("Result: {:?}", result);
    
    let pos = position().unwrap();
    println!("Now at ({}, {})\n", pos.x, pos.y);
    sleep(Duration::from_millis(300));
    
    // ========================================
    // 4. Smooth Drag To with Different Tweens
    // ========================================
    println!("--- 4. Smooth Drag To with Tweens ---");
    
    // Linear
    move_to(100, 300).ok();
    sleep(Duration::from_millis(200));
    println!("Linear drag to (500, 300) over 800ms...");
    agent.execute_json(
        r#"{"action": "mouse_drag_to", "x": 500, "y": 300, "duration": 800, "tween": "linear"}"#
    );
    sleep(Duration::from_millis(200));
    
    // EaseOutCubic
    move_to(100, 400).ok();
    sleep(Duration::from_millis(200));
    println!("EaseOutCubic drag to (500, 400) over 800ms...");
    agent.execute_json(
        r#"{"action": "mouse_drag_to", "x": 500, "y": 400, "duration": 800, "tween": "ease_out_cubic"}"#
    );
    sleep(Duration::from_millis(200));
    
    // EaseOutBounce
    move_to(100, 500).ok();
    sleep(Duration::from_millis(200));
    println!("EaseOutBounce drag to (500, 500) over 1000ms...");
    agent.execute_json(
        r#"{"action": "mouse_drag_to", "x": 500, "y": 500, "duration": 1000, "tween": "ease_out_bounce"}"#
    );
    
    // ========================================
    // 5. Right-button Drag
    // ========================================
    println!("\n--- 5. Right-button Drag ---");
    
    move_to(400, 400).ok();
    sleep(Duration::from_millis(200));
    
    println!("Right-button drag +100, +50...");
    let result = agent.execute_json(
        r#"{"action": "mouse_drag", "x": 100, "y": 50, "button": "right"}"#
    );
    println!("Result: {:?}", result);
    
    // ==============================
    // 6. Drawing a Rectangle (multiple drags)
    // ========================================
    println!("\n--- 6. Drawing a Rectangle ---");
    
    let rect_start_x = 200;
    let rect_start_y = 200;
    let rect_width = 300;
    let rect_height = 200;
    
    move_to(rect_start_x, rect_start_y).ok();
    sleep(Duration::from_millis(200));
    
    println!("Drawing rectangle {}x{}...", rect_width, rect_height);
    
    // Top edge
    agent.execute_json(&format!(
        r#"{{"action": "mouse_drag", "x": {}, "y": 0, "duration": 300}}"#, rect_width
    ));
    
    // Right edge
    agent.execute_json(&format!(
        r#"{{"action": "mouse_drag", "x": 0, "y": {}, "duration": 300}}"#, rect_height
    ));
    
    // Bottom edge
    agent.execute_json(&format!(
        r#"{{"action": "mouse_drag", "x": -{}, "y": 0, "duration": 300}}"#, rect_width
    ));
    
    // Left edge
    agent.execute_json(&format!(
        r#"{{"action": "mouse_drag", "x": 0, "y": -{}, "duration": 300}}"#, rect_height
    ));
    
    println!("Rectangle complete!");
    
    // Move to center
    let center_x = screen.width as i32 / 2;
    let center_y = screen.height as i32 / 2;
    move_to(center_x, center_y).ok();
    
    println!("\n=== Demo Complete ===");
}
