# useHID Python Bindings

Cross-platform virtual HID device library for Python.

## Installation

```bash
pip install usehid
```

## Quick Start

```python
from usehid import Mouse, Keyboard, AgentHID

# Basic Mouse
mouse = Mouse()
mouse.move_by(100, 50)
mouse.click()
mouse.double_click("left")
mouse.scroll(-3)  # Scroll down

# Basic Keyboard
keyboard = Keyboard()
keyboard.type_text("Hello, World!")
keyboard.press("enter")
keyboard.combo(["ctrl"], "c")  # Copy

# For AI Agents
agent = AgentHID()

# Execute actions from dict
result = agent.execute({
    "action": "mouse_move",
    "x": 100,
    "y": 200
})

result = agent.execute({
    "action": "type",
    "text": "Hello from AI!"
})

result = agent.execute({
    "action": "key_combo",
    "modifiers": ["ctrl", "shift"],
    "key": "s"
})
```

## Available Actions

### Mouse Actions

- `mouse_move` - Move mouse by offset (`x`, `y`)
- `mouse_click` - Click button (`button`: "left"/"right"/"middle")
- `mouse_double_click` - Double click
- `mouse_down` - Press button
- `mouse_up` - Release button
- `mouse_scroll` - Scroll wheel (`delta`: positive=up, negative=down)

### Keyboard Actions

- `type` - Type text (`text`)
- `key_press` - Press and release key (`key`)
- `key_down` - Press key
- `key_up` - Release key
- `key_combo` - Key combination (`modifiers`: list, `key`)

### Gamepad Actions

- `gamepad_press` - Press button
- `gamepad_release` - Release button
- `gamepad_left_stick` - Set left stick (`x`, `y`: 0-255)
- `gamepad_right_stick` - Set right stick
- `gamepad_triggers` - Set triggers (`left`, `right`: 0-255)

## Building

```bash
cd usehid-python
maturin develop
```

## License

MIT
