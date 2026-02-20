//! Platform-specific HID backends

use crate::error::Result;

/// HID backend trait
pub trait HidBackend: Send + Sync {
    /// Send HID report
    fn send_report(&self, report: &[u8]) -> Result<()>;
    
    /// Destroy the device
    fn destroy(self: Box<Self>) -> Result<()>;
}

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "windows")]
pub use windows::*;

// Fallback implementations for unsupported platforms
#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
mod fallback {
    use super::*;
    use crate::error::Error;
    
    pub fn create_mouse_backend(_name: &str) -> Result<Box<dyn HidBackend>> {
        Err(Error::PlatformNotSupported("unsupported platform".into()))
    }
    
    pub fn create_keyboard_backend(_name: &str) -> Result<Box<dyn HidBackend>> {
        Err(Error::PlatformNotSupported("unsupported platform".into()))
    }
    
    pub fn create_gamepad_backend(_name: &str) -> Result<Box<dyn HidBackend>> {
        Err(Error::PlatformNotSupported("unsupported platform".into()))
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
pub use fallback::*;
