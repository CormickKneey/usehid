//! Agent-friendly HID interface
//!
//! JSON-based interface for LLM agents to control HID devices.

use crate::error::{Error, Result};
use crate::{Device, Keyboard, Key, Modifiers, Mouse, MouseButton, Gamepad, GamepadButton};
use serde::{Deserialize, Serialize};

/// Action types for agent interface
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum AgentAction {
    // Mouse actions
    MouseMove { x: i32, y: i32 },
    MouseClick { button: Option<String> },
    MouseDoubleClick { button: Option<String> },
    MouseDown { button: Option<String> },
    MouseUp { button: Option<String> },
    MouseScroll { delta: i8 },
    
    // Keyboard actions
    Type { text: String },
    KeyPress { key: String },
    KeyDown { key: String },
    KeyUp { key: String },
    KeyCombo { modifiers: Vec<String>, key: String },
    
    // Gamepad actions
    GamepadPress { button: String },
    GamepadRelease { button: String },
    GamepadLeftStick { x: u8, y: u8 },
    GamepadRightStick { x: u8, y: u8 },
    GamepadTriggers { left: u8, right: u8 },
}

/// Result of an agent action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResult {
    pub success: bool,
    pub error: Option<String>,
}

impl AgentResult {
    fn ok() -> Self {
        Self { success: true, error: None }
    }
    
    fn err(msg: impl Into<String>) -> Self {
        Self { success: false, error: Some(msg.into()) }
    }
}

/// Agent HID controller
pub struct AgentHID {
    mouse: Mouse,
    keyboard: Keyboard,
    gamepad: Gamepad,
    mouse_created: bool,
    keyboard_created: bool,
    gamepad_created: bool,
}

impl AgentHID {
    /// Create a new agent HID controller
    pub fn new() -> Self {
        Self {
            mouse: Mouse::new(),
            keyboard: Keyboard::new(),
            gamepad: Gamepad::new(),
            mouse_created: false,
            keyboard_created: false,
            gamepad_created: false,
        }
    }
    
    /// Execute an action from JSON
    pub fn execute_json(&mut self, json: &str) -> AgentResult {
        match serde_json::from_str::<AgentAction>(json) {
            Ok(action) => self.execute(action),
            Err(e) => AgentResult::err(format!("Invalid JSON: {}", e)),
        }
    }
    
    /// Execute an action
    pub fn execute(&mut self, action: AgentAction) -> AgentResult {
        match action {
            // Mouse
            AgentAction::MouseMove { x, y } => self.mouse_move(x, y),
            AgentAction::MouseClick { button } => self.mouse_click(button),
            AgentAction::MouseDoubleClick { button } => self.mouse_double_click(button),
            AgentAction::MouseDown { button } => self.mouse_down(button),
            AgentAction::MouseUp { button } => self.mouse_up(button),
            AgentAction::MouseScroll { delta } => self.mouse_scroll(delta),
            
            // Keyboard
            AgentAction::Type { text } => self.type_text(&text),
            AgentAction::KeyPress { key } => self.key_press(&key),
            AgentAction::KeyDown { key } => self.key_down(&key),
            AgentAction::KeyUp { key } => self.key_up(&key),
            AgentAction::KeyCombo { modifiers, key } => self.key_combo(&modifiers, &key),
            
            // Gamepad
            AgentAction::GamepadPress { button } => self.gamepad_press(&button),
            AgentAction::GamepadRelease { button } => self.gamepad_release(&button),
            AgentAction::GamepadLeftStick { x, y } => self.gamepad_left_stick(x, y),
            AgentAction::GamepadRightStick { x, y } => self.gamepad_right_stick(x, y),
            AgentAction::GamepadTriggers { left, right } => self.gamepad_triggers(left, right),
        }
    }
    
    fn ensure_mouse(&mut self) -> Result<()> {
        if !self.mouse_created {
            self.mouse.create()?;
            self.mouse_created = true;
        }
        Ok(())
    }
    
    fn ensure_keyboard(&mut self) -> Result<()> {
        if !self.keyboard_created {
            self.keyboard.create()?;
            self.keyboard_created = true;
        }
        Ok(())
    }
    
    fn ensure_gamepad(&mut self) -> Result<()> {
        if !self.gamepad_created {
            self.gamepad.create()?;
            self.gamepad_created = true;
        }
        Ok(())
    }
    
