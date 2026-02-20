/// <reference types="node" />

/**
 * Mouse button types
 */
export type MouseButton = 'left' | 'right' | 'middle';

/**
 * Virtual Mouse device
 */
export class Mouse {
  /**
   * Create a new virtual mouse
   * @param name Optional device name
   */
  constructor(name?: string);

  /**
   * Move mouse by relative offset
   */
  moveBy(dx: number, dy: number): void;

  /**
   * Click mouse button
   */
  click(button?: MouseButton): void;

  /**
   * Double click mouse button
   */
  doubleClick(button?: MouseButton): void;

  /**
   * Press mouse button
   */
  press(button?: MouseButton): void;

  /**
   * Release mouse button
   */
  release(button?: MouseButton): void;

  /**
   * Scroll wheel (positive = up, negative = down)
   */
  scroll(delta: number): void;

  /**
   * Destroy the virtual device
   */
  close(): void;
}

/**
 * Virtual Keyboard device
 */
export class Keyboard {
  /**
   * Create a new virtual keyboard
   * @param name Optional device name
   */
  constructor(name?: string);

  /**
   * Type a string
   */
  typeText(text: string): void;

  /**
   * Press and release a key
   */
  press(key: string): void;

  /**
   * Press a key combination
   * @param modifiers Array of modifiers: 'ctrl', 'shift', 'alt', 'cmd'
   * @param key The key to press
   */
  combo(modifiers: string[], key: string): void;

  /**
   * Release all keys
   */
  releaseAll(): void;

  /**
   * Destroy the virtual device
   */
  close(): void;
}

/**
 * Gamepad button types
 */
export type GamepadButtonType =
  | 'a' | 'b' | 'x' | 'y'
  | 'lb' | 'rb'
  | 'back' | 'start' | 'guide'
  | 'left_stick' | 'right_stick'
  | 'dpad_up' | 'dpad_down' | 'dpad_left' | 'dpad_right';

/**
 * Virtual Gamepad device
 */
export class Gamepad {
  /**
   * Create a new virtual gamepad
   * @param name Optional device name
   */
  constructor(name?: string);

  /**
   * Press a button
   */
  press(button: GamepadButtonType): void;

  /**
   * Release a button
   */
  release(button: GamepadButtonType): void;

  /**
   * Tap a button (press and release)
   */
  tap(button: GamepadButtonType): void;

  /**
   * Set left stick position (0-255, 128 = center)
   */
  leftStick(x: number, y: number): void;

  /**
   * Set right stick position (0-255, 128 = center)
   */
  rightStick(x: number, y: number): void;

  /**
   * Set triggers (0-255)
   */
  triggers(left: number, right: number): void;

  /**
   * Reset all to default
   */
  reset(): void;

  /**
   * Destroy the virtual device
   */
  close(): void;
}

/**
 * Agent action types
 */
export interface MouseMoveAction {
  action: 'mouse_move';
  x: number;
  y: number;
}

export interface MouseClickAction {
  action: 'mouse_click' | 'mouse_double_click' | 'mouse_down' | 'mouse_up';
  button?: MouseButton;
}

export interface MouseScrollAction {
  action: 'mouse_scroll';
  delta: number;
}

export interface TypeAction {
  action: 'type';
  text: string;
}

export interface KeyAction {
  action: 'key_press' | 'key_down' | 'key_up';
  key: string;
}

export interface KeyComboAction {
  action: 'key_combo';
  modifiers: string[];
  key: string;
}

export interface GamepadPressAction {
  action: 'gamepad_press' | 'gamepad_release';
  button: GamepadButtonType;
}

export interface GamepadStickAction {
  action: 'gamepad_left_stick' | 'gamepad_right_stick';
  x: number;
  y: number;
}

export interface GamepadTriggersAction {
  action: 'gamepad_triggers';
  left: number;
  right: number;
}

export type AgentAction =
  | MouseMoveAction
  | MouseClickAction
  | MouseScrollAction
  | TypeAction
  | KeyAction
  | KeyComboAction
  | GamepadPressAction
  | GamepadStickAction
  | GamepadTriggersAction;

/**
 * Result of an agent action
 */
export interface AgentResult {
  success: boolean;
  error?: string;
}

/**
 * Agent HID controller for LLM agents
 */
export class AgentHID {
  constructor();

  /**
   * Execute an action
   */
  execute(action: AgentAction): AgentResult;

  /**
   * Execute an action from JSON string
   */
  executeJson(json: string): string;

  /**
   * Destroy the agent controller
   */
  close(): void;
}
