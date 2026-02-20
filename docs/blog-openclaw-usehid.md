---
title: "让 AI 完全控制你的电脑：OpenClaw + useHID 实战指南"
date: 2026-02-20
author: Zoe
tags: [AI, OpenClaw, useHID, 自动化, HID, Agent]
---

# 让 AI 完全控制你的电脑：OpenClaw + useHID 实战指南

想象一下，你只需要用自然语言告诉 AI "帮我打开 Chrome，搜索 GitHub，然后滚动到底部"，AI 就会自动操控鼠标和键盘完成这些任务。这不再是科幻电影的场景——通过 **OpenClaw** 和 **useHID** 的结合，这已经成为现实。

## 什么是 useHID？

[useHID](https://github.com/jiusanzhou/usehid) 是一个跨平台的虚拟 HID（Human Interface Device）设备库，专为 AI Agent 设计。它提供：

- 🖱️ **虚拟鼠标** — 移动、点击、滚轮
- ⌨️ **虚拟键盘** — 打字、按键、组合键
- 🎮 **虚拟游戏手柄** — 摇杆、按钮、扳机
- 🤖 **Agent API** — JSON 接口，完美适配 LLM 工具调用

支持 macOS、Linux 和 Windows，无需安装特殊驱动。

## 为什么需要它？

现有的 AI 自动化工具（如 browser-use）通常只能控制浏览器。但很多任务需要操作桌面应用：

- 打开 Finder/文件管理器
- 操作 IDE 写代码
- 控制 Photoshop 修图
- 玩游戏（是的，AI 可以帮你打游戏）

useHID 填补了这个空白，让 AI 可以像人一样操控电脑的任何应用。

## 快速开始

### 1. 安装 useHID

```bash
# 克隆仓库
git clone https://github.com/jiusanzhou/usehid.git
cd usehid

# 安装 Python 绑定
cd usehid-python
pip install maturin
maturin develop --release
```

### 2. 授予权限 (macOS)

useHID 使用 macOS 的 CGEvent API，需要辅助功能权限：

1. 打开 **系统偏好设置** → **安全性与隐私** → **隐私** → **辅助功能**
2. 添加你的终端应用（Terminal、iTerm2 等）

### 3. 测试一下

```python
from usehid import AgentHID

agent = AgentHID()

# 移动鼠标
agent.execute({"action": "mouse_move", "x": 100, "y": 50})

# 点击
agent.execute({"action": "mouse_click", "button": "left"})

# 打字
agent.execute({"action": "type", "text": "Hello, World!"})

# 组合键 (Cmd+S 保存)
agent.execute({"action": "key_combo", "modifiers": ["cmd"], "key": "s"})
```

## 集成到 OpenClaw

### 1. 复制 Skill

```bash
cp -r usehid/skills/usehid ~/.openclaw/skills/
```

### 2. 配置 OpenClaw

在 `~/.openclaw/workspace/SKILLS.md` 中添加：

```markdown
## usehid

控制电脑的鼠标、键盘和游戏手柄。

当用户要求控制电脑（点击、输入、滚动、按键）时使用。

位置：~/.openclaw/skills/usehid/SKILL.md
```

### 3. 开始使用

现在你可以用自然语言控制电脑了：

```
你: 打开 Spotlight，搜索 Chrome 并打开

AI: 好的，我来帮你打开 Chrome。
[执行 Cmd+Space]
[输入 "Chrome"]
[按 Enter]
Chrome 已打开。

你: 在地址栏输入 github.com 然后访问

AI: 正在导航到 GitHub...
[执行 Cmd+L 聚焦地址栏]
[输入 "github.com"]
[按 Enter]
已打开 GitHub 首页。

你: 向下滚动一点

AI: [执行滚轮向下]
已向下滚动。
```

## 实战案例

### 案例 1：自动化表单填写

```
你: 帮我填写这个注册表单，用户名 demo，邮箱 demo@example.com

AI: 好的，我来帮你填写表单。
[点击用户名输入框]
[输入 "demo"]
[按 Tab 切换到邮箱]
[输入 "demo@example.com"]
表单已填写完成，需要我提交吗？
```

### 案例 2：文件操作

```
你: 打开桌面上的 report.pdf

AI: 正在打开 report.pdf...
[Cmd+Space 打开 Spotlight]
[输入 "report.pdf"]
[按 Enter]
文件已打开。
```

### 案例 3：截图并保存

```
你: 截个屏保存到桌面

AI: 正在截图...
[执行 Cmd+Shift+3]
截图已保存到桌面。
```

## 支持的操作

### 鼠标

| 操作 | JSON |
|------|------|
| 移动 | `{"action": "mouse_move", "x": 100, "y": 50}` |
| 左键点击 | `{"action": "mouse_click", "button": "left"}` |
| 右键点击 | `{"action": "mouse_click", "button": "right"}` |
| 双击 | `{"action": "mouse_double_click"}` |
| 滚轮 | `{"action": "mouse_scroll", "delta": -3}` |
| 拖拽 | `mouse_down` → `mouse_move` → `mouse_up` |

### 键盘

| 操作 | JSON |
|------|------|
| 打字 | `{"action": "type", "text": "Hello"}` |
| 按键 | `{"action": "key_press", "key": "enter"}` |
| 组合键 | `{"action": "key_combo", "modifiers": ["cmd"], "key": "s"}` |

### 修饰键

- `ctrl` — Control
- `shift` — Shift  
- `alt` — Alt/Option
- `cmd` / `meta` / `win` — Command (macOS) / Windows 键

### 特殊键

`enter`, `escape`, `backspace`, `tab`, `space`, `up`, `down`, `left`, `right`, `home`, `end`, `pageup`, `pagedown`, `delete`, `f1`-`f12`

## 安全注意事项

⚠️ **重要**：让 AI 控制电脑是强大但有风险的。请注意：

1. **确认破坏性操作** — 删除文件、关闭未保存的文档前要确认
2. **在测试环境先试** — 不要在生产环境直接使用
3. **设置权限边界** — 考虑限制可访问的应用
4. **人工监督** — 初期使用时保持监督

## 技术实现

useHID 在不同平台使用不同的后端：

| 平台 | 鼠标/键盘 | 游戏手柄 |
|------|-----------|----------|
| macOS | CGEvent API | IOHIDUserDevice (需签名) |
| Linux | uhid (/dev/uhid) | uhid |
| Windows | SendInput API | ViGEmBus |

macOS 使用 CGEvent 而不是 IOHIDUserDevice，因为后者需要 Apple 开发者签名，而 CGEvent 只需要辅助功能权限。

## 总结

useHID + OpenClaw 的组合开启了 AI 自动化的新时代。从简单的点击操作到复杂的工作流自动化，AI 现在可以像人一样操控你的电脑。

**项目地址：**
- useHID: https://github.com/jiusanzhou/usehid
- OpenClaw: https://github.com/openclaw/openclaw

**安装命令：**
```bash
# Rust
cargo add usehid-core

# Python (源码)
cd usehid-python && maturin develop --release

# Go
go get go.zoe.im/usehid-go@latest
```

试试看，让 AI 成为你的桌面助手吧！🚀

---

*如有问题或建议，欢迎在 GitHub 提 Issue 或 PR。*
