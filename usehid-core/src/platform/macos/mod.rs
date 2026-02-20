//! macOS HID backend using IOHIDUserDevice with CGEvent fallback
//!
//! First tries IOHIDUserDevice (requires entitlements).
//! Falls back to CGEvent API (requires Accessibility permissions).

use super::HidBackend;
use crate::error::{Error, Result};
use crate::hid::{KEYBOARD_REPORT_DESCRIPTOR, MOUSE_REPORT_DESCRIPTOR, GAMEPAD_REPORT_DESCRIPTOR};
use std::ffi::CString;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

mod cgevent;
use cgevent::*;

// IOKit bindings (simplified)
#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOHIDUserDeviceCreate(
        allocator: *const std::ffi::c_void,
        properties: *const std::ffi::c_void,
    ) -> *mut std::ffi::c_void;
    
    fn IOHIDUserDeviceScheduleWithRunLoop(
        device: *mut std::ffi::c_void,
        runLoop: *const std::ffi::c_void,
        runLoopMode: *const std::ffi::c_void,
    );
    
    fn IOHIDUserDeviceHandleReport(
        device: *mut std::ffi::c_void,
        report: *const u8,
        reportLength: usize,
    ) -> i32;
    
    fn CFRelease(cf: *const std::ffi::c_void);
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFRunLoopGetCurrent() -> *const std::ffi::c_void;
    static kCFRunLoopDefaultMode: *const std::ffi::c_void;
    
    fn CFDictionaryCreateMutable(
        allocator: *const std::ffi::c_void,
        capacity: isize,
        keyCallBacks: *const std::ffi::c_void,
        valueCallBacks: *const std::ffi::c_void,
    ) -> *mut std::ffi::c_void;
    
    fn CFDictionarySetValue(
        dict: *mut std::ffi::c_void,
        key: *const std::ffi::c_void,
        value: *const std::ffi::c_void,
    );
    
    fn CFStringCreateWithCString(
        allocator: *const std::ffi::c_void,
        cStr: *const i8,
        encoding: u32,
    ) -> *const std::ffi::c_void;
    
    fn CFDataCreate(
        allocator: *const std::ffi::c_void,
        bytes: *const u8,
        length: isize,
    ) -> *const std::ffi::c_void;
    
    fn CFNumberCreate(
        allocator: *const std::ffi::c_void,
        theType: isize,
        valuePtr: *const std::ffi::c_void,
    ) -> *const std::ffi::c_void;
    
    static kCFTypeDictionaryKeyCallBacks: *const std::ffi::c_void;
    static kCFTypeDictionaryValueCallBacks: *const std::ffi::c_void;
}

const K_CF_STRING_ENCODING_UTF8: u32 = 0x08000100;
const K_CF_NUMBER_SINT32_TYPE: isize = 3;

/// macOS HID device backend using IOHIDUserDevice
pub struct MacOSHidBackend {
    device: *mut std::ffi::c_void,
    _running: Arc<AtomicBool>,
}

unsafe impl Send for MacOSHidBackend {}
unsafe impl Sync for MacOSHidBackend {}

