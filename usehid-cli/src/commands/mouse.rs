use clap::Subcommand;
use usehid::agent::AgentAction;

#[derive(Subcommand)]
pub enum MouseCommand {
    /// Move mouse to absolute position
    MoveTo {
        /// X coordinate
        x: i32,
        /// Y coordinate
        y: i32,
        /// Animation duration in milliseconds
        #[arg(long)]
        duration: Option<u64>,
        /// Tween function name
        #[arg(long)]
        tween: Option<String>,
    },
    /// Move mouse by relative offset
    Move {
        /// X offset
        x: i32,
        /// Y offset
        y: i32,
        /// Animation duration in milliseconds
        #[arg(long)]
        duration: Option<u64>,
        /// Tween function name
        #[arg(long)]
        tween: Option<String>,
    },
    /// Click a mouse button
    Click {
        /// Button: left, right, middle
        #[arg(long, default_value = "left")]
        button: String,
    },
    /// Double-click a mouse button
    DoubleClick {
        /// Button: left, right, middle
        #[arg(long, default_value = "left")]
        button: String,
    },
    /// Press and hold a mouse button
    Down {
        /// Button: left, right, middle
        #[arg(long, default_value = "left")]
        button: String,
    },
    /// Release a mouse button
    Up {
        /// Button: left, right, middle
        #[arg(long, default_value = "left")]
        button: String,
    },
    /// Scroll the mouse wheel
    Scroll {
        /// Scroll delta (positive=up, negative=down)
        delta: i8,
    },
    /// Drag by relative offset
    Drag {
        /// X offset
        x: i32,
        /// Y offset
        y: i32,
        /// Button: left, right, middle
        #[arg(long, default_value = "left")]
        button: String,
        /// Animation duration in milliseconds
        #[arg(long)]
        duration: Option<u64>,
        /// Tween function name
        #[arg(long)]
        tween: Option<String>,
    },
    /// Drag to absolute position
    DragTo {
        /// Target X coordinate
        x: i32,
        /// Target Y coordinate
        y: i32,
        /// Button: left, right, middle
        #[arg(long, default_value = "left")]
        button: String,
        /// Animation duration in milliseconds
        #[arg(long)]
        duration: Option<u64>,
        /// Tween function name
        #[arg(long)]
        tween: Option<String>,
    },
}

pub fn to_action(cmd: MouseCommand) -> AgentAction {
    match cmd {
        MouseCommand::MoveTo { x, y, duration, tween } => {
            AgentAction::MouseMoveTo { x, y, duration, tween }
        }
        MouseCommand::Move { x, y, duration, tween } => {
            AgentAction::MouseMove { x, y, duration, tween }
        }
        MouseCommand::Click { button } => {
            AgentAction::MouseClick { button: Some(button) }
        }
        MouseCommand::DoubleClick { button } => {
            AgentAction::MouseDoubleClick { button: Some(button) }
        }
        MouseCommand::Down { button } => {
            AgentAction::MouseDown { button: Some(button) }
        }
        MouseCommand::Up { button } => {
            AgentAction::MouseUp { button: Some(button) }
        }
        MouseCommand::Scroll { delta } => {
            AgentAction::MouseScroll { delta }
        }
        MouseCommand::Drag { x, y, button, duration, tween } => {
            AgentAction::MouseDrag { x, y, button: Some(button), duration, tween }
        }
        MouseCommand::DragTo { x, y, button, duration, tween } => {
            AgentAction::MouseDragTo { x, y, button: Some(button), duration, tween }
        }
    }
}
