//! Linux HID backend using uhid (/dev/uhid)
//!
//! This requires access to /dev/uhid, typically needs root or uinput group.

use super::HidBackend;
use crate::error::{Error, Result};
use crate::hid::{KEYBOARD_REPORT_DESCRIPTOR, MOUSE_REPORT_DESCRIPTOR, GAMEPAD_REPORT_DESCRIPTOR};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::unix::io::AsRawFd;

// UHID event types
const UHID_CREATE2: u32 = 11;
const UHID_DESTROY: u32 = 1;
const UHID_INPUT2: u32 = 12;

// UHID_CREATE2 structure
#[repr(C)]
struct UhidCreate2 {
    name: [u8; 128],
    phys: [u8; 64],
    uniq: [u8; 64],
    rd_size: u16,
    bus: u16,
    vendor: u32,
    product: u32,
    version: u32,
    country: u32,
    rd_data: [u8; 4096],
}

// UHID event structure
#[repr(C)]
struct UhidEvent {
    event_type: u32,
    data: [u8; 4380], // Union of all event types
}

// UHID_INPUT2 structure
#[repr(C)]
struct UhidInput2 {
    size: u16,
    data: [u8; 4096],
}

const BUS_VIRTUAL: u16 = 0x06;

/// Linux UHID backend
pub struct LinuxUhidBackend {
    file: File,
}

impl LinuxUhidBackend {
    fn new(name: &str, report_descriptor: &[u8], vendor_id: u32, product_id: u32) -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/uhid")
            .map_err(|e| Error::CreateFailed(format!("Failed to open /dev/uhid: {}. Try running as root or add user to 'input' group.", e)))?;
        
        // Create UHID device
        let mut create = UhidCreate2 {
            name: [0; 128],
            phys: [0; 64],
            uniq: [0; 64],
            rd_size: report_descriptor.len() as u16,
            bus: BUS_VIRTUAL,
            vendor: vendor_id,
            product: product_id,
            version: 0,
            country: 0,
            rd_data: [0; 4096],
        };
        
        // Copy name
        let name_bytes = name.as_bytes();
        let name_len = name_bytes.len().min(127);
        create.name[..name_len].copy_from_slice(&name_bytes[..name_len]);
        
        // Copy report descriptor
        let rd_len = report_descriptor.len().min(4096);
        create.rd_data[..rd_len].copy_from_slice(&report_descriptor[..rd_len]);
        
        // Create event
        let mut event = UhidEvent {
            event_type: UHID_CREATE2,
            data: [0; 4380],
        };
        
        // Copy create struct to event data
        unsafe {
            let create_ptr = &create as *const UhidCreate2 as *const u8;
            let create_size = std::mem::size_of::<UhidCreate2>();
            std::ptr::copy_nonoverlapping(
                create_ptr,
                event.data.as_mut_ptr(),
                create_size.min(event.data.len()),
            );
        }
        
        // Write event
        let event_bytes = unsafe {
            std::slice::from_raw_parts(
                &event as *const UhidEvent as *const u8,
                std::mem::size_of::<UhidEvent>(),
            )
        };
        
        let mut f = file.try_clone()?;
        f.write_all(event_bytes)
            .map_err(|e| Error::CreateFailed(format!("Failed to create UHID device: {}", e)))?;
        
        Ok(Self { file })
    }
}

impl HidBackend for LinuxUhidBackend {
    fn send_report(&self, report: &[u8]) -> Result<()> {
        let mut input = UhidInput2 {
            size: report.len() as u16,
            data: [0; 4096],
        };
        
        let len = report.len().min(4096);
        input.data[..len].copy_from_slice(&report[..len]);
        
        let mut event = UhidEvent {
            event_type: UHID_INPUT2,
            data: [0; 4380],
        };
        
        unsafe {
            let input_ptr = &input as *const UhidInput2 as *const u8;
            let input_size = std::mem::size_of::<UhidInput2>();
            std::ptr::copy_nonoverlapping(
                input_ptr,
                event.data.as_mut_ptr(),
                input_size.min(event.data.len()),
            );
        }
        
        let event_bytes = unsafe {
            std::slice::from_raw_parts(
                &event as *const UhidEvent as *const u8,
                std::mem::size_of::<UhidEvent>(),
            )
        };
        
        let mut f = self.file.try_clone()?;
        f.write_all(event_bytes)
            .map_err(|e| Error::SendFailed(format!("Failed to send UHID report: {}", e)))?;
        
        Ok(())
    }
    
    fn destroy(self: Box<Self>) -> Result<()> {
        let event = UhidEvent {
            event_type: UHID_DESTROY,
            data: [0; 4380],
        };
        
        let event_bytes = unsafe {
            std::slice::from_raw_parts(
                &event as *const UhidEvent as *const u8,
                std::mem::size_of::<UhidEvent>(),
            )
        };
        
        let mut f = self.file.try_clone()?;
        f.write_all(event_bytes).ok(); // Best effort
        
        Ok(())
    }
}

pub fn create_mouse_backend(name: &str) -> Result<Box<dyn HidBackend>> {
    Ok(Box::new(LinuxUhidBackend::new(
        name,
        MOUSE_REPORT_DESCRIPTOR,
        0x1234,
        0x0001,
    )?))
}

pub fn create_keyboard_backend(name: &str) -> Result<Box<dyn HidBackend>> {
    Ok(Box::new(LinuxUhidBackend::new(
        name,
        KEYBOARD_REPORT_DESCRIPTOR,
        0x1234,
        0x0002,
    )?))
}

pub fn create_gamepad_backend(name: &str) -> Result<Box<dyn HidBackend>> {
    Ok(Box::new(LinuxUhidBackend::new(
        name,
        GAMEPAD_REPORT_DESCRIPTOR,
        0x1234,
        0x0003,
    )?))
}
