//! Example: Virtual Mouse
//!
//! This example demonstrates how to create a virtual mouse and move it around.
//!
//! Note: On macOS, this requires proper entitlements. Run with sudo or sign with
//! com.apple.developer.hid.virtual.device entitlement.

use usehid::{Device, Mouse, MouseButton};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating virtual mouse...");
    
    let mut mouse = Mouse::new();
    mouse.create()?;
    
    println!("Virtual mouse created! Moving around...");
    
    // Move mouse in a square pattern
    for _ in 0..4 {
        // Move right
        for _ in 0..10 {
            mouse.move_by(10, 0)?;
            sleep(Duration::from_millis(50));
        }
        
        // Move down
        for _ in 0..10 {
            mouse.move_by(0, 10)?;
            sleep(Duration::from_millis(50));
        }
        
        // Move left
        for _ in 0..10 {
            mouse.move_by(-10, 0)?;
            sleep(Duration::from_millis(50));
        }
        
        // Move up
        for _ in 0..10 {
            mouse.move_by(0, -10)?;
            sleep(Duration::from_millis(50));
        }
    }
    
    // Click
    println!("Clicking...");
    mouse.click(MouseButton::LEFT)?;
    
    // Double click
    sleep(Duration::from_millis(500));
    println!("Double clicking...");
    mouse.double_click(MouseButton::LEFT)?;
    
    // Right click
    sleep(Duration::from_millis(500));
    println!("Right clicking...");
    mouse.click(MouseButton::RIGHT)?;
    
    // Scroll
    sleep(Duration::from_millis(500));
    println!("Scrolling...");
    for _ in 0..5 {
        mouse.scroll(-3)?; // Scroll down
        sleep(Duration::from_millis(100));
    }
    for _ in 0..5 {
        mouse.scroll(3)?; // Scroll up
        sleep(Duration::from_millis(100));
    }
    
    println!("Done!");
    mouse.destroy()?;
    
    Ok(())
}
