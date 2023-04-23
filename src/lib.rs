pub use crate::commands::*;
pub use crate::configuration::*;
pub use crate::errors::*;
pub use crate::shell::*;
pub use crate::workspaces::*;

pub mod commands;
pub mod configuration;
pub mod errors;
pub mod shell;

// Set up general tests
#[cfg(test)]
mod tests;
pub mod workspaces;

pub fn run(config: Config) -> Result<(), WorkspaceError> {
    let file_config = match FileConfig::build() {
        Ok(val) => val,
        Err(err) => return Err(WorkspaceError::new(err, Severity::Error)),
    };

    // Initialize workspaces
    let workspaces = Workspaces::new(file_config.workspaces_file.as_str());

    // Initialize command
    return Command::run(workspaces, config.command.clone()).into();
}
