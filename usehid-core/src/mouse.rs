//! Virtual Mouse implementation

use crate::error::Result;
use crate::hid::MouseReport;
use crate::platform::HidBackend;
use crate::Device;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    /// Mouse button flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct MouseButton: u8 {
        const LEFT = 0x01;
        const RIGHT = 0x02;
        const MIDDLE = 0x04;
        const BUTTON4 = 0x08;
        const BUTTON5 = 0x10;
    }
}

/// Virtual mouse device
pub struct Mouse {
    backend: Option<Box<dyn HidBackend>>,
    report: MouseReport,
    name: String,
}

impl Mouse {
    /// Create a new virtual mouse
    pub fn new() -> Self {
        Self::with_name("useHID Virtual Mouse")
    }
    
    /// Create a new virtual mouse with custom name
    pub fn with_name(name: &str) -> Self {
        Self {
            backend: None,
            report: MouseReport::default(),
            name: name.to_string(),
        }
    }
    
    /// Move mouse by relative offset
    pub fn move_by(&mut self, dx: i32, dy: i32) -> Result<()> {
        // Clamp to i8 range, may need multiple reports for large movements
        let steps_x = (dx.abs() + 126) / 127;
        let steps_y = (dy.abs() + 126) / 127;
        let steps = steps_x.max(steps_y).max(1);
        
        for i in 0..steps {
            let x = if i < steps_x {
                (dx.signum() * (dx.abs() / steps).min(127)) as i8
            } else {
                0
            };
            let y = if i < steps_y {
                (dy.signum() * (dy.abs() / steps).min(127)) as i8
            } else {
                0
            };
            
            self.report.x = x;
            self.report.y = y;
            self.send_report()?;
        }
        
        self.report.x = 0;
        self.report.y = 0;
        Ok(())
    }
    
    /// Press mouse button(s)
    pub fn press(&mut self, button: MouseButton) -> Result<()> {
        self.report.buttons |= button.bits();
        self.send_report()
    }
    
    /// Release mouse button(s)
    pub fn release(&mut self, button: MouseButton) -> Result<()> {
        self.report.buttons &= !button.bits();
        self.send_report()
    }
    
    /// Click (press and release) mouse button
    pub fn click(&mut self, button: MouseButton) -> Result<()> {
        self.press(button)?;
        std::thread::sleep(std::time::Duration::from_millis(10));
        self.release(button)
    }
    
    /// Double click
    pub fn double_click(&mut self, button: MouseButton) -> Result<()> {
        self.click(button)?;
        std::thread::sleep(std::time::Duration::from_millis(50));
        self.click(button)
    }
    
    /// Scroll wheel
    pub fn scroll(&mut self, delta: i8) -> Result<()> {
        self.report.wheel = delta;
        self.send_report()?;
        self.report.wheel = 0;
        Ok(())
    }
    
    fn send_report(&self) -> Result<()> {
        if let Some(backend) = &self.backend {
            backend.send_report(self.report.as_bytes())
        } else {
            Err(crate::error::Error::DeviceNotCreated)
        }
    }
}

impl Default for Mouse {
    fn default() -> Self {
        Self::new()
    }
}

impl Device for Mouse {
    fn create(&mut self) -> Result<()> {
        if self.backend.is_some() {
            return Err(crate::error::Error::DeviceAlreadyExists);
        }
        
        let backend = crate::platform::create_mouse_backend(&self.name)?;
        self.backend = Some(backend);
        Ok(())
    }
    
    fn destroy(&mut self) -> Result<()> {
        if let Some(backend) = self.backend.take() {
            backend.destroy()
        } else {
            Err(crate::error::Error::DeviceNotCreated)
        }
    }
    
    fn is_created(&self) -> bool {
        self.backend.is_some()
    }
}
