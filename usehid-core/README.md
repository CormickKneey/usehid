# usehid-core

[![Crates.io](https://img.shields.io/crates/v/usehid-core.svg)](https://crates.io/crates/usehid-core)
[![Documentation](https://docs.rs/usehid-core/badge.svg)](https://docs.rs/usehid-core)
[![License](https://img.shields.io/crates/l/usehid-core.svg)](LICENSE)

Cross-platform virtual HID device library for AI agents. Control mouse, keyboard, and gamepad programmatically.

## Features

- 🖱️ **Virtual Mouse** - Move, click, double-click, scroll
- ⌨️ **Virtual Keyboard** - Type text, press keys, key combinations
- 🎮 **Virtual Gamepad** - Joysticks, buttons, triggers
- 🤖 **Agent API** - JSON-based interface for LLM tool-calling
- 🦀 **Pure Rust** - Safe, fast, no unsafe dependencies

## Platform Support

| Platform | Mouse | Keyboard | Gamepad | Backend |
|----------|:-----:|:--------:|:-------:|---------|
| macOS | ✅ | ✅ | ⚠️ | CGEvent |
| Linux | ✅ | ✅ | ✅ | uhid |
| Windows | ✅ | ✅ | ⚠️ | SendInput |

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
usehid-core = "0.1"
```

### For AI Agents (Recommended)

```rust
use usehid_core::AgentHID;

let mut agent = AgentHID::new();

// Mouse actions
agent.execute_json(r#"{"action": "mouse_move", "x": 100, "y": 50}"#);
agent.execute_json(r#"{"action": "mouse_click", "button": "left"}"#);
agent.execute_json(r#"{"action": "mouse_scroll", "delta": -3}"#);

// Keyboard actions
agent.execute_json(r#"{"action": "type", "text": "Hello, World!"}"#);
agent.execute_json(r#"{"action": "key_press", "key": "enter"}"#);
agent.execute_json(r#"{"action": "key_combo", "modifiers": ["ctrl"], "key": "s"}"#);
```

### Direct Device Control

```rust
use usehid_core::{Device, Mouse, Keyboard, MouseButton, Key, Modifiers};

// Mouse
let mut mouse = Mouse::new();
mouse.create()?;
mouse.move_by(100, 50)?;
mouse.click(MouseButton::LEFT)?;
mouse.scroll(-3)?;
mouse.destroy()?;

// Keyboard
let mut keyboard = Keyboard::new();
keyboard.create()?;
keyboard.type_text("Hello!")?;
keyboard.tap(Key::Enter)?;
keyboard.press_combo(Modifiers::CTRL, Key::S)?;
keyboard.destroy()?;
```

## API Reference

### AgentHID Actions

#### Mouse

| Action | Parameters | Description |
|--------|------------|-------------|
| `mouse_move` | `x: i32, y: i32` | Move by relative offset |
| `mouse_click` | `button?: string` | Click (left/right/middle) |
| `mouse_double_click` | `button?: string` | Double click |
| `mouse_down` | `button?: string` | Press button |
| `mouse_up` | `button?: string` | Release button |
| `mouse_scroll` | `delta: i8` | Scroll wheel (+up/-down) |

#### Keyboard

| Action | Parameters | Description |
|--------|------------|-------------|
| `type` | `text: string` | Type a string |
| `key_press` | `key: string` | Press and release key |
| `key_down` | `key: string` | Press key |
| `key_up` | `key: string` | Release key |
| `key_combo` | `modifiers: [string], key: string` | Key combination |

**Modifiers:** `ctrl`, `shift`, `alt`, `cmd`/`meta`/`win`

**Keys:** `a-z`, `0-9`, `enter`, `escape`, `backspace`, `tab`, `space`, `up`, `down`, `left`, `right`, `home`, `end`, `pageup`, `pagedown`, `delete`, `insert`, `f1-f12`

#### Gamepad

| Action | Parameters | Description |
|--------|------------|-------------|
| `gamepad_press` | `button: string` | Press button |
| `gamepad_release` | `button: string` | Release button |
| `gamepad_left_stick` | `x: u8, y: u8` | Left stick (0-255, 128=center) |
| `gamepad_right_stick` | `x: u8, y: u8` | Right stick |
| `gamepad_triggers` | `left: u8, right: u8` | Triggers (0-255) |

**Buttons:** `a`, `b`, `x`, `y`, `lb`, `rb`, `back`, `start`, `guide`, `left_stick`, `right_stick`, `dpad_up`, `dpad_down`, `dpad_left`, `dpad_right`

### Mouse API

```rust
impl Mouse {
    pub fn new() -> Self;
    pub fn with_name(name: &str) -> Self;
    pub fn move_by(&mut self, dx: i32, dy: i32) -> Result<()>;
    pub fn press(&mut self, button: MouseButton) -> Result<()>;
    pub fn release(&mut self, button: MouseButton) -> Result<()>;
    pub fn click(&mut self, button: MouseButton) -> Result<()>;
    pub fn double_click(&mut self, button: MouseButton) -> Result<()>;
    pub fn scroll(&mut self, delta: i8) -> Result<()>;
}
```

### Keyboard API

```rust
impl Keyboard {
    pub fn new() -> Self;
    pub fn with_name(name: &str) -> Self;
    pub fn type_text(&mut self, text: &str) -> Result<()>;
    pub fn press_key(&mut self, key: Key) -> Result<()>;
    pub fn release_key(&mut self, key: Key) -> Result<()>;
    pub fn tap(&mut self, key: Key) -> Result<()>;
    pub fn press_modifiers(&mut self, mods: Modifiers) -> Result<()>;
    pub fn release_modifiers(&mut self, mods: Modifiers) -> Result<()>;
    pub fn press_combo(&mut self, mods: Modifiers, key: Key) -> Result<()>;
    pub fn release_all(&mut self) -> Result<()>;
}
```

### Gamepad API

```rust
impl Gamepad {
    pub fn new() -> Self;
    pub fn with_name(name: &str) -> Self;
    pub fn press(&mut self, button: GamepadButton) -> Result<()>;
    pub fn release(&mut self, button: GamepadButton) -> Result<()>;
    pub fn tap(&mut self, button: GamepadButton) -> Result<()>;
    pub fn set_left_stick(&mut self, x: u8, y: u8) -> Result<()>;
    pub fn set_right_stick(&mut self, x: u8, y: u8) -> Result<()>;
    pub fn set_left_trigger(&mut self, value: u8) -> Result<()>;
    pub fn set_right_trigger(&mut self, value: u8) -> Result<()>;
    pub fn reset(&mut self) -> Result<()>;
}
```

## Examples

```bash
# Basic mouse control
cargo run --example mouse

# Keyboard typing
cargo run --example keyboard

# Agent browsing demo
cargo run --example agent_browse

# Auto-fill login form
cargo run --example auto_login

# Copy & paste
cargo run --example copy_paste

# Browser shortcuts
cargo run --example browser_shortcuts

# Window management
cargo run --example window_mgmt

# Screenshot
cargo run --example screenshot

# Drag and drop
cargo run --example drag_drop

# Mouse drawing
cargo run --example draw_mouse
```

## Permissions

### macOS
Requires **Accessibility** permissions:
1. System Preferences → Security & Privacy → Privacy → Accessibility
2. Add your terminal or application

### Linux
Requires access to `/dev/uhid`:
```bash
sudo chmod 666 /dev/uhid
# Or add user to input group
sudo usermod -aG input $USER
```

### Windows
No special permissions required for mouse/keyboard (uses SendInput API).

## License

MIT License - see [LICENSE](https://github.com/jiusanzhou/usehid/blob/main/LICENSE)

## Links

- [GitHub Repository](https://github.com/jiusanzhou/usehid)
- [Documentation](https://docs.rs/usehid-core)
- [Changelog](https://github.com/jiusanzhou/usehid/releases)
