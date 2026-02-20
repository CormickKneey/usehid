//! useHID - Cross-platform virtual HID device library
//!
//! This library provides a unified API for creating virtual HID devices
//! (mouse, keyboard, gamepad) across macOS, Linux, and Windows.

pub mod error;
pub mod hid;
pub mod keyboard;
pub mod mouse;
pub mod gamepad;
pub mod platform;
pub mod agent;

pub use error::{Error, Result};
pub use keyboard::{Keyboard, Key, Modifiers};
pub use mouse::{Mouse, MouseButton};
pub use gamepad::{Gamepad, GamepadButton};
pub use agent::AgentHID;

/// Device trait for all virtual HID devices
pub trait Device {
    /// Create and register the virtual device
    fn create(&mut self) -> Result<()>;
    
    /// Destroy and unregister the virtual device
    fn destroy(&mut self) -> Result<()>;
    
    /// Check if device is created
    fn is_created(&self) -> bool;
}
