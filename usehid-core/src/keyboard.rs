//! Virtual Keyboard implementation

use crate::error::{Error, Result};
use crate::hid::KeyboardReport;
use crate::platform::HidBackend;
use crate::Device;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

bitflags! {
    /// Keyboard modifier flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
    pub struct Modifiers: u8 {
        const LEFT_CTRL = 0x01;
        const LEFT_SHIFT = 0x02;
        const LEFT_ALT = 0x04;
        const LEFT_GUI = 0x08;
        const RIGHT_CTRL = 0x10;
        const RIGHT_SHIFT = 0x20;
        const RIGHT_ALT = 0x40;
        const RIGHT_GUI = 0x80;
        
        const CTRL = Self::LEFT_CTRL.bits();
        const SHIFT = Self::LEFT_SHIFT.bits();
        const ALT = Self::LEFT_ALT.bits();
        const GUI = Self::LEFT_GUI.bits();
        const CMD = Self::LEFT_GUI.bits();
    }
}

/// Key codes (USB HID usage IDs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Key {
    // Letters
    A = 0x04, B = 0x05, C = 0x06, D = 0x07, E = 0x08, F = 0x09,
    G = 0x0A, H = 0x0B, I = 0x0C, J = 0x0D, K = 0x0E, L = 0x0F,
    M = 0x10, N = 0x11, O = 0x12, P = 0x13, Q = 0x14, R = 0x15,
    S = 0x16, T = 0x17, U = 0x18, V = 0x19, W = 0x1A, X = 0x1B,
    Y = 0x1C, Z = 0x1D,
    
    // Numbers
    Num1 = 0x1E, Num2 = 0x1F, Num3 = 0x20, Num4 = 0x21, Num5 = 0x22,
    Num6 = 0x23, Num7 = 0x24, Num8 = 0x25, Num9 = 0x26, Num0 = 0x27,
    
    // Special keys
    Enter = 0x28,
    Escape = 0x29,
    Backspace = 0x2A,
    Tab = 0x2B,
    Space = 0x2C,
    Minus = 0x2D,
    Equal = 0x2E,
    LeftBracket = 0x2F,
    RightBracket = 0x30,
    Backslash = 0x31,
    Semicolon = 0x33,
    Quote = 0x34,
    Grave = 0x35,
    Comma = 0x36,
    Period = 0x37,
    Slash = 0x38,
    CapsLock = 0x39,
    
    // Function keys
    F1 = 0x3A, F2 = 0x3B, F3 = 0x3C, F4 = 0x3D, F5 = 0x3E, F6 = 0x3F,
    F7 = 0x40, F8 = 0x41, F9 = 0x42, F10 = 0x43, F11 = 0x44, F12 = 0x45,
    
    // Navigation
    PrintScreen = 0x46,
    ScrollLock = 0x47,
    Pause = 0x48,
    Insert = 0x49,
    Home = 0x4A,
    PageUp = 0x4B,
    Delete = 0x4C,
    End = 0x4D,
    PageDown = 0x4E,
    Right = 0x4F,
    Left = 0x50,
    Down = 0x51,
    Up = 0x52,
}

