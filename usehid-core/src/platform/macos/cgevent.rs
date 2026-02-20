//! macOS CGEvent fallback backend
//!
//! Uses CGEvent API for input simulation when IOHIDUserDevice is not available.
//! This requires Accessibility permissions instead of entitlements.

use super::HidBackend;
use crate::error::{Error, Result};
use std::cell::UnsafeCell;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGEventCreateMouseEvent(
        source: *const std::ffi::c_void,
        mouseType: u32,
        mouseCursorPosition: CGPoint,
        mouseButton: u32,
    ) -> *mut std::ffi::c_void;
    
    fn CGEventCreateKeyboardEvent(
        source: *const std::ffi::c_void,
        virtualKey: u16,
        keyDown: bool,
    ) -> *mut std::ffi::c_void;
    
    fn CGEventCreateScrollWheelEvent(
        source: *const std::ffi::c_void,
        units: u32,
        wheelCount: u32,
        wheel1: i32,
    ) -> *mut std::ffi::c_void;
    
    fn CGEventPost(tap: u32, event: *mut std::ffi::c_void);
    fn CFRelease(cf: *const std::ffi::c_void);
    fn CGEventSetFlags(event: *mut std::ffi::c_void, flags: u64);
    fn CGEventGetLocation(event: *mut std::ffi::c_void) -> CGPoint;
    fn CGEventCreate(source: *const std::ffi::c_void) -> *mut std::ffi::c_void;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct CGPoint {
    x: f64,
    y: f64,
}

// CGEventType
const K_CG_EVENT_LEFT_MOUSE_DOWN: u32 = 1;
const K_CG_EVENT_LEFT_MOUSE_UP: u32 = 2;
const K_CG_EVENT_RIGHT_MOUSE_DOWN: u32 = 3;
const K_CG_EVENT_RIGHT_MOUSE_UP: u32 = 4;
const K_CG_EVENT_MOUSE_MOVED: u32 = 5;
const K_CG_EVENT_OTHER_MOUSE_DOWN: u32 = 25;
const K_CG_EVENT_OTHER_MOUSE_UP: u32 = 26;

// CGMouseButton
const K_CG_MOUSE_BUTTON_LEFT: u32 = 0;
const K_CG_MOUSE_BUTTON_RIGHT: u32 = 1;
const K_CG_MOUSE_BUTTON_CENTER: u32 = 2;

// CGEventTapLocation
const K_CG_HID_EVENT_TAP: u32 = 0;

// Scroll units
const K_CG_SCROLL_EVENT_UNIT_LINE: u32 = 1;

// CGEventFlags
const K_CG_EVENT_FLAG_MASK_SHIFT: u64 = 0x00020000;
const K_CG_EVENT_FLAG_MASK_CONTROL: u64 = 0x00040000;
const K_CG_EVENT_FLAG_MASK_ALTERNATE: u64 = 0x00080000;
const K_CG_EVENT_FLAG_MASK_COMMAND: u64 = 0x00100000;

/// Internal state for mouse backend
struct MouseState {
    current_pos: CGPoint,
    buttons: u8,
}

/// CGEvent-based mouse backend
pub struct CGEventMouseBackend {
    state: UnsafeCell<MouseState>,
}

unsafe impl Send for CGEventMouseBackend {}
unsafe impl Sync for CGEventMouseBackend {}

impl CGEventMouseBackend {
    pub fn new() -> Result<Self> {
        // Get current mouse position
        let pos = unsafe {
            let event = CGEventCreate(std::ptr::null());
            if event.is_null() {
                return Err(Error::CreateFailed("Failed to create CGEvent".into()));
       }
            let pos = CGEventGetLocation(event);
            CFRelease(event as *const std::ffi::c_void);
            pos
        };
        
        Ok(Self {
            state: UnsafeCell::new(MouseState {
                current_pos: pos,
                buttons: 0,
            }),
        })
    }
    
    fn post_mouse_event(&self, event_type: u32, button: u32, pos: CGPoint) -> Result<()> {
        unsafe {
            let event = CGEventCreateMouseEvent(
                std::ptr::null(),
                event_type,
                pos,
                button,
            );
            if event.is_null() {
                return Err(Error::SendFailed("Failed to create mouse event".into()));
            }
            CGEventPost(K_CG_HID_EVENT_TAP, event);
            CFRelease(event as *const std::ffi::c_void);
        }
        Ok(())
    }
}

