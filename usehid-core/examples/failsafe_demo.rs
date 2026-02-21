//! Example: Failsafe mechanism demo
//!
//! Run with: cargo run --release -p usehid --example failsafe_demo

use usehid::{
    AgentHID, position, size, move_to,
    set_failsafe_enabled, is_failsafe_enabled, is_failsafe_triggered,
    reset_failsafe, check_failsafe_default,
};

fn main() {
    println!("=== useHID Failsafe Demo ===\n");

    let mut agent = AgentHID::new();
    let screen = size().expect("Failed to get screen size");
    
    println!("Screen: {}x{}", screen.width, screen.height);
    println!("Failsafe triggers when mouse is in any screen corner (within 5px)\n");

    // Check failsafe status via Agent API
    println!("--- Failsafe Status ---");
    let result = agent.execute_json(r#"{"action": "failsafe_status"}"#);
    println!("Status: {:?}\n", result);

    // Move to center (safe)
    let center_x = screen.width as i32 / 2;
    let center_y = screen.height as i32 / 2;
    println!("Moving to center ({}, {})...", center_x, center_y);
    move_to(center_x, center_y).ok();
    
    // Check - should pass
    match check_failsafe_default() {
        Ok(_) => println!("✓ Failsafe check passed (mouse in safe area)\n"),
        Err(e) => println!("✗ Failsafe triggered: {}\n", e),
    }

    // Move near corner but not in it
    println!("Moving near top-left corner (10, 10)...");
    move_to(10, 10).ok();
    
    match check_failsafe_default() {
        Ok(_) => println!("✓ Failsafe check passed (just outside threshold)\n"),
        Err(e) => println!("✗ Failsafe triggered: {}\n", e),
    }

    // Move into corner (will trigger failsafe)
    println!("Moving to top-left corner (0, 0)...");
    move_to(0, 0).ok();
    
    match check_failsafe_default() {
        Ok(_) => println!("✓ Failsafe check passed\n"),
        Err(e) => println!("✗ Failsafe triggered: {}\n", e),
    }

    // Check triggered state
    println!("Is failsafe triggered? {}", is_failsafe_triggered());
    
    // Try to do something - should fail
    println!("\nTrying to move mouse while failsafe is triggered...");
    let result = agent.execute_json(r#"{"action": "mouse_move", "x": 100, "y": 100}"#);
    println!("Result: {:?}", result);

    // Reset failsafe
    println!("\n--- Resetting Failsafe ---");
    agent.execute_json(r#"{"action": "failsafe_reset"}"#);
    println!("Failsafe reset.");
    
    // Move back to center
    move_to(center_x, center_y).ok();
    
    // Now operations should work
    println!("Trying to move mouse after reset...");
    let result = agent.execute_json(r#"{"action": "mouse_move", "x": 50, "y": 50}"#);
    println!("Result: {:?}", result);

    // Disable failsafe
    println!("\n--- Disable Failsafe ---");
    agent.execute_json(r#"{"action": "failsafe_disable"}"#);
    println!("Failsafe disabled: {}", !is_failsafe_enabled());
    
    // Move to corner - should NOT trigger now
    move_to(0, 0).ok();
    let result = agent.execute_json(r#"{"action": "mouse_move", "x": 10, "y": 10}"#);
    println!("Move in corner with failsafe disabled: {:?}", result);

    // Re-enable
    agent.execute_json(r#"{"action": "failsafe_enable"}"#);
    println!("Failsafe re-enabled.");

    // Move back to center
    move_to(center_x, center_y).ok();
    
    println!("\nDone!");
}