impl Key {
    /// Parse key from string
    pub fn from_str(s: &str) -> Result<Self> {
        let key = match s.to_lowercase().as_str() {
            "a" => Key::A, "b" => Key::B, "c" => Key::C, "d" => Key::D,
            "e" => Key::E, "f" => Key::F, "g" => Key::G, "h" => Key::H,
            "i" => Key::I, "j" => Key::J, "k" => Key::K, "l" => Key::L,
            "m" => Key::M, "n" => Key::N, "o" => Key::O, "p" => Key::P,
            "q" => Key::Q, "r" => Key::R, "s" => Key::S, "t" => Key::T,
            "u" => Key::U, "v" => Key::V, "w" => Key::W, "x" => Key::X,
            "y" => Key::Y, "z" => Key::Z,
            "1" => Key::Num1, "2" => Key::Num2, "3" => Key::Num3,
            "4" => Key::Num4, "5" => Key::Num5, "6" => Key::Num6,
            "7" => Key::Num7, "8" => Key::Num8, "9" => Key::Num9, "0" => Key::Num0,
            "enter" | "return" => Key::Enter,
            "escape" | "esc" => Key::Escape,
            "backspace" => Key::Backspace,
            "tab" => Key::Tab,
            "space" | " " => Key::Space,
            "up" => Key::Up,
            "down" => Key::Down,
            "left" => Key::Left,
            "right" => Key::Right,
            "home" => Key::Home,
            "end" => Key::End,
            "pageup" => Key::PageUp,
            "pagedown" => Key::PageDown,
            "delete" => Key::Delete,
            "insert" => Key::Insert,
            "f1" => Key::F1, "f2" => Key::F2, "f3" => Key::F3, "f4" => Key::F4,
            "f5" => Key::F5, "f6" => Key::F6, "f7" => Key::F7, "f8" => Key::F8,
            "f9" => Key::F9, "f10" => Key::F10, "f11" => Key::F11, "f12" => Key::F12,
            _ => return Err(Error::InvalidKey(s.to_string())),
        };
        Ok(key)
    }
}

/// Virtual keyboard device
pub struct Keyboard {
    backend: Option<Box<dyn HidBackend>>,
    report: KeyboardReport,
    name: String,
    char_map: HashMap<char, (Key, bool)>, // (key, needs_shift)
}

impl Keyboard {
    /// Create a new virtual keyboard
    pub fn new() -> Self {
        Self::with_name("useHID Virtual Keyboard")
    }
    
    /// Create a new virtual keyboard with custom name
    pub fn with_name(name: &str) -> Self {
        let mut kb = Self {
            backend: None,
            report: KeyboardReport::default(),
            name: name.to_string(),
            char_map: HashMap::new(),
        };
        kb.init_char_map();
        kb
    }
    
    fn init_char_map(&mut self) {
        // Lowercase letters
        for c in 'a'..='z' {
            let key = match c {
                'a' => Key::A, 'b' => Key::B, 'c' => Key::C, 'd' => Key::D,
                'e' => Key::E, 'f' => Key::F, 'g' => Key::G, 'h' => Key::H,
                'i' => Key::I, 'j' => Key::J, 'k' => Key::K, 'l' => Key::L,
                'm' => Key::M, 'n' => Key::N, 'o' => Key::O, 'p' => Key::P,
                'q' => Key::Q, 'r' => Key::R, 's' => Key::S, 't' => Key::T,
                'u' => Key::U, 'v' => Key::V, 'w' => Key::W, 'x' => Key::X,
                'y' => Key::Y, 'z' => Key::Z,
                _ => unreachable!(),
            };
            self.char_map.insert(c, (key, false));
            self.char_map.insert(c.to_ascii_uppercase(), (key, true));
        }
        
        // Numbers
        let numbers = [
            ('1', Key::Num1), ('2', Key::Num2), ('3', Key::Num3),
            ('4', Key::Num4), ('5', Key::Num5), ('6', Key::Num6),
            ('7', Key::Num7), ('8', Key::Num8), ('9', Key::Num9), ('0', Key::Num0),
        ];
        for (c, key) in numbers {
            self.char_map.insert(c, (key, false));
        }
        
        // Shifted numbers (symbols)
        let shifted = [
            ('!', Key::Num1), ('@', Key::Num2), ('#', Key::Num3),
            ('$', Key::Num4), ('%', Key::Num5), ('^', Key::Num6),
            ('&', Key::Num7), ('*', Key::Num8), ('(', Key::Num9), (')', Key::Num0),
        ];
        for (c, key) in shifted {
            self.char_map.insert(c, (key, true));
        }
        
        // Other characters
        self.char_map.insert(' ', (Key::Space, false));
        self.char_map.insert('\n', (Key::Enter, false));
        self.char_map.insert('\t', (Key::Tab, false));
        self.char_map.insert('-', (Key::Minus, false));
        self.char_map.insert('_', (Key::Minus, true));
        self.char_map.insert('=', (Key::Equal, false));
        self.char_map.insert('+', (Key::Equal, true));
        self.char_map.insert('[', (Key::LeftBracket, false));
        self.char_map.insert('{', (Key::LeftBracket, true));
        self.char_map.insert(']', (Key::RightBracket, false));
        self.char_map.insert('}', (Key::RightBracket, true));
        self.char_map.insert('\\', (Key::Backslash, false));
        self.char_map.insert('|', (Key::Backslash, true));
        self.char_map.insert(';', (Key::Semicolon, false));
        self.char_map.insert(':', (Key::Semicolon, true));
        self.char_map.insert('\'', (Key::Quote, false));
        self.char_map.insert('"', (Key::Quote, true));
        self.char_map.insert('`', (Key::Grave, false));
        self.char_map.insert('~', (Key::Grave, true));
        self.char_map.insert(',', (Key::Comma, false));
        self.char_map.insert('<', (Key::Comma, true));
        self.char_map.insert('.', (Key::Period, false));
        self.char_map.insert('>', (Key::Period, true));
        self.char_map.insert('/', (Key::Slash, false));
        self.char_map.insert('?', (Key::Slash, true));
    }
    
