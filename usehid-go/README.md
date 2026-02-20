# useHID Go Bindings

[![Go Reference](https://pkg.go.dev/badge/go.zoe.im/usehid-go.svg)](https://pkg.go.dev/go.zoe.im/usehid-go)

Cross-platform virtual HID device library for Go. Control mouse, keyboard, and gamepad programmatically.

## Installation

```bash
go get go.zoe.im/usehid-go@latest
```

## Quick Start

```go
package main

import (
    usehid "go.zoe.im/usehid-go"
)

func main() {
    // Create AgentHID for AI agent usage
    agent := usehid.NewAgentHID()
    defer agent.Close()

    // Mouse operations
    agent.Execute(map[string]interface{}{
        "action": "mouse_move",
        "x":      100,
        "y":      50,
    })

    agent.Execute(map[string]interface{}{
        "action": "mouse_click",
        "button": "left",
    })

    agent.Execute(map[string]interface{}{
        "action": "mouse_scroll",
        "delta":  -3, // scroll down
    })

    // Keyboard operations
    agent.Execute(map[string]interface{}{
        "action": "type",
        "text":   "Hello, World!",
    })

    agent.Execute(map[string]interface{}{
        "action": "key_press",
        "key":    "enter",
    })

    agent.Execute(map[string]interface{}{
        "action":    "key_combo",
        "modifiers": []string{"ctrl"},
        "key":       "s",
    })
}
```

## API Reference

### Mouse Actions

| Action | Parameters | Description |
|--------|------------|-------------|
| `mouse_move` | `x`, `y` | Move by relative offset |
| `mouse_click` | `button?` | Click (left/right/middle) |
| `mouse_double_click` | `button?` | Double click |
| `mouse_down` | `button?` | Press button |
| `mouse_up` | `button?` | Release button |
| `mouse_scroll` | `delta` | Scroll wheel (+up/-down) |

### Keyboard Actions

| Action | Parameters | Description |
|--------|------------|-------------|
| `type` | `text` | Type a string |
| `key_press` | `key` | Press and release key |
| `key_down` | `key` | Press key |
| `key_up` | `key` | Release key |
| `key_combo` | `modifiers[]`, `key` | Key combination |

**Modifiers:** `ctrl`, `shift`, `alt`, `cmd`/`meta`/`win`

**Keys:** `a-z`, `0-9`, `enter`, `escape`, `backspace`, `tab`, `space`, `up`, `down`, `left`, `right`, `home`, `end`, `pageup`, `pagedown`, `delete`, `insert`, `f1-f12`

### Gamepad Actions

| Action | Parameters | Description |
|--------|------------|-------------|
| `gamepad_press` | `button` | Press button |
| `gamepad_release` | `button` | Release button |
| `gamepad_left_stick` | `x`, `y` | Left stick (0-255) |
| `gamepad_right_stick` | `x`, `y` | Right stick (0-255) |
| `gamepad_triggers` | `left`, `right` | Triggers (0-255) |

## Direct Device Control

```go
package main

import (
    usehid "go.zoe.im/usehid-go"
)

func main() {
    // Mouse
    mouse := usehid.NewMouse()
    defer mouse.Close()
    
    mouse.MoveBy(100, 50)
    mouse.Click(usehid.ButtonLeft)
    mouse.DoubleClick(usehid.ButtonLeft)
    mouse.Scroll(-3)

    // Keyboard
    keyboard := usehid.NewKeyboard()
    defer keyboard.Close()
    
    keyboard.TypeText("Hello!")
    keyboard.Press("enter")
    keyboard.Combo([]string{"ctrl"}, "c")

    // Gamepad
    gamepad := usehid.NewGamepad()
    defer gamepad.Close()
    
    gamepad.Press("a")
    gamepad.SetLeftStick(128, 0)  // center-up
    gamepad.SetTriggers(255, 0)   // full left trigger
}
```

## Platform Support

| Platform | Mouse | Keyboard | Gamepad | Backend |
|----------|:-----:|:--------:|:-------:|---------|
| **macOS** | ✅ | ✅ | ⚠️ | CGEvent |
| **Linux** | ✅ | ✅ | ✅ | uhid |
| **Windows** | ✅ | ✅ | ⚠️ | SendInput |

## Permissions

### macOS
Requires **Accessibility** permission:
System Preferences → Security & Privacy → Privacy → Accessibility

### Linux
Requires access to `/dev/uhid`:
```bash
sudo chmod 666 /dev/uhid
```

### Windows
No special permissions required.

## Building from Source

Requires the useHID Rust library:

```bash
# Build Rust library
cd usehid
cargo build --release

# Build Go package
cd usehid-go
CGO_ENABLED=1 go build
```

## License

MIT - see [LICENSE](../LICENSE)

## Links

- [GitHub Repository](https://github.com/jiusanzhou/usehid)
- [Rust Crate (usehid-core)](https://crates.io/crates/usehid-core)
- [Documentation](https://pkg.go.dev/go.zoe.im/usehid-go)
