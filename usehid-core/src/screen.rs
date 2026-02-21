//! Screen information utilities
//!
//! Provides screen dimensions and mouse position queries.

use crate::error::{Error, Result};

/// Screen dimensions
#[derive(Debug, Clone, Copy, Default)]
pub struct ScreenSize {
    pub width: u32,
    pub height: u32,
}

/// Mouse position
#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// Get the size of the primary screen
pub fn size() -> Result<ScreenSize> {
    platform::get_screen_size()
}

/// Get the current mouse cursor position
pub fn position() -> Result<Position> {
    platform::get_mouse_position()
}

/// Move mouse to absolute coordinates
pub fn move_to(x: i32, y: i32) -> Result<()> {
    platform::move_mouse_to(x, y)
}

#[cfg(target_os = "macos")]
mod platform {
    use super::*;

    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {
        fn CGMainDisplayID() -> u32;
        fn CGDisplayPixelsWide(display: u32) -> usize;
        fn CGDisplayPixelsHigh(display: u32) -> usize;
        fn CGEventCreate(source: *const std::ffi::c_void) -> *mut std::ffi::c_void;
        fn CGEventGetLocation(event: *mut std::ffi::c_void) -> CGPoint;
        fn CGWarpMouseCursorPosition(point: CGPoint) -> i32;
        fn CGAssociateMouseAndMouseCursorPosition(connected: bool) -> i32;
    }
    
    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        fn CFRelease(cf: *const std::ffi::c_void);
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, Default)]
    pub struct CGPoint {
        pub x: f64,
        pub y: f64,
    }

    pub fn get_screen_size() -> Result<ScreenSize> {
        unsafe {
            let display = CGMainDisplayID();
            let width = CGDisplayPixelsWide(display);
            let height = CGDisplayPixelsHigh(display);
            Ok(ScreenSize {
                width: width as u32,
                height: height as u32,
            })
        }
    }

    pub fn get_mouse_position() -> Result<Position> {
        unsafe {
            let event = CGEventCreate(std::ptr::null());
            if event.is_null() {
                return Err(Error::QueryFailed("Failed to create CGEvent".into()));
            }
            let pos = CGEventGetLocation(event);
            CFRelease(event as *const std::ffi::c_void);
            Ok(Position {
                x: pos.x as i32,
                y: pos.y as i32,
            })
        }
    }

    pub fn move_mouse_to(x: i32, y: i32) -> Result<()> {
        unsafe {
            let point = CGPoint { x: x as f64, y: y as f64 };
            let result = CGWarpMouseCursorPosition(point);
            if result != 0 {
                return Err(Error::MoveFailed(format!("CGWarpMouseCursorPosition failed: {}", result)));
            }
            // Re-associate mouse to prevent acceleration issues
            CGAssociateMouseAndMouseCursorPosition(true);
            Ok(())
        }
    }
}

#[cfg(target_os = "linux")]
mod platform {
    use super::*;
    use std::process::Command;

    pub fn get_screen_size() -> Result<ScreenSize> {
        // Use xdpyinfo or xrandr to get screen size
        let output = Command::new("xdpyinfo")
            .output()
            .map_err(|e| Error::QueryFailed(format!("xdpyinfo failed: {}", e)))?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("dimensions:") {
                // Parse: "  dimensions:    1920x1080 pixels"
                if let Some(dims) = line.split_whitespace().nth(1) {
                    let parts: Vec<&str> = dims.split('x').collect();
                    if parts.len() == 2 {
                        let width = parts[0].parse().unwrap_or(0);
                        let height = parts[1].parse().unwrap_or(0);
                        return Ok(ScreenSize { width, height });
                    }
                }
            }
        }
        Err(Error::QueryFailed("Could not parse screen size".into()))
    }

    pub fn get_mouse_position() -> Result<Position> {
        // Use xdotool to get mouse position
        let output = Command::new("xdotool")
            .args(["getmouselocation", "--shell"])
            .output()
            .map_err(|e| Error::QueryFailed(format!("xdotool failed: {}", e)))?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut x = 0;
        let mut y = 0;
        for line in stdout.lines() {
            if let Some(val) = line.strip_prefix("X=") {
                x = val.parse().unwrap_or(0);
            } else if let Some(val) = line.strip_prefix("Y=") {
                y = val.parse().unwrap_or(0);
            }
        }
        Ok(Position { x, y })
    }

    pub fn move_mouse_to(x: i32, y: i32) -> Result<()> {
        let status = Command::new("xdotool")
            .args(["mousemove", &x.to_string(), &y.to_string()])
            .status()
            .map_err(|e| Error::MoveFailed(format!("xdotool failed: {}", e)))?;
        
        if !status.success() {
            return Err(Error::MoveFailed("xdotool mousemove failed".into()));
        }
        Ok(())
    }
}

#[cfg(target_os = "windows")]
mod platform {
    use super::*;

    #[link(name = "user32")]
    extern "system" {
        fn GetSystemMetrics(nIndex: i32) -> i32;
        fn GetCursorPos(lpPoint: *mut POINT) -> i32;
        fn SetCursorPos(x: i32, y: i32) -> i32;
    }

    #[repr(C)]
    struct POINT {
        x: i32,
        y: i32,
    }

    const SM_CXSCREEN: i32 = 0;
    const SM_CYSCREEN: i32 = 1;

    pub fn get_screen_size() -> Result<ScreenSize> {
        unsafe {
            let width = GetSystemMetrics(SM_CXSCREEN);
            let height = GetSystemMetrics(SM_CYSCREEN);
            Ok(ScreenSize {
                width: width as u32,
                height: height as u32,
            })
        }
    }

    pub fn get_mouse_position() -> Result<Position> {
        unsafe {
            let mut point = POINT { x: 0, y: 0 };
            if GetCursorPos(&mut point) == 0 {
                return Err(Error::QueryFailed("GetCursorPos failed".into()));
            }
            Ok(Position {
                x: point.x,
                y: point.y,
            })
        }
    }

    pub fn move_mouse_to(x: i32, y: i32) -> Result<()> {
        unsafe {
            if SetCursorPos(x, y) == 0 {
                return Err(Error::MoveFailed("SetCursorPos failed".into()));
            }
            Ok(())
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
mod platform {
    use super::*;

    pub fn get_screen_size() -> Result<ScreenSize> {
        Err(Error::PlatformNotSupported("screen size query".into()))
    }

    pub fn get_mouse_position() -> Result<Position> {
        Err(Error::PlatformNotSupported("mouse position query".into()))
    }

    pub fn move_mouse_to(_x: i32, _y: i32) -> Result<()> {
        Err(Error::PlatformNotSupported("mouse move_to".into()))
    }
}