    /// Press a key
    pub fn press_key(&mut self, key: Key) -> Result<()> {
        // Find empty slot in keys array
        for i in 0..6 {
            if self.report.keys[i] == 0 {
                self.report.keys[i] = key as u8;
                break;
            }
        }
        self.send_report()
    }
    
    /// Release a key
    pub fn release_key(&mut self, key: Key) -> Result<()> {
        for i in 0..6 {
            if self.report.keys[i] == key as u8 {
                self.report.keys[i] = 0;
                break;
            }
        }
        self.send_report()
    }
    
    /// Press modifier(s)
    pub fn press_modifiers(&mut self, mods: Modifiers) -> Result<()> {
        self.report.modifiers |= mods.bits();
        self.send_report()
    }
    
    /// Release modifier(s)
    pub fn release_modifiers(&mut self, mods: Modifiers) -> Result<()> {
        self.report.modifiers &= !mods.bits();
        self.send_report()
    }
    
    /// Tap a key (press and release)
    pub fn tap(&mut self, key: Key) -> Result<()> {
        self.press_key(key)?;
        std::thread::sleep(std::time::Duration::from_millis(10));
        self.release_key(key)
    }
    
    /// Press key combination (e.g., Ctrl+C)
    pub fn press_combo(&mut self, mods: Modifiers, key: Key) -> Result<()> {
        self.press_modifiers(mods)?;
        self.tap(key)?;
        self.release_modifiers(mods)
    }
    
    /// Type a string
    pub fn type_text(&mut self, text: &str) -> Result<()> {
        for c in text.chars() {
            self.type_char(c)?;
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        Ok(())
    }
    
    /// Type a single character
    pub fn type_char(&mut self, c: char) -> Result<()> {
        if let Some(&(key, shift)) = self.char_map.get(&c) {
            if shift {
                self.press_modifiers(Modifiers::SHIFT)?;
            }
            self.tap(key)?;
            if shift {
                self.release_modifiers(Modifiers::SHIFT)?;
            }
            Ok(())
        } else {
            // Skip unknown characters
            Ok(())
        }
    }
    
    /// Release all keys
    pub fn release_all(&mut self) -> Result<()> {
        self.report = KeyboardReport::default();
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

impl Default for Keyboard {
    fn default() -> Self {
        Self::new()
    }
}

impl Device for Keyboard {
    fn create(&mut self) -> Result<()> {
        if self.backend.is_some() {
            return Err(Error::DeviceAlreadyExists);
        }
        
        let backend = crate::platform::create_keyboard_backend(&self.name)?;
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
