//! Example: Query screen size and mouse position
//!
//! Run with: cargo run --release -p usehid --example screen_info

use usehid::{size, position, move_to, AgentHID};

fn main() {
    println!("=== useHID Screen Info Example ===\n");

    // Direct API
    println!("--- Direct API ---");
    
    match size() {
        Ok(s) => println!("Screen size: {}x{}", s.width, s.height),
        Err(e) => println!("Error getting screen size: {}", e),
    }

    match position() {
        Ok(p) => println!("Mouse position: ({}, {})", p.x, p.y),
        Err(e) => println!("Error getting mouse position: {}", e),
    }

    // Agent API (JSON)
    println!("\n--- Agent API ---");
    
    let mut agent = AgentHID::new();
    
    let result = agent.execute_json(r#"{"action": "size"}"#);
    println!("size: {:?}", result);
    
    let result = agent.execute_json(r#"{"action": "position"}"#);
    println!("position: {:?}", result);

    // Test move_to
    println!("\n--- Move To Test ---");
    println!("Moving mouse to (100, 100)...");
    
    match move_to(100, 100) {
        Ok(_) => {
            match position() {
                Ok(p) => println!("New position: ({}, {})", p.x, p.y),
                Err(e) => println!("Error: {}", e),
            }
        }
        Err(e) => println!("Error moving mouse: {}", e),
    }

    // Agent API move_to
    let result = agent.execute_json(r#"{"action": "mouse_move_to", "x": 200, "y": 200}"#);
    println!("mouse_move_to result: {:?}", result);
    
    let result = agent.execute_json(r#"{"action": "position"}"#);
    println!("position after move_to: {:?}", result);

    // Test drag
    println!("\n--- Drag Test ---");
    let result = agent.execute_json(r#"{"action": "mouse_drag", "x": 50, "y": 50}"#);
    println!("mouse_drag result: {:?}", result);

    println!("\nDone!");
}
