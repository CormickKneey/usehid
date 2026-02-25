use clap::Parser;
use usehid::agent::AgentAction;
use usehid::AgentHID;

mod commands;
mod output;

use commands::*;
use output::print_result;

/// Cross-platform virtual HID device control for AI agents
#[derive(Parser)]
#[command(name = "usehid", version, about)]
struct Cli {
    /// Output in human-readable format instead of JSON
    #[arg(long, global = true)]
    human: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Mouse control
    Mouse {
        #[command(subcommand)]
        command: mouse::MouseCommand,
    },
    /// Type text (shortcut for keyboard input)
    Type {
        /// Text to type
        text: String,
        /// Interval between keystrokes in milliseconds
        #[arg(long)]
        interval: Option<u64>,
    },
    /// Keyboard control
    Key {
        #[command(subcommand)]
        command: keyboard::KeyCommand,
    },
    /// Gamepad control
    Gamepad {
        #[command(subcommand)]
        command: gamepad::GamepadCommand,
    },
    /// Screen queries
    Screen {
        #[command(subcommand)]
        command: screen::ScreenCommand,
    },
    /// Failsafe control
    Failsafe {
        #[command(subcommand)]
        command: failsafe::FailsafeCommand,
    },
    /// Execute a raw JSON action
    Exec {
        /// JSON action string, or "-" to read from stdin
        json: Option<String>,
    },
    /// Run a batch of actions from a JSON file
    Run {
        /// Path to JSON file containing an array of actions
        file: String,
        /// Delay between actions in milliseconds
        #[arg(long)]
        delay: Option<u64>,
    },
}

fn main() {
    let cli = Cli::parse();
    let human = cli.human;

    match cli.command {
        Command::Mouse { command } => {
            let action = mouse::to_action(command);
            let mut hid = AgentHID::new();
            let result = hid.execute(action);
            print_result(&result, human);
        }
        Command::Type { text, interval } => {
            let action = AgentAction::Type { text, interval };
            let mut hid = AgentHID::new();
            let result = hid.execute(action);
            print_result(&result, human);
        }
        Command::Key { command } => {
            let action = keyboard::to_action(command);
            let mut hid = AgentHID::new();
            let result = hid.execute(action);
            print_result(&result, human);
        }
        Command::Gamepad { command } => {
            let action = gamepad::to_action(command);
            let mut hid = AgentHID::new();
            let result = hid.execute(action);
            print_result(&result, human);
        }
        Command::Screen { command } => {
            let action = screen::to_action(command);
            let mut hid = AgentHID::new();
            let result = hid.execute(action);
            print_result(&result, human);
        }
        Command::Failsafe { command } => {
            let action = failsafe::to_action(command);
            let mut hid = AgentHID::new();
            let result = hid.execute(action);
            print_result(&result, human);
        }
        Command::Exec { json } => {
            exec::run(json, human);
        }
        Command::Run { file, delay } => {
            run::run(&file, delay, human);
        }
    }
}
