# useHID Skill for OpenClaw

Control mouse, keyboard, and gamepad through natural language commands.

## Description

This skill enables OpenClaw to control your computer's input devices programmatically. It provides a JSON-based API that integrates seamlessly with LLM tool-calling.

## Requirements

- Python 3.9+
- usehid Python package

### Installation

```bash
pip install usehid
```

### macOS Permissions

Grant Accessibility permissions:
1. System Preferences → Security & Privacy → Privacy → Accessibility
2. Add your terminal or OpenClaw application

## Usage

When the user asks to control the computer (click, type, scroll, press keys), use the `usehid_tool.py` script.

### Mouse Actions

```bash
# Move mouse
python usehid_tool.py '{"action": "mouse_move", "x": 100, "y": 50}'

# Click
python usehid_tool.py '{"action": "mouse_click", "button": "left"}'

# Double click
python usehid_tool.py '{"action": "mouse_double_click"}'

# Scroll
python usehid_tool.py '{"action": "mouse_scroll", "delta": -3}'

# Drag (press, move, release)
python usehid_tool.py '{"action": "mouse_down", "button": "left"}'
python usehid_tool.py '{"action": "mouse_move", "x": 200, "y": 100}'
python usehid_tool.py '{"action": "mouse_up", "button": "left"}'
```

### Keyboard Actions

```bash
# Type text
python usehid_tool.py '{"action": "type", "text": "Hello, World!"}'

# Press a key
python usehid_tool.py '{"action": "key_press", "key": "enter"}'

# Key combination (Ctrl+S, Cmd+C, etc.)
python usehid_tool.py '{"action": "key_combo", "modifiers": ["ctrl"], "key": "s"}'
python usehid_tool.py '{"action": "key_combo", "modifiers": ["cmd"], "key": "c"}'
```

### Common Workflows

**Open application (macOS Spotlight):**
```bash
python usehid_tool.py '{"action": "key_combo", "modifiers": ["cmd"], "key": "space"}'
sleep 0.5
python usehid_tool.py '{"action": "type", "text": "Chrome"}'
python usehid_tool.py '{"action": "key_press", "key": "enter"}'
```

**Copy and Paste:**
```bash
python usehid_tool.py '{"action": "key_combo", "modifiers": ["cmd"], "key": "a"}'
python usehid_tool.py '{"action": "key_combo", "modifiers": ["cmd"], "key": "c"}'
python usehid_tool.py '{"action": "key_combo", "modifiers": ["cmd"], "key": "v"}'
```

**Save file:**
```bash
python usehid_tool.py '{"action": "key_combo", "modifiers": ["cmd"], "key": "s"}'
```

## Supported Keys

**Modifiers:** `ctrl`, `shift`, `alt`, `cmd`/`meta`/`win`

**Special Keys:** `enter`, `escape`, `backspace`, `tab`, `space`, `up`, `down`, `left`, `right`, `home`, `end`, `pageup`, `pagedown`, `delete`, `insert`, `f1`-`f12`

**Mouse Buttons:** `left`, `right`, `middle`

## Safety Notes

- Always confirm destructive actions (delete, close without save)
- Use small movements for precision
- Add delays between rapid actions
- Test on non-critical applications first
