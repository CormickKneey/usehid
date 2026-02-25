use clap::Subcommand;
use usehid::agent::AgentAction;

#[derive(Subcommand)]
pub enum KeyCommand {
    /// Press and release a key
    Press {
        /// Key name (e.g., enter, space, a, f1)
        key: String,
    },
    /// Press and hold a key
    Down {
        /// Key name
        key: String,
    },
    /// Release a key
    Up {
        /// Key name
        key: String,
    },
    /// Press a key combination (e.g., cmd shift t)
    Combo {
        /// Modifier keys followed by the target key (last arg is the key)
        #[arg(required = true, num_args = 2..)]
        keys: Vec<String>,
    },
}

pub fn to_action(cmd: KeyCommand) -> AgentAction {
    match cmd {
        KeyCommand::Press { key } => AgentAction::KeyPress { key },
        KeyCommand::Down { key } => AgentAction::KeyDown { key },
        KeyCommand::Up { key } => AgentAction::KeyUp { key },
        KeyCommand::Combo { keys } => {
            let (key, modifiers) = keys.split_last().expect("at least 2 args required");
            AgentAction::KeyCombo {
                modifiers: modifiers.to_vec(),
                key: key.clone(),
            }
        }
    }
}