impl HidBackend for CGEventMouseBackend {
    fn send_report(&self, report: &[u8]) -> Result<()> {
        if report.len() < 4 {
            return Err(Error::SendFailed("Invalid mouse report".into()));
        }
        
        let buttons = report[0];
        let dx = report[1] as i8;
        let dy = report[2] as i8;
        let wheel = report[3] as i8;
        
        // SAFETY: We're single-threaded in practice for HID operations
        let state = unsafe { &mut *self.state.get() };
        
        state.current_pos.x += dx as f64;
        state.current_pos.y += dy as f64;
        
        // Clamp to screen bounds (approximate)
        if state.current_pos.x < 0.0 { state.current_pos.x = 0.0; }
        if state.current_pos.y < 0.0 { state.current_pos.y = 0.0; }
        
        let pos = state.current_pos;
        
        // Move mouse
        if dx != 0 || dy != 0 {
            self.post_mouse_event(K_CG_EVENT_MOUSE_MOVED, K_CG_MOUSE_BUTTON_LEFT, pos)?;
        }
        
        // Handle button changes
        let old_buttons = state.buttons;
        state.buttons = buttons;
        
        // Left button
        if (buttons & 0x01) != 0 && (old_buttons & 0x01) == 0 {
            self.post_mouse_event(K_CG_EVENT_LEFT_MOUSE_DOWN, K_CG_MOUSE_BUTTON_LEFT, pos)?;
        } else if (buttons & 0x01) == 0 && (old_buttons & 0x01) != 0 {
            self.post_mouse_event(K_CG_EVENT_LEFT_MOUSE_UP, K_CG_MOUSE_BUTTON_LEFT, pos)?;
        }
        
        // Right button
        if (buttons & 0x02) != 0 && (old_buttons & 0x02) == 0 {
            self.post_mouse_event(K_CG_EVENT_RIGHT_MOUSE_DOWN, K_CG_MOUSE_BUTTON_RIGHT, pos)?;
        } else if (buttons & 0x02) == 0 && (old_buttons & 0x02) != 0 {
            self.post_mouse_event(K_CG_EVENT_RIGHT_MOUSE_UP, K_CG_MOUSE_BUTTON_RIGHT, pos)?;
        }
        
        // Middle button
        if (buttons & 0x04) != 0 && (old_buttons & 0x04) == 0 {
            self.post_mouse_event(K_CG_EVENT_OTHER_MOUSE_DOWN, K_CG_MOUSE_BUTTON_CENTER, pos)?;
        } else if (buttons & 0x04) == 0 && (old_buttons & 0x04) != 0 {
            self.post_mouse_event(K_CG_EVENT_OTHER_MOUSE_UP, K_CG_MOUSE_BUTTON_CENTER, pos)?;
        }
        
        // Scroll wheel
        if wheel != 0 {
            unsafe {
                let event = CGEventCreateScrollWheelEvent(
                    std::ptr::null(),
                    K_CG_SCROLL_EVENT_UNIT_LINE,
                    1,
                    wheel as i32,
                );
                if !event.is_null() {
                    CGEventPost(K_CG_HID_EVENT_TAP, event);
                    CFRelease(event as *const std::ffi::c_void);
                }
            }
        }
        
        Ok(())
    }
    
    fn destroy(self: Box<Self>) -> Result<()> {
        Ok(())
    }
}

/// CGEvent-based keyboard backend
pub struct CGEventKeyboardBackend;

unsafe impl Send for CGEventKeyboardBackend {}
unsafe impl Sync for CGEventKeyboardBackend {}

impl CGEventKeyboardBackend {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
    
    fn get_flags(modifiers: u8) -> u64 {
        let mut flags = 0u64;
        if modifiers & 0x01 != 0 || modifiers & 0x10 != 0 { flags |= K_CG_EVENT_FLAG_MASK_CONTROL; }
        if modifiers & 0x02 != 0 || modifiers & 0x20 != 0 { flags |= K_CG_EVENT_FLAG_MASK_SHIFT; }
        if modifiers & 0x04 != 0 || modifiers & 0x40 != 0 { flags |= K_CG_EVENT_FLAG_MASK_ALTERNATE; }
        if modifiers & 0x08 != 0 || modifiers & 0x80 != 0 { flags |= K_CG_EVENT_FLAG_MASK_COMMAND; }
        flags
    }
    
