# useHID Go Bindings

Cross-platform virtual HID device library for Go.

## Installation

```bash
go get github.com/jiusanzhou/usehid-go
```

## Quick Start

```go
package main

import (
    "github.com/jiusanzhou/usehid-go"
)

func main() {
    // Basic Mouse
    mouse := usehid.NewMouse()
    defer mouse.Close()
    
    mouse.MoveBy(100, 50)
    mouse.Click(usehid.ButtonLeft)
    mouse.Scroll(-3) // Scroll down

    // Basic Keyboard
    keyboard := usehid.NewKeyboard()
    defer keyboard.Close()
    
    keyboard.TypeText("Hello, World!")
    keyboard.Press("enter")
    keyboard.Combo([]usehid.Modifier{usehid.ModCtrl}, "c") // Copy

    // For AI Agents
    agent := usehid.NewAgentHID()
    defer agent.Close()
    
    // Execute actions from map
    result := agent.Execute(map[string]interface{}{
        "action": "mouse_move",
        "x":      100,
        "y":      200,
    })
    
    result = agent.Execute(map[string]interface{}{
        "action": "type",
        "text":   "Hello from AI!",
    })
}
```

## Building

Requires the useHID Rust library to be built first:

```bash
cd usehid
cargo build --release
```

Then build the Go package:

```bash
cd usehid-go
go build
```

## License

MIT
