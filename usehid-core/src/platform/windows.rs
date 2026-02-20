//! Windows HID backend using SendInput API
//!
//! Uses Win32 SendInput for mouse and keyboard simulation.
//! For gamepad, uses ViGEmBus if available.

use super::HidBackend;
use crate::error::{Error, Result};

#[cfg(target_os = "windows")]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, INPUT_MOUSE,
    KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP,
    MOUSEINPUT, MOUSE_EVENT_FLAGS,
    MOUSEEVENTF_MOVE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
    MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,
    MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP,
    MOUSEEVENTF_WHEEL,
    VIRTUAL_KEY,
};

#[cfg(target_os = "windows")]
const WHEEL_DELTA: i32 = 120;

#[cfg(target_os = "windows")]
use std::mem::size_of;

/// Windows SendInput mouse backend
#[cfg(target_os = "windows")]
pub struct WindowsMouseBackend {
    buttons: u8,
}

#[cfg(target_os = "windows")]
unsafe impl Send for WindowsMouseBackend {}
#[cfg(target_os = "windows")]
unsafe impl Sync for WindowsMouseBackend {}

#[cfg(target_os = "windows")]
impl WindowsMouseBackend {
    pub fn new() -> Result<Self> {
        Ok(Self { buttons: 0 })
    }
    
    fn send_mouse_input(&self, flags: MOUSE_EVENT_FLAGS, dx: i32, dy: i32, data: i32) -> Result<()> {
        let input = INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx,
                    dy,
                    mouseData: data as u32,
                    dwFlags: flags,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        
        unsafe {
            let sent = SendInput(&[input], size_of::<INPUT>() as i32);
            if sent == 0 {
                return Err(Error::SendFailed("SendInput failed".into()));
            }
        }
        Ok(())
    }
}

#[cfg(target_os = "windows")]
impl HidBackend for WindowsMouseBackend {
    fn send_report(&self, report: &[u8]) -> Result<()> {
        if report.len() < 4 {
            return Err(Error::SendFailed("Invalid mouse report".into()));
        }
        
        let buttons = report[0];
        let dx = report[1] as i8 as i32;
        let dy = report[2] as i8 as i32;
        let wheel = report[3] as i8 as i32;
        
        // Move mouse
        if dx != 0 || dy != 0 {
            self.send_mouse_input(MOUSEEVENTF_MOVE, dx, dy, 0)?;
        }
        
        // Get mutable reference to update button state
        let self_mut = unsafe { &mut *(self as *const Self as *mut Self) };
        let old_buttons = self_mut.buttons;
        self_mut.buttons = buttons;
        
        // Left button
        if (buttons & 0x01) != 0 && (old_buttons & 0x01) == 0 {
            self.send_mouse_input(MOUSEEVENTF_LEFTDOWN, 0, 0, 0)?;
        } else if (buttons & 0x01) == 0 && (old_buttons & 0x01) != 0 {
            self.send_mouse_input(MOUSEEVENTF_LEFTUP, 0, 0, 0)?;
        }
        
        // Right button
        if (buttons & 0x02) != 0 && (old_buttons & 0x02) == 0 {
            self.send_mouse_input(MOUSEEVENTF_RIGHTDOWN, 0, 0, 0)?;
        } else if (buttons & 0x02) == 0 && (old_buttons & 0x02) != 0 {
            self.send_mouse_input(MOUSEEVENTF_RIGHTUP, 0, 0, 0)?;
        }
        
        // Middle button
        if (buttons & 0x04) != 0 && (old_buttons & 0x04) == 0 {
            self.send_mouse_input(MOUSEEVENTF_MIDDLEDOWN, 0, 0, 0)?;
        } else if (buttons & 0x04) == 0 && (old_buttons & 0x04) != 0 {
            self.send_mouse_input(MOUSEEVENTF_MIDDLEUP, 0, 0, 0)?;
        }
        
        // Scroll wheel
        if wheel != 0 {
            self.send_mouse_input(MOUSEEVENTF_WHEEL, 0, 0, wheel * WHEEL_DELTA as i32)?;
        }
        
        Ok(())
    }
    
    fn destroy(self: Box<Self>) -> Result<()> {
        Ok(())
    }
}

/// Windows SendInput keyboard backend
#[cfg(target_os = "windows")]
pub struct WindowsKeyboardBackend;

#[cfg(target_os = "windows")]
unsafe impl Send for WindowsKeyboardBackend {}
#[cfg(target_os = "windows")]
unsafe impl Sync for WindowsKeyboardBackend {}

#[cfg(target_os = "windows")]
impl WindowsKeyboardBackend {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
    
    fn send_key(&self, vk: u16, down: bool) -> Result<()> {
        let flags = if down {
            KEYBD_EVENT_FLAGS::default()
        } else {
            KEYEVENTF_KEYUP
        };
        
        let input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(vk),
                    wScan: 0,
                    dwFlags: flags,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        
        unsafe {
            let sent = SendInput(&[input], size_of::<INPUT>() as i32);
            if sent == 0 {
                return Err(Error::SendFailed("SendInput failed".into()));
            }
        }
        Ok(())
    }
    
