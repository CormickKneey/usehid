use clap::Subcommand;
use usehid::agent::AgentAction;

#[derive(Subcommand)]
pub enum GamepadCommand {
    /// Press a gamepad button
    Press {
        /// Button name (e.g., a, b, x, y, lb, rb, start, dpad_up)
        button: String,
    },
    /// Release a gamepad button
    Release {
        /// Button name
        button: String,
    },
    /// Set left stick position
    LeftStick {
        /// X axis (0-255, 128=center)
        x: u8,
        /// Y axis (0-255, 128=center)
        y: u8,
    },
    /// Set right stick position
    RightStick {
        /// X axis (0-255, 128=center)
        x: u8,
        /// Y axis (0-255, 128=center)
        y: u8,
    },
    /// Set trigger values
    Triggers {
        /// Left trigger (0-255)
        #[arg(long)]
        left: u8,
        /// Right trigger (0-255)
        #[arg(long)]
        right: u8,
    },
}

pub fn to_action(cmd: GamepadCommand) -> AgentAction {
    match cmd {
        GamepadCommand::Press { button } => AgentAction::GamepadPress { button },
        GamepadCommand::Release { button } => AgentAction::GamepadRelease { button },
        GamepadCommand::LeftStick { x, y } => AgentAction::GamepadLeftStick { x, y },
        GamepadCommand::RightStick { x, y } => AgentAction::GamepadRightStick { x, y },
        GamepadCommand::Triggers { left, right } => AgentAction::GamepadTriggers { left, right },
    }
}
