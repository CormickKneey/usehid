//! Example: Virtual Keyboard
//!
//! This example demonstrates how to create a virtual keyboard and type text.

use usehid_core::{Device, Keyboard, Key, Modifiers};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating virtual keyboard...");
    println!("You have 3 seconds to focus a text input...");
    sleep(Duration::from_secs(3));
    
    let mut keyboard = Keyboard::new();
    keyboard.create()?;
    
    println!("Typing...");
    
    // Type some text
    keyboard.type_text("Hello from useHID! ")?;
    sleep(Duration::from_millis(500));
    
    // Type with special characters
    keyboard.type_text("Testing 123... @#$%")?;
    sleep(Duration::from_millis(500));
    
    // Press Enter
    keyboard.tap(Key::Enter)?;
    sleep(Duration::from_millis(200));
    
    // Type another line
    keyboard.type_text("New line!")?;
    
    // Key combination: Ctrl+A (select all)
    sleep(Duration::from_millis(500));
    println!("Pressing Ctrl+A...");
    keyboard.press_combo(Modifiers::CTRL, Key::A)?;
    
    // Key combination: Ctrl+C (copy)
    sleep(Duration::from_millis(500));
    println!("Pressing Ctrl+C...");
    keyboard.press_combo(Modifiers::CTRL, Key::C)?;
    
    println!("Done!");
    keyboard.destroy()?;
    
    Ok(())
}
