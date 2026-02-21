# Agent HID API Specification v1.0

跨平台统一的 AI Agent 输入控制 API 规范。

## 概述

本规范定义了 AI Agent 控制设备输入的 JSON API 格式，适用于：
- **Desktop**: useHID (macOS/Linux/Windows)
- **Android**: flutter_automate
- **iOS**: 暂不支持

## 通用格式

### 请求

```json
{
  "action": "action_name",
  "param1": "value1",
  "param2": "value2"
}
```

### 响应

```json
{
  "success": true,
  "error": null,
  // 其他返回字段...
}
```

---

## 屏幕查询

### size - 获取屏幕尺寸

```json
// 请求
{"action": "size"}

// 响应
{"success": true, "width": 1920, "height": 1080}
```

### position - 获取当前触点/鼠标位置

```json
// 请求
{"action": "position"}

// 响应 (Desktop)
{"success": true, "x": 100, "y": 200}

// 响应 (Android - 返回最后触点位置，可能为 null)
{"success": true, "x": null, "y": null}
```

---

## 点击操作

### click - 点击

```json
// 坐标点击 (推荐)
{"action": "click", "x": 100, "y": 200}

// 当前位置点击 (仅 Desktop)
{"action": "click"}

// 带按钮参数 (仅 Desktop)
{"action": "click", "x": 100, "y": 200, "button": "left"}
```

| 参数 | 类型 | 必须 | 说明 |
|------|------|------|------|
| x | number | Desktop: 否, Android: 是 | X 坐标 |
| y | number | Desktop: 否, Android: 是 | Y 坐标 |
| button | string | 否 | left/right/middle (仅 Desktop) |

### double_click - 双击

```json
{"action": "double_click", "x": 100, "y": 200}
```

### long_click - 长按

```json
{"action": "long_click", "x": 100, "y": 200, "duration": 500}
```

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| duration | number | 500 | 按住时长 (ms) |

---

## 移动操作

### move_to - 移动到绝对坐标

```json
{"action": "move_to", "x": 500, "y": 300}

// 带动画
{"action": "move_to", "x": 500, "y": 300, "duration": 1000, "tween": "ease_out"}
```

| 参数 | 类型 | 必须 | 说明 |
|------|------|------|------|
| x | number | 是 | 目标 X |
| y | number | 是 | 目标 Y |
| duration | number | 否 | 动画时长 (ms) |
| tween | string | 否 | 缓动函数 |

### move - 相对移动

```json
{"action": "move", "x": 100, "y": -50}
```

---

## 滑动/拖拽

### swipe - 滑动

```json
{"action": "swipe", "x1": 500, "y1": 800, "x2": 500, "y2": 200, "duration": 300}
```

| 参数 | 类型 | 必须 | 说明 |
|------|------|------|------|
| x1, y1 | number | 是 | 起点 |
| x2, y2 | number | 是 | 终点 |
| duration | number | 否 | 滑动时长 (ms) |

### swipe_up / swipe_down / swipe_left / swipe_right

```json
{"action": "swipe_up"}
{"action": "swipe_down"}
{"action": "swipe_left"}
{"action": "swipe_right"}
```

### drag - 拖拽 (Desktop)

```json
// 相对拖拽
{"action": "drag", "x": 100, "y": 50}

// 绝对拖拽
{"action": "drag_to", "x": 600, "y": 400}
```

---

## 滚动

### scroll - 滚动

```json
// Desktop: 鼠标滚轮
{"action": "scroll", "delta": -3}

// Android: 等同于短滑动
{"action": "scroll", "direction": "down", "amount": 300}
```

| 参数 | 类型 | 平台 | 说明 |
|------|------|------|------|
| delta | number | Desktop | 滚轮增量 (+上/-下) |
| direction | string | Android | up/down/left/right |
| amount | number | Android | 滚动距离 (px) |

---

## 键盘输入

### type - 输入文本

```json
{"action": "type", "text": "Hello World!"}

// 带间隔 (打字机效果)
{"action": "type", "text": "Hello", "interval": 100}
```

| 参数 | 类型 | 必须 | 说明 |
|------|------|------|------|
| text | string | 是 | 输入文本 |
| interval | number | 否 | 每字符间隔 (ms) |

### key_press - 按键

```json
{"action": "key_press", "key": "enter"}
```

**通用按键**: enter, backspace, delete, tab, escape, space, up, down, left, right, home, end

### key_combo - 组合键 (仅 Desktop)

```json
{"action": "key_combo", "modifiers": ["ctrl"], "key": "c"}
{"action": "key_combo", "modifiers": ["cmd", "shift"], "key": "s"}
```

---

## 系统按键 (仅 Android)

### back - 返回键

```json
{"action": "back"}
```

### home - Home 键

```json
{"action": "home"}
```

### recents - 最近任务

```json
{"action": "recents"}
```

---

## UI 查询 (仅 Android)

### dump_ui - 获取界面元素树

```json
// 请求
{"action": "dump_ui"}

// 响应
{
  "success": true,
  "package": "com.example.app",
  "activity": "MainActivity",
  "elements": [
    {
      "id": "com.example:id/button",
      "class": "android.widget.Button",
      "text": "Click Me",
      "desc": "Submit button",
      "bounds": {"x": 100, "y": 200, "width": 200, "height": 50},
      "clickable": true,
      "children": []
    }
  ]
}
```

### find_and_click - 查找并点击

```json
{"action": "find_and_click", "text": "确定"}
{"action": "find_and_click", "id": "com.example:id/submit"}
{"action": "find_and_click", "desc": "Submit button"}
```

---

## 安全机制

### failsafe_status - 查询状态

```json
// 请求
{"action": "failsafe_status"}

// 响应
{"success": true, "enabled": true, "triggered": false}
```

### failsafe_enable / failsafe_disable

```json
{"action": "failsafe_enable"}
{"action": "failsafe_disable"}
```

### failsafe_reset

```json
{"action": "failsafe_reset"}
```

---

## 平台能力对照表

| Action | Desktop | Android | 说明 |
|--------|:-------:|:-------:|------|
| size | ✅ | ✅ | |
| position | ✅ | ⚠️ | Android 返回 null |
| click | ✅ | ✅ | |
| double_click | ✅ | ✅ | |
| long_click | ✅ | ✅ | |
| move_to | ✅ | ❌ | Android 无鼠标 |
| move | ✅ | ❌ | |
| swipe | ✅ | ✅ | Desktop = drag |
| swipe_up/down/left/right | ⚠️ | ✅ | |
| drag | ✅ | ❌ | Android 用 swipe |
| scroll | ✅ | ✅ | 参数不同 |
| type | ✅ | ✅ | |
| key_press | ✅ | ✅ | |
| key_combo | ✅ | ❌ | |
| back | ❌ | ✅ | |
| home | ❌ | ✅ | |
| recents | ❌ | ✅ | |
| dump_ui | ❌ | ✅ | |
| find_and_click | ❌ | ✅ | |
| failsafe_* | ✅ | ✅ | |

---

## 实现要求

1. **统一入口**: 实现 `execute(json)` 方法
2. **错误处理**: 不支持的 action 返回 `{"success": false, "error": "unsupported_action"}`
3. **参数验证**: 缺少必须参数返回明确错误
4. **异步执行**: 所有操作应支持异步

---

## 版本历史

- v1.0 (2026-02-21): 初始版本
