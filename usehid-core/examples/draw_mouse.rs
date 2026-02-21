//! Example: Drawing with mouse
//!
//! Demonstrates mouse drag operations by drawing shapes

use usehid::{Device, Mouse, MouseButton};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Mouse Drawing Example");
    println!("========================");
    println!("Open a drawing app (Paint, Preview, etc.) and get ready!");
    sleep(Duration::from_secs(3));
    
    let mut mouse = Mouse::new();
    mouse.create()?;
    
    // Draw a square by dragging
    println!("📐 Drawing a square...");
    
    // Press left button to start drawing
    mouse.press(MouseButton::LEFT)?;
    
    // Draw right
    for _ in 0..50 {
        mouse.move_by(2, 0)?;
        sleep(Duration::from_millis(10));
    }
    
    // Draw down
    for _ in 0..50 {
        mouse.move_by(0, 2)?;
        sleep(Duration::from_millis(10));
    }
    
    // Draw left
    for _ in 0..50 {
        mouse.move_by(-2, 0)?;
        sleep(Duration::from_millis(10));
    }
    
    // Draw up
    for _ in 0..50 {
        mouse.move_by(0, -2)?;
        sleep(Duration::from_millis(10));
    }
    
    // Release to stop drawing
    mouse.release(MouseButton::LEFT)?;
    
    sleep(Duration::from_millis(500));
    
    // Draw a circle
    println!("⭕ Drawing a circle...");
    mouse.move_by(150, 0)?;
    sleep(Duration::from_millis(100));
    
    mouse.press(MouseButton::LEFT)?;
    
    let radius = 40.0;
    let steps = 60;
    for i in 0..=steps {
        let angle = 2.0 * std::f64::consts::PI * (i as f64) / (steps as f64);
        let dx = (radius * angle.cos() - radius * ((i - 1) as f64 / steps as f64 * 2.0 * std::f64::consts::PI).cos()) as i32;
        let dy = (radius * angle.sin() - radius * ((i - 1) as f64 / steps as f64 * 2.0 * std::f64::consts::PI).sin()) as i32;
        mouse.move_by(dx.max(-127).min(127), dy.max(-127).min(127))?;
        sleep(Duration::from_millis(20));
    }
    
    mouse.release(MouseButton::LEFT)?;
    
    println!("\n✅ Drawing complete!");
    
    mouse.destroy()?;
    Ok(())
}