    fn parse_mouse_button(s: Option<String>) -> MouseButton {
        match s.as_deref() {
            Some("right") => MouseButton::RIGHT,
            Some("middle") => MouseButton::MIDDLE,
            _ => MouseButton::LEFT,
        }
    }
    
    fn parse_gamepad_button(s: &str) -> Result<GamepadButton> {
        let btn = match s.to_lowercase().as_str() {
            "a" => GamepadButton::A,
            "b" => GamepadButton::B,
            "x" => GamepadButton::X,
            "y" => GamepadButton::Y,
            "lb" | "left_bumper" => GamepadButton::LB,
            "rb" | "right_bumper" => GamepadButton::RB,
            "back" | "select" => GamepadButton::BACK,
            "start" => GamepadButton::START,
            "guide" | "home" => GamepadButton::GUIDE,
            "left_stick" | "ls" => GamepadButton::LEFT_STICK,
            "right_stick" | "rs" => GamepadButton::RIGHT_STICK,
            "dpad_up" | "up" => GamepadButton::DPAD_UP,
            "dpad_down" | "down" => GamepadButton::DPAD_DOWN,
            "dpad_left" | "left" => GamepadButton::DPAD_LEFT,
            "dpad_right" | "right" => GamepadButton::DPAD_RIGHT,
            _ => return Err(Error::InvalidAction(format!("Unknown gamepad button: {}", s))),
        };
        Ok(btn)
    }
    
    fn parse_modifiers(mods: &[String]) -> Modifiers {
        let mut result = Modifiers::empty();
        for m in mods {
            match m.to_lowercase().as_str() {
                "ctrl" | "control" => result |= Modifiers::CTRL,
                "shift" => result |= Modifiers::SHIFT,
                "alt" | "option" => result |= Modifiers::ALT,
                "cmd" | "command" | "meta" | "gui" | "win" => result |= Modifiers::CMD,
                _ => {}
            }
        }
        result
    }
    
    // Mouse actions
    fn mouse_move(&mut self, x: i32, y: i32) -> AgentResult {
        if let Err(e) = self.ensure_mouse() {
            return AgentResult::err(e.to_string());
        }
        match self.mouse.move_by(x, y) {
            Ok(_) => AgentResult::ok(),
            Err(e) => AgentResult::err(e.to_string()),
        }
    }
    
    fn mouse_click(&mut self, button: Option<String>) -> AgentResult {
        if let Err(e) = self.ensure_mouse() {
            return AgentResult::err(e.to_string());
        }
        let btn = Self::parse_mouse_button(button);
        match self.mouse.click(btn) {
            Ok(_) => AgentResult::ok(),
            Err(e) => AgentResult::err(e.to_string()),
        }
    }
    
    fn mouse_double_click(&mut self, button: Option<String>) -> AgentResult {
        if let Err(e) = self.ensure_mouse() {
            return AgentResult::err(e.to_string());
        }
        let btn = Self::parse_mouse_button(button);
        match self.mouse.double_click(btn) {
            Ok(_) => AgentResult::ok(),
            Err(e) => AgentResult::err(e.to_string()),
        }
    }
    
    fn mouse_down(&mut self, button: Option<String>) -> AgentResult {
        if let Err(e) = self.ensure_mouse() {
            return AgentResult::err(e.to_string());
        }
        let btn = Self::parse_mouse_button(button);
        match self.mouse.press(btn) {
            Ok(_) => AgentResult::ok(),
            Err(e) => AgentResult::err(e.to_string()),
        }
    }
    
    fn mouse_up(&mut self, button: Option<String>) -> AgentResult {
        if let Err(e) = self.ensure_mouse() {
            return AgentResult::err(e.to_string());
        }
        let btn = Self::parse_mouse_button(button);
        match self.mouse.release(btn) {
            Ok(_) => AgentResult::ok(),
            Err(e) => AgentResult::err(e.to_string()),
        }
    }
    
    fn mouse_scroll(&mut self, delta: i8) -> AgentResult {
        if let Err(e) = self.ensure_mouse() {
            return AgentResult::err(e.to_string());
        }
        match self.mouse.scroll(delta) {
            Ok(_) => AgentResult::ok(),
            Err(e) => AgentResult::err(e.to_string()),
        }
    }
    
    // Keyboard actions
    fn type_text(&mut self, text: &str) -> AgentResult {
        if let Err(e) = self.ensure_keyboard() {
            return AgentResult::err(e.to_string());
        }
        match self.keyboard.type_text(text) {
            Ok(_) => AgentResult::ok(),
            Err(e) => AgentResult::err(e.to_string()),
        }
    }
    
