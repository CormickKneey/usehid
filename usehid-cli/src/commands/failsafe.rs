use clap::Subcommand;
use usehid::agent::AgentAction;

#[derive(Subcommand)]
pub enum FailsafeCommand {
    /// Check failsafe status
    Status,
    /// Enable failsafe
    Enable,
    /// Disable failsafe
    Disable,
    /// Reset failsafe trigger
    Reset,
}

pub fn to_action(cmd: FailsafeCommand) -> AgentAction {
    match cmd {
        FailsafeCommand::Status => AgentAction::FailsafeStatus,
        FailsafeCommand::Enable => AgentAction::FailsafeEnable,
        FailsafeCommand::Disable => AgentAction::FailsafeDisable,
        FailsafeCommand::Reset => AgentAction::FailsafeReset,
    }
}
