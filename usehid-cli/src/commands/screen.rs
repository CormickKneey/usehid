use clap::Subcommand;
use usehid::agent::AgentAction;

#[derive(Subcommand)]
pub enum ScreenCommand {
    /// Get screen size
    Size,
    /// Get current mouse position
    Position,
}

pub fn to_action(cmd: ScreenCommand) -> AgentAction {
    match cmd {
        ScreenCommand::Size => AgentAction::Size,
        ScreenCommand::Position => AgentAction::Position,
    }
}