impl MacOSHidBackend {
    fn new(name: &str, report_descriptor: &[u8], vendor_id: u32, product_id: u32) -> Result<Self> {
        unsafe {
            let dict = CFDictionaryCreateMutable(
                std::ptr::null(),
                0,
                kCFTypeDictionaryKeyCallBacks,
                kCFTypeDictionaryValueCallBacks,
            );
            
            if dict.is_null() {
                return Err(Error::CreateFailed("Failed to create properties dictionary".into()));
            }
            
            // Set product name
            let name_key = CFStringCreateWithCString(
                std::ptr::null(),
                b"Product\0".as_ptr() as *const i8,
                K_CF_STRING_ENCODING_UTF8,
            );
            let name_cstring = CString::new(name).unwrap();
            let name_val = CFStringCreateWithCString(
                std::ptr::null(),
                name_cstring.as_ptr(),
                K_CF_STRING_ENCODING_UTF8,
            );
            CFDictionarySetValue(dict, name_key, name_val);
            
            // Set vendor ID
            let vendor_key = CFStringCreateWithCString(
                std::ptr::null(),
                b"VendorID\0".as_ptr() as *const i8,
                K_CF_STRING_ENCODING_UTF8,
            );
            let vendor_val = CFNumberCreate(
                std::ptr::null(),
                K_CF_NUMBER_SINT32_TYPE,
                &vendor_id as *const u32 as *const std::ffi::c_void,
            );
            CFDictionarySetValue(dict, vendor_key, vendor_val);
            
            // Set product ID
            let product_key = CFStringCreateWithCString(
                std::ptr::null(),
                b"ProductID\0".as_ptr() as *const i8,
                K_CF_STRING_ENCODING_UTF8,
            );
            let product_val = CFNumberCreate(
                std::ptr::null(),
                K_CF_NUMBER_SINT32_TYPE,
                &product_id as *const u32 as *const std::ffi::c_void,
            );
            CFDictionarySetValue(dict, product_key, product_val);
            
            // Set report descriptor
            let desc_key = CFStringCreateWithCString(
                std::ptr::null(),
                b"ReportDescriptor\0".as_ptr() as *const i8,
                K_CF_STRING_ENCODING_UTF8,
            );
            let desc_val = CFDataCreate(
                std::ptr::null(),
                report_descriptor.as_ptr(),
                report_descriptor.len() as isize,
            );
            CFDictionarySetValue(dict, desc_key, desc_val);
            
            // Create device
            let device = IOHIDUserDeviceCreate(std::ptr::null(), dict as *const std::ffi::c_void);
            
            if device.is_null() {
                CFRelease(dict as *const std::ffi::c_void);
                return Err(Error::CreateFailed(
                    "IOHIDUserDevice requires entitlements".into()
                ));
            }
            
            // Schedule with run loop
            IOHIDUserDeviceScheduleWithRunLoop(
                device,
                CFRunLoopGetCurrent(),
                kCFRunLoopDefaultMode,
            );
            
            Ok(Self {
                device,
                _running: Arc::new(AtomicBool::new(true)),
            })
        }
    }
}

impl HidBackend for MacOSHidBackend {
    fn send_report(&self, report: &[u8]) -> Result<()> {
        unsafe {
            let result = IOHIDUserDeviceHandleReport(self.device, report.as_ptr(), report.len());
            if result != 0 {
                return Err(Error::SendFailed(format!("IOHIDUserDeviceHandleReport failed: {}", result)));
            }
        }
        Ok(())
    }
    
    fn destroy(self: Box<Self>) -> Result<()> {
        unsafe {
            if !self.device.is_null() {
                CFRelease(self.device as *const std::ffi::c_void);
            }
        }
        Ok(())
    }
}

/// Try IOHIDUserDevice first, fall back to CGEvent
pub fn create_mouse_backend(_name: &str) -> Result<Box<dyn HidBackend>> {
    // Try IOHIDUserDevice first
    match MacOSHidBackend::new(_name, MOUSE_REPORT_DESCRIPTOR, 0x1234, 0x0001) {
        Ok(backend) => Ok(Box::new(backend)),
        Err(_) => {
            // Fall back to CGEvent
            tracing::info!("IOHIDUserDevice not available, using CGEvent fallback");
            create_cgevent_mouse_backend()
        }
    }
}

pub fn create_keyboard_backend(_name: &str) -> Result<Box<dyn HidBackend>> {
    match MacOSHidBackend::new(_name, KEYBOARD_REPORT_DESCRIPTOR, 0x1234, 0x0002) {
        Ok(backend) => Ok(Box::new(backend)),
        Err(_) => {
            tracing::info!("IOHIDUserDevice not available, using CGEvent fallback");
            create_cgevent_keyboard_backend()
        }
    }
}

pub fn create_gamepad_backend(name: &str) -> Result<Box<dyn HidBackend>> {
    // Gamepad only works with IOHIDUserDevice, no CGEvent fallback
    Ok(Box::new(MacOSHidBackend::new(
        name,
        GAMEPAD_REPORT_DESCRIPTOR,
        0x1234,
        0x0003,
    )?))
}