    // Convert USB HID keycode to macOS virtual keycode
    fn hid_to_virtual_key(hid: u8) -> Option<u16> {
        Some(match hid {
            0x04 => 0x00, // A
            0x05 => 0x0B, // B
            0x06 => 0x08, // C
            0x07 => 0x02, // D
            0x08 => 0x0E, // E
            0x09 => 0x03, // F
            0x0A => 0x05, // G
            0x0B => 0x04, // H
            0x0C => 0x22, // I
            0x0D => 0x26, // J
            0x0E => 0x28, // K
            0x0F => 0x25, // L
            0x10 => 0x2E, // M
            0x11 => 0x2D, // N
            0x12 => 0x1F, // O
            0x13 => 0x23, // P
            0x14 => 0x0C, // Q
            0x15 => 0x0F, // R
            0x16 => 0x01, // S
            0x17 => 0x11, // T
            0x18 => 0x20, // U
            0x19 => 0x09, // V
            0x1A => 0x0D, // W
            0x1B => 0x07, // X
            0x1C => 0x10, // Y
            0x1D => 0x06, // Z
            0x1E => 0x12, // 1
            0x1F => 0x13, // 2
            0x20 => 0x14, // 3
            0x21 => 0x15, // 4
            0x22 => 0x17, // 5
            0x23 => 0x16, // 6
            0x24 => 0x1A, // 7
            0x25 => 0x1C, // 8
            0x26 => 0x19, // 9
            0x27 => 0x1D, // 0
            0x28 => 0x24, // Enter
            0x29 => 0x35, // Escape
            0x2A => 0x33, // Backspace
            0x2B => 0x30, // Tab
            0x2C => 0x31, // Space
            0x2D => 0x1B, // Minus
            0x2E => 0x18, // Equal
            0x2F => 0x21, // Left Bracket
            0x30 => 0x1E, // Right Bracket
            0x31 => 0x2A, // Backslash
            0x33 => 0x29, // Semicolon
            0x34 => 0x27, // Quote
            0x35 => 0x32, // Grave
            0x36 => 0x2B, // Comma
            0x37 => 0x2F, // Period
            0x38 => 0x2C, // Slash
            0x4F => 0x7C, // Right Arrow
            0x50 => 0x7B, // Left Arrow
            0x51 => 0x7D, // Down Arrow
            0x52 => 0x7E, // Up Arrow
            _ => return None,
        })
    }
}

impl HidBackend for CGEventKeyboardBackend {
    fn send_report(&self, report: &[u8]) -> Result<()> {
        if report.len() < 8 {
            return Err(Error::SendFailed("Invalid keyboard report".into()));
        }
        
        let modifiers = report[0];
        // report[1] is reserved
        let keys = &report[2..8];
        
        let flags = Self::get_flags(modifiers);
        
        // Send key events
        for &key in keys {
            if key == 0 { continue; }
            
            if let Some(vk) = Self::hid_to_virtual_key(key) {
                unsafe {
                    // Key down
                    let event = CGEventCreateKeyboardEvent(std::ptr::null(), vk, true);
                    if !event.is_null() {
                        CGEventSetFlags(event, flags);
                        CGEventPost(K_CG_HID_EVENT_TAP, event);
                        CFRelease(event as *const std::ffi::c_void);
                    }
                    
                    // Small delay
                    std::thread::sleep(std::time::Duration::from_millis(5));
                    
                    // Key up
                    let event = CGEventCreateKeyboardEvent(std::ptr::null(), vk, false);
                    if !event.is_null() {
                        CGEventSetFlags(event, flags);
                        CGEventPost(K_CG_HID_EVENT_TAP, event);
                        CFRelease(event as *const std::ffi::c_void);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn destroy(self: Box<Self>) -> Result<()> {
        Ok(())
    }
}

pub fn create_cgevent_mouse_backend() -> Result<Box<dyn HidBackend>> {
    Ok(Box::new(CGEventMouseBackend::new()?))
}

pub fn create_cgevent_keyboard_backend() -> Result<Box<dyn HidBackend>> {
    Ok(Box::new(CGEventKeyboardBackend::new()?))
}