    fn key_press(&mut self, key: &str) -> AgentResult {
        if let Err(e) = self.ensure_keyboard() {
            return AgentResult::err(e.to_string());
        }
        match Key::from_str(key) {
            Ok(k) => match self.keyboard.tap(k) {
                Ok(_) => AgentResult::ok(),
                Err(e) => AgentResult::err(e.to_string()),
            },
            Err(e) => AgentResult::err(e.to_string()),
        }
    }
    
    fn key_down(&mut self, key: &str) -> AgentResult {
        if let Err(e) = self.ensure_keyboard() {
            return AgentResult::err(e.to_string());
        }
        match Key::from_str(key) {
            Ok(k) => match self.keyboard.press_key(k) {
                Ok(_) => AgentResult::ok(),
                Err(e) => AgentResult::err(e.to_string()),
            },
            Err(e) => AgentResult::err(e.to_string()),
        }
    }
    
    fn key_up(&mut self, key: &str) -> AgentResult {
        if let Err(e) = self.ensure_keyboard() {
            return AgentResult::err(e.to_string());
        }
        match Key::from_str(key) {
            Ok(k) => match self.keyboard.release_key(k) {
                Ok(_) => AgentResult::ok(),
                Err(e) => AgentResult::err(e.to_string()),
            },
            Err(e) => AgentResult::err(e.to_string()),
        }
    }
    
    fn key_combo(&mut self, modifiers: &[String], key: &str) -> AgentResult {
        if let Err(e) = self.ensure_keyboard() {
            return AgentResult::err(e.to_string());
        }
        let mods = Self::parse_modifiers(modifiers);
        match Key::from_str(key) {
            Ok(k) => match self.keyboard.press_combo(mods, k) {
                Ok(_) => AgentResult::ok(),
                Err(e) => AgentResult::err(e.to_string()),
            },
            Err(e) => AgentResult::err(e.to_string()),
        }
    }
    
    // Gamepad actions
    fn gamepad_press(&mut self, button: &str) -> AgentResult {
        if let Err(e) = self.ensure_gamepad() {
            return AgentResult::err(e.to_string());
        }
        match Self::parse_gamepad_button(button) {
            Ok(btn) => match self.gamepad.press(btn) {
                Ok(_) => AgentResult::ok(),
                Err(e) => AgentResult::err(e.to_string()),
            },
            Err(e) => AgentResult::err(e.to_string()),
        }
    }
    
    fn gamepad_release(&mut self, button: &str) -> AgentResult {
        if let Err(e) = self.ensure_gamepad() {
            return AgentResult::err(e.to_string());
        }
        match Self::parse_gamepad_button(button) {
            Ok(btn) => match self.gamepad.release(btn) {
                Ok(_) => AgentResult::ok(),
                Err(e) => AgentResult::err(e.to_string()),
            },
            Err(e) => AgentResult::err(e.to_string()),
        }
    }
    
    fn gamepad_left_stick(&mut self, x: u8, y: u8) -> AgentResult {
        if let Err(e) = self.ensure_gamepad() {
            return AgentResult::err(e.to_string());
        }
        match self.gamepad.set_left_stick(x, y) {
            Ok(_) => AgentResult::ok(),
            Err(e) => AgentResult::err(e.to_string()),
        }
    }
    
    fn gamepad_right_stick(&mut self, x: u8, y: u8) -> AgentResult {
        if let Err(e) = self.ensure_gamepad() {
            return AgentResult::err(e.to_string());
        }
        match self.gamepad.set_right_stick(x, y) {
            Ok(_) => AgentResult::ok(),
            Err(e) => AgentResult::err(e.to_string()),
        }
    }
    
    fn gamepad_triggers(&mut self, left: u8, right: u8) -> AgentResult {
        if let Err(e) = self.ensure_gamepad() {
            return AgentResult::err(e.to_string());
        }
        if let Err(e) = self.gamepad.set_left_trigger(left) {
            return AgentResult::err(e.to_string());
        }
        match self.gamepad.set_right_trigger(right) {
            Ok(_) => AgentResult::ok(),
            Err(e) => AgentResult::err(e.to_string()),
        }
    }
}

impl Default for AgentHID {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AgentHID {
    fn drop(&mut self) {
        if self.mouse_created {
            let _ = self.mouse.destroy();
        }
        if self.keyboard_created {
            let _ = self.keyboard.destroy();
        }
        if self.gamepad_created {
            let _ = self.gamepad.destroy();
        }
    }
}
