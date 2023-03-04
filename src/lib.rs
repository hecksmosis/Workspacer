use crate::commands::*;
use crate::configuration::*;
use crate::workspaces::*;
use std::error::Error;

pub mod commands;
pub mod configuration;
#[cfg(test)]
mod tests;
pub mod workspaces;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_config = FileConfig::build(false)?;

    let workspaces = Workspaces::new(
        file_config
            .workspaces_file
            .expect("Environment variable WORKSPACE_FILE not set")
            .as_str(),
    );

    let mut command_manager =
        match CommandConfigurator::build(serialize_commands(), workspaces, config.clone()) {
            Some(command_manager) => command_manager,
            None => return Err("No commands found".into()),
        };

    command_manager.run(config.command);

    Ok(())
}
