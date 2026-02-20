//! Example: Drag and drop
//!
//! Demonstrates drag and drop operations

use usehid_core::AgentHID;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("🖱️ Drag and Drop Example");
    println!("=========================");
    println!("This example simulates dragging an item.");
    println!("Position your mouse over an item to drag in 3 seconds...");
    sleep(Duration::from_secs(3));
    
    let mut agent = AgentHID::new();
    
    // Press and hold left button
    println!("⬇️ Pressing mouse button...");
    agent.execute_json(r#"{"action": "mouse_down", "button": "left"}"#);
    sleep(Duration::from_millis(100));
    
    // Drag to the right and down
    println!("➡️⬇️ Dragging...");
    for _ in 0..20 {
        agent.execute_json(r#"{"action": "mouse_move", "x": 10, "y": 5}"#);
        sleep(Duration::from_millis(30));
    }
    
    sleep(Duration::from_millis(200));
    
    // Release to drop
    println!("⬆️ Releasing (drop)...");
    agent.execute_json(r#"{"action": "mouse_up", "button": "left"}"#);
    
    println!("\n✅ Drag and drop complete!");
    println!("\nTip: Use this for:");
    println!("  - Moving files in file managers");
    println!("  - Reordering items in lists");
    println!("  - Moving windows");
}
