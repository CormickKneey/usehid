//! Windows HID backend (placeholder)
//!
//! TODO: Implement using ViGEmBus for gamepad, or custom virtual HID driver

use super::HidBackend;
use crate::error::{Error, Result};

pub struct WindowsHidBackend;

impl HidBackend for WindowsHidBackend {
    fn send_report(&self, _report: &[u8]) -> Result<()> {
        Err(Error::PlatformNotSupported("Windows support not yet implemented".into()))
    }
    
    fn destroy(self: Box<Self>) -> Result<()> {
        Ok(())
    }
}

pub fn create_mouse_backend(_name: &str) -> Result<Box<dyn HidBackend>> {
    Err(Error::PlatformNotSupported(
        "Windows virtual mouse not yet implemented. Consider using SendInput API for now.".into()
    ))
}

pub fn create_keyboard_backend(_name: &str) -> Result<Box<dyn HidBackend>> {
    Err(Error::PlatformNotSupported(
        "Windows virtual keyboard not yet implemented. Consider using SendInput API for now.".into()
    ))
}

pub fn create_gamepad_backend(_name: &str) -> Result<Box<dyn HidBackend>> {
    Err(Error::PlatformNotSupported(
        "Windows virtual gamepad not yet implemented. Consider using ViGEmBus.".into()
    ))
}
