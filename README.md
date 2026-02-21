# 🎮 useHID

> Cross-platform virtual HID device library for AI agents

[![Crates.io](https://img.shields.io/crates/v/usehid.svg)](https://crates.io/crates/usehid)
[![PyPI](https://img.shields.io/pypi/v/usehid.svg)](https://pypi.org/project/usehid/)
[![npm](https://img.shields.io/npm/v/usehid.svg)](https://www.npmjs.com/package/usehid)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey.svg)](#platform-support)

**useHID** lets AI agents control computers like humans do — moving the mouse, typing on the keyboard, and pressing gamepad buttons. Built in Rust with bindings for Python, TypeScript, and Go.

```python
from usehid import AgentHID

agent = AgentHID()
agent.execute({"action": "key_combo", "modifiers": ["cmd"], "key": "space"})
agent.execute({"action": "type", "text": "Google Chrome"})
agent.execute({"action": "key_press", "key": "enter"})
```

---

## ✨ Features

- 🖱️ **Virtual Mouse** — Move, click, double-click, drag, scroll
- ⌨️ **Virtual Keyboard** — Type text, press keys, key combinations
- 🎮 **Virtual Gamepad** — Joysticks, buttons, triggers
- 🤖 **Agent-Ready API** — Simple JSON interface for LLM agents
- 🦀 **Pure Rust Core** — Safe, fast, zero-copy where possible
- 🐍 **Multi-language** — Python, TypeScript, Go bindings

---

## 🚀 Quick Start

### For AI Agents (Recommended)

The `AgentHID` class provides a JSON-based interface perfect for LLM tool-calling:

```python
from usehid import AgentHID

agent = AgentHID()

# Query screen info
result = agent.execute({"action": "size"})        # Returns: {width: 1920, height: 1080}
result = agent.execute({"action": "position"})    # Returns: {x: 100, y: 200}

# Mouse actions
agent.execute({"action": "mouse_move_to", "x": 500, "y": 300})  # Absolute move
agent.execute({"action": "mouse_move", "x": 100, "y": 50})      # Relative move
agent.execute({"action": "mouse_click", "button": "left"})
agent.execute({"action": "mouse_drag_to", "x": 600, "y": 400})  # Drag to position
agent.execute({"action": "mouse_scroll", "delta": -3})

# Keyboard actions
agent.execute({"action": "type", "text": "Hello, World!"})
agent.execute({"action": "key_press", "key": "enter"})
agent.execute({"action": "key_combo", "modifiers": ["ctrl", "shift"], "key": "s"})
```

### Direct Functions (Python)

```python
import usehid

# Screen info
width, height = usehid.size()       # Get screen dimensions
x, y = usehid.position()            # Get mouse position
usehid.move_to(500, 300)            # Move mouse to absolute position
```

### Direct Device Control

For more control, use the device classes directly:

```python
from usehid import Mouse, Keyboard

mouse = Mouse()
mouse.move_by(100, 50)
mouse.click("left")
mouse.drag(200, 100)   # Drag by relative offset
mouse.scroll(-3)

keyboard = Keyboard()
keyboard.type_text("Hello!")
keyboard.combo(["cmd"], "s")  # Cmd+S to save
```

---

## 📋 Supported Actions

### Screen/Query

| Action | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `size` | — | `width`, `height` | Get screen dimensions |
| `position` | — | `x`, `y` | Get current mouse position |

### Mouse

| Action | Parameters | Description |
|--------|------------|-------------|
| `mouse_move` | `x`, `y`, `duration`?, `tween`? | Move by relative offset |
| `mouse_move_to` | `x`, `y`, `duration`?, `tween`? | Move to absolute position |
| `mouse_click` | `button`? | Click (left/right/middle) |
| `mouse_double_click` | `button`? | Double click |
| `mouse_down` | `button`? | Press and hold |
| `mouse_up` | `button`? | Release |
| `mouse_scroll` | `delta` | Scroll wheel (+up/-down) |
| `mouse_drag` | `x`, `y`, `button`?, `duration`?, `tween`? | Drag by relative offset |
| `mouse_drag_to` | `x`, `y`, `button`?, `duration`?, `tween`? | Drag to absolute position |

### Keyboard

| Action | Parameters | Description |
|--------|------------|-------------|
| `type` | `text`, `interval`? | Type a string (interval in ms) |
| `key_press` | `key` | Press and release a key |
| `key_down` | `key` | Press and hold |
| `key_up` | `key` | Release |
| `key_combo` | `modifiers[]`, `key` | Key combination |

**Supported modifiers:** `ctrl`, `shift`, `alt`, `cmd`/`meta`/`win`

**Supported keys:** `a-z`, `0-9`, `enter`, `escape`, `backspace`, `tab`, `space`, `up`, `down`, `left`, `right`, `home`, `end`, `pageup`, `pagedown`, `delete`, `insert`, `f1-f12`

### Tween Functions

For smooth movement animations, use the `tween` parameter:

| Tween | Description |
|-------|-------------|
| `linear` | Constant speed (default if duration=0) |
| `ease_in` / `ease_in_quad` | Slow start |
| `ease_out` / `ease_out_quad` | Slow end |
| `ease_in_out` / `ease_in_out_quad` | Slow start and end (default) |
| `ease_in_cubic` | Cubic slow start |
| `ease_out_cubic` | Cubic slow end |
| `ease_in_out_cubic` | Cubic slow start and end |
| `ease_out_elastic` | Elastic bounce effect |
| `ease_out_bounce` | Bounce effect |

### Failsafe

Emergency stop mechanism - moving mouse to any screen corner triggers failsafe and blocks further actions.

| Action | Returns | Description |
|--------|---------|-------------|
| `failsafe_status` | `enabled`, `triggered` | Check failsafe state |
| `failsafe_enable` | — | Enable failsafe (default) |
| `failsafe_disable` | — | Disable failsafe |
| `failsafe_reset` | — | Reset triggered state |

**Usage:**
```python
# Check status
result = agent.execute({"action": "failsafe_status"})
# {success: true, enabled: true, triggered: false}

# If triggered, reset to continue
agent.execute({"action": "failsafe_reset"})

# Disable for specific operations (use with caution!)
agent.execute({"action": "failsafe_disable"})
# ... operations ...
agent.execute({"action": "failsafe_enable"})
```

### Gamepad

| Action | Parameters | Description |
|--------|------------|-------------|
| `gamepad_press` | `button` | Press button |
| `gamepad_release` | `button` | Release button |
| `gamepad_left_stick` | `x`, `y` | Left stick (0-255) |
| `gamepad_right_stick` | `x`, `y` | Right stick (0-255) |
| `gamepad_triggers` | `left`, `right` | Triggers (0-255) |

**Buttons:** `a`, `b`, `x`, `y`, `lb`, `rb`, `back`, `start`, `guide`, `left_stick`, `right_stick`, `dpad_up`, `dpad_down`, `dpad_left`, `dpad_right`

---

## 🖥️ Platform Support

| Platform | Mouse | Keyboard | Gamepad | Backend |
|----------|:-----:|:--------:|:-------:|---------|
| **macOS** | ✅ | ✅ | ⚠️ | CGEvent (fallback) / IOHIDUserDevice |
| **Linux** | ✅ | ✅ | ✅ | uhid (`/dev/uhid`) |
| **Windows** | ✅ | ✅ | ⚠️ | SendInput API |

> ⚠️ Gamepad notes:
> - macOS: Requires code signing with `com.apple.developer.hid.virtual.device` entitlement
> - Windows: Requires [ViGEmBus](https://github.com/ViGEm/ViGEmBus) driver

### macOS Permissions

useHID uses **CGEvent** by default, which requires **Accessibility** permissions:

1. Go to **System Preferences → Security & Privacy → Privacy → Accessibility**
2. Add your terminal app or the application using useHID

### Windows

No special permissions required. Uses Win32 `SendInput` API for mouse and keyboard.

---

## 📦 Installation

### Rust

```toml
[dependencies]
usehid = "0.1"
```

### Python

```bash
pip install usehid
```

### TypeScript / Node.js

```bash
npm install usehid
```

### Go

```bash
go get go.zoe.im/usehid@latest
```

```go
import usehid "go.zoe.im/usehid"
```

---

## 🧪 Examples

### Open Chrome and Browse (Agent Test)

```bash
cargo run --release -p usehid --example agent_browse
```

This example demonstrates an AI agent:
1. Opening Spotlight with `Cmd+Space`
2. Typing "Google Chrome" and pressing Enter
3. Opening a new tab with `Cmd+T`
4. Navigating to github.com
5. Scrolling the page
6. Performing a search

### Basic Mouse Control

```bash
cargo run --release -p usehid --example mouse
```

### Keyboard Typing

```bash
cargo run --release -p usehid --example keyboard
```

---

## 🏗️ Architecture

```
usehid/
├── usehid/          # Rust core library
│   ├── src/
│   │   ├── lib.rs        # Public API
│   │   ├── mouse.rs      # Virtual mouse
│   │   ├── keyboard.rs   # Virtual keyboard
│   │   ├── gamepad.rs    # Virtual gamepad
│   │   ├── agent.rs      # JSON API for agents
│   │   ├── hid.rs        # HID report descriptors
│   │   └── platform/     # Platform backends
│   │       ├── macos/    # CGEvent + IOHIDUserDevice
│   │       ├── linux.rs  # uhid
│   │       └── windows.rs
│   └── examples/
├── usehid-python/        # Python bindings (PyO3)
├── usehid-node/          # Node.js bindings (napi-rs)
└── usehid-go/            # Go bindings (CGO)
```

---

## 🔧 Building from Source

```bash
# Clone the repository
git clone https://github.com/jiusanzhou/usehid
cd usehid

# Build the core library
cargo build --release

# Run tests
cargo test

# Build Python bindings
cd usehid-python
maturin develop --release
```

---

## 🤖 OpenClaw Integration

useHID can be integrated with [OpenClaw](https://github.com/openclaw/openclaw) to give AI agents full control over your computer.

### Quick Setup

1. **Install useHID:**
```bash
pip install usehid
```

2. **Grant Permissions (macOS):**
System Preferences → Security & Privacy → Privacy → Accessibility

3. **Copy the skill to OpenClaw:**
```bash
cp -r skills/usehid ~/.openclaw/skills/
```

4. **Use it:**
```
You: Click the Chrome icon on the dock
OpenClaw: [executes mouse_click at dock position]

You: Type "github.com" and press enter  
OpenClaw: [types text and presses enter]

You: Scroll down slowly
OpenClaw: [executes mouse_scroll multiple times]
```

### Example Commands

| User Request | useHID Action |
|--------------|---------------|
| "Click here" | `{"action": "mouse_click"}` |
| "Type hello world" | `{"action": "type", "text": "hello world"}` |
| "Press Enter" | `{"action": "key_press", "key": "enter"}` |
| "Save the file" | `{"action": "key_combo", "modifiers": ["cmd"], "key": "s"}` |
| "Scroll down" | `{"action": "mouse_scroll", "delta": -3}` |
| "Select all and copy" | Two actions: Cmd+A then Cmd+C |

See [skills/usehid/SKILL.md](skills/usehid/SKILL.md) for full documentation.

---

## 🤝 Use Cases

- **AI Agents** — Let LLMs control desktop applications
- **Browser Automation** — Alternative to browser-specific APIs
- **Game Bots** — Gamepad simulation for testing
- **Accessibility Tools** — Alternative input methods
- **Testing** — Automated UI testing without browser dependencies

---

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

## 🙏 Acknowledgments

- Inspired by [foohid](https://github.com/unbit/foohid) for macOS virtual HID
- Agent API design inspired by [browser-use](https://github.com/browser-use/browser-use)

---

<p align="center">
  Made with 🦀 by <a href="https://github.com/jiusanzhou">Zoe</a>
</p>
