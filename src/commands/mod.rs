use crate::{CommandReturn, Workspaces};
use clap::Subcommand;
pub use command_dict::*;
use std::fmt::{Display, Formatter};

pub mod command_dict;

#[derive(Debug, Subcommand, Clone)]
pub enum Command {
    /// Initialize a workspace
    Init {
        /// Name of the workspace to initialize
        name: String,
    },
    /// Adds a workspace to the list
    Add {
        /// Name of the workspace to add
        name: String,

        /// The path to the workspace (this will be the directory that will be cd'd into)
        path: std::path::PathBuf,

        /// The shell to execute
        #[clap(short = 's', long = "shell")]
        shell: Option<String>,

        /// The init commands in text form
        #[clap(short = 't', long = "text")]
        text: Option<String>,

        /// The init commands in file form
        #[clap(short = 'c', long = "cmd-path")]
        command_path: Option<std::path::PathBuf>,
    },
    /// List all workspaces
    List,
    /// Delete a workspace or clear all workspaces
    Delete {
        /// Name of the workspace to clear
        name: Option<String>,

        #[clap(short = 'y', long = "yes")]
        confirm: bool,
    },
    /// Modifies a workspace, changing its attributes
    Modify {
        /// The name of the workspace to modify
        name: String,

        /// The path to the workspace (this will be the directory that will be cd'd into)
        path: std::path::PathBuf,

        /// The shell to execute
        #[clap(short = 's', long = "shell")]
        shell: Option<String>,

        /// The init commands in text form
        #[clap(short = 't', long = "text")]
        text: Option<String>,

        /// The init commands in file form
        #[clap(short = 'c', long = "cmd-path")]
        command_path: Option<std::path::PathBuf>,
    },
    /// => init
    Run { name: String },
    // Config(fn(Config) -> CommandReturn),
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Init { .. } => write!(f, "init"),
            Command::Add { .. } => write!(f, "add"),
            Command::List { .. } => write!(f, "list"),
            Command::Delete { .. } => write!(f, "clear"),
            Command::Modify { .. } => write!(f, "modify"),
            Command::Run { .. } => write!(f, "run"),
            // Command::Config(_) => write!(f, "config"),
        }
    }
}

impl Command {
    pub fn run(workspaces: Workspaces, command: Command) -> CommandReturn {
        match command {
            Command::Init { name } => init(workspaces, name),
            Command::Add {
                name,
                path,
                shell,
                text,
                command_path,
            } => add(workspaces, name, path, shell, text, command_path),
            Command::List {} => list(workspaces),
            Command::Delete { name, confirm } => delete(workspaces, name, confirm),
            Command::Run { name } => init(workspaces, name),
            Command::Modify {
                name,
                path,
                shell,
                text,
                command_path,
            } => modify(workspaces, name, path, shell, text, command_path),
            // Command::Config(set_config) => set_config(config),
        }
    }

    // pub fn run_string(command: String) -> CommandReturn {
    //     let config = Config::new(command.into());
    //
    //     match Command::from(config.command.as_str()) {
    //         Command::Config(set_config) => set_config(config),
    //         _ => return CommandReturn::from("Runnable command not implemented"),
    //     }
    // }
}
