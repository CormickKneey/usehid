//! Virtual Gamepad implementation

use crate::error::{Error, Result};
use crate::hid::GamepadReport;
use crate::platform::HidBackend;
use crate::Device;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    /// Gamepad button flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
    pub struct GamepadButton: u16 {
        const A = 0x0001;
        const B = 0x0002;
        const X = 0x0004;
        const Y = 0x0008;
        const LB = 0x0010;
        const RB = 0x0020;
        const BACK = 0x0040;
        const START = 0x0080;
        const GUIDE = 0x0100;
        const LEFT_STICK = 0x0200;
        const RIGHT_STICK = 0x0400;
        const DPAD_UP = 0x0800;
        const DPAD_DOWN = 0x1000;
        const DPAD_LEFT = 0x2000;
        const DPAD_RIGHT = 0x4000;
    }
}

/// Virtual gamepad device
pub struct Gamepad {
    backend: Option<Box<dyn HidBackend>>,
    report: GamepadReport,
    name: String,
}

impl Gamepad {
    /// Create a new virtual gamepad
    pub fn new() -> Self {
        Self::with_name("useHID Virtual Gamepad")
    }
    
    /// Create a new virtual gamepad with custom name
    pub fn with_name(name: &str) -> Self {
        let mut gamepad = Self {
            backend: None,
            report: GamepadReport::default(),
            name: name.to_string(),
        };
        // Center sticks
        gamepad.report.left_x = 128;
        gamepad.report.left_y = 128;
        gamepad.report.right_x = 128;
        gamepad.report.right_y = 128;
        gamepad
    }
    
    /// Press button(s)
    pub fn press(&mut self, button: GamepadButton) -> Result<()> {
        self.report.buttons |= button.bits();
        self.send_report()
    }
    
    /// Release button(s)
    pub fn release(&mut self, button: GamepadButton) -> Result<()> {
        self.report.buttons &= !button.bits();
        self.send_report()
    }
    
    /// Tap button (press and release)
    pub fn tap(&mut self, button: GamepadButton) -> Result<()> {
        self.press(button)?;
        std::thread::sleep(std::time::Duration::from_millis(50));
        self.release(button)
    }
    
    /// Set left stick position (0-255, 128 = center)
    pub fn set_left_stick(&mut self, x: u8, y: u8) -> Result<()> {
        self.report.left_x = x;
        self.report.left_y = y;
        self.send_report()
    }
    
    /// Set right stick position (0-255, 128 = center)
    pub fn set_right_stick(&mut self, x: u8, y: u8) -> Result<()> {
        self.report.right_x = x;
        self.report.right_y = y;
        self.send_report()
    }
    
    /// Set left trigger (0-255)
    pub fn set_left_trigger(&mut self, value: u8) -> Result<()> {
        self.report.left_trigger = value;
        self.send_report()
    }
    
    /// Set right trigger (0-255)
    pub fn set_right_trigger(&mut self, value: u8) -> Result<()> {
        self.report.right_trigger = value;
        self.send_report()
    }
    
    /// Reset all to default (center sticks, no buttons)
    pub fn reset(&mut self) -> Result<()> {
        self.report.buttons = 0;
        self.report.left_x = 128;
        self.report.left_y = 128;
        self.report.right_x = 128;
        self.report.right_y = 128;
        self.report.left_trigger = 0;
        self.report.right_trigger = 0;
        self.send_report()
    }
    
    fn send_report(&self) -> Result<()> {
        if let Some(backend) = &self.backend {
            backend.send_report(self.report.as_bytes())
        } else {
            Err(Error::DeviceNotCreated)
        }
    }
}

impl Default for Gamepad {
    fn default() -> Self {
        Self::new()
    }
}

impl Device for Gamepad {
    fn create(&mut self) -> Result<()> {
        if self.backend.is_some() {
            return Err(Error::DeviceAlreadyExists);
        }
        
        let backend = crate::platform::create_gamepad_backend(&self.name)?;
        self.backend = Some(backend);
        Ok(())
    }
    
    fn destroy(&mut self) -> Result<()> {
        if let Some(backend) = self.backend.take() {
            backend.destroy()
        } else {
            Err(Error::DeviceNotCreated)
        }
    }
    
    fn is_created(&self) -> bool {
        self.backend.is_some()
    }
}
