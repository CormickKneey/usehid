# useHID Node.js Bindings

[![npm](https://img.shields.io/npm/v/usehid.svg)](https://www.npmjs.com/package/usehid)
[![Node](https://img.shields.io/node/v/usehid.svg)](https://www.npmjs.com/package/usehid)

Cross-platform virtual HID device library for Node.js/TypeScript.

## Installation

```bash
npm install usehid
# or
pnpm add usehid
```

## Quick Start

```typescript
import { Mouse, Keyboard, AgentHID } from 'usehid';

// Basic Mouse
const mouse = new Mouse();
mouse.moveBy(100, 50);
mouse.click();
mouse.doubleClick('left');
mouse.scroll(-3);  // Scroll down
mouse.close();

// Basic Keyboard
const keyboard = new Keyboard();
keyboard.typeText("Hello, World!");
keyboard.press("enter");
keyboard.combo(["ctrl"], "c");  // Copy
keyboard.close();

// For AI Agents
const agent = new AgentHID();

// Execute actions
const result = agent.execute({
  action: "mouse_move",
  x: 100,
  y: 200
});

agent.execute({
  action: "type",
  text: "Hello from AI!"
});

agent.execute({
  action: "key_combo",
  modifiers: ["ctrl", "shift"],
  key: "s"
});

agent.close();
```

## Building

```bash
cd usehid-node
npm install
npm run build
```

## License

MIT