    // Convert USB HID keycode to Windows virtual key code
    fn hid_to_vk(hid: u8) -> Option<u16> {
        Some(match hid {
            0x04..=0x1D => (hid - 0x04 + 0x41) as u16, // A-Z
            0x1E..=0x26 => (hid - 0x1E + 0x31) as u16, // 1-9
            0x27 => 0x30, // 0
            0x28 => 0x0D, // Enter
            0x29 => 0x1B, // Escape
            0x2A => 0x08, // Backspace
            0x2B => 0x09, // Tab
            0x2C => 0x20, // Space
            0x2D => 0xBD, // Minus
            0x2E => 0xBB, // Equal
            0x2F => 0xDB, // Left Bracket
            0x30 => 0xDD, // Right Bracket
            0x31 => 0xDC, // Backslash
            0x33 => 0xBA, // Semicolon
            0x34 => 0xDE, // Quote
            0x35 => 0xC0, // Grave
            0x36 => 0xBC, // Comma
            0x37 => 0xBE, // Period
            0x38 => 0xBF, // Slash
            0x39 => 0x14, // Caps Lock
            0x3A..=0x45 => (hid - 0x3A + 0x70) as u16, // F1-F12
            0x49 => 0x2D, // Insert
            0x4A => 0x24, // Home
            0x4B => 0x21, // Page Up
            0x4C => 0x2E, // Delete
            0x4D => 0x23, // End
            0x4E => 0x22, // Page Down
            0x4F => 0x27, // Right Arrow
            0x50 => 0x25, // Left Arrow
            0x51 => 0x28, // Down Arrow
            0x52 => 0x26, // Up Arrow
            _ => return None,
        })
    }
}

#[cfg(target_os = "windows")]
impl HidBackend for WindowsKeyboardBackend {
    fn send_report(&self, report: &[u8]) -> Result<()> {
        if report.len() < 8 {
            return Err(Error::SendFailed("Invalid keyboard report".into()));
        }
        
        let modifiers = report[0];
        let keys = &report[2..8];
        
        // Handle modifiers
        // Left Ctrl
        if modifiers & 0x01 != 0 {
            self.send_key(0xA2, true)?; // VK_LCONTROL
        }
        // Left Shift
        if modifiers & 0x02 != 0 {
            self.send_key(0xA0, true)?; // VK_LSHIFT
        }
        // Left Alt
        if modifiers & 0x04 != 0 {
            self.send_key(0xA4, true)?; // VK_LMENU
        }
        // Left GUI (Win)
        if modifiers & 0x08 != 0 {
            self.send_key(0x5B, true)?; // VK_LWIN
        }
        
        // Send key events
        for &key in keys {
            if key == 0 { continue; }
            
            if let Some(vk) = Self::hid_to_vk(key) {
                self.send_key(vk, true)?;
                std::thread::sleep(std::time::Duration::from_millis(5));
                self.send_key(vk, false)?;
            }
        }
        
        // Release modifiers
        if modifiers & 0x01 != 0 {
            self.send_key(0xA2, false)?;
        }
        if modifiers & 0x02 != 0 {
            self.send_key(0xA0, false)?;
        }
        if modifiers & 0x04 != 0 {
            self.send_key(0xA4, false)?;
        }
        if modifiers & 0x08 != 0 {
            self.send_key(0x5B, false)?;
        }
        
        Ok(())
    }
    
    fn destroy(self: Box<Self>) -> Result<()> {
        Ok(())
    }
}

/// Placeholder gamepad backend (requires ViGEmBus)
#[cfg(target_os = "windows")]
pub struct WindowsGamepadBackend;

#[cfg(target_os = "windows")]
impl HidBackend for WindowsGamepadBackend {
    fn send_report(&self, _report: &[u8]) -> Result<()> {
        Err(Error::PlatformNotSupported(
            "Windows gamepad requires ViGEmBus driver. Install from https://github.com/ViGEm/ViGEmBus".into()
        ))
    }
    
    fn destroy(self: Box<Self>) -> Result<()> {
        Ok(())
    }
}

#[cfg(target_os = "windows")]
pub fn create_mouse_backend(_name: &str) -> Result<Box<dyn HidBackend>> {
    Ok(Box::new(WindowsMouseBackend::new()?))
}

#[cfg(target_os = "windows")]
pub fn create_keyboard_backend(_name: &str) -> Result<Box<dyn HidBackend>> {
    Ok(Box::new(WindowsKeyboardBackend::new()?))
}

#[cfg(target_os = "windows")]
pub fn create_gamepad_backend(_name: &str) -> Result<Box<dyn HidBackend>> {
    Err(Error::PlatformNotSupported(
        "Windows gamepad requires ViGEmBus driver".into()
    ))
}

// Non-Windows stubs
#[cfg(not(target_os = "windows"))]
pub fn create_mouse_backend(_name: &str) -> Result<Box<dyn HidBackend>> {
    Err(Error::PlatformNotSupported("Not on Windows".into()))
}

#[cfg(not(target_os = "windows"))]
pub fn create_keyboard_backend(_name: &str) -> Result<Box<dyn HidBackend>> {
    Err(Error::PlatformNotSupported("Not on Windows".into()))
}

#[cfg(not(target_os = "windows"))]
pub fn create_gamepad_backend(_name: &str) -> Result<Box<dyn HidBackend>> {
    Err(Error::PlatformNotSupported("Not on Windows".into()))
}
