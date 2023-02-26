use crate::configuration::*;
use crate::workspaces::*;
use std::error::Error;

// Create tests for add, remove and read_from_file
pub mod configuration;
#[cfg(test)]
mod tests;
pub mod workspaces;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_config = FileConfig::build(false)?;

    let mut workspaces = Workspaces::new(
        file_config
            .workspaces_file
            .expect("Environment variable WORKSPACE_FILE not set")
            .as_str(),
    );

    match config.command.as_str() {
        "add" => {
            if let Some(name) = config.name {
                if let Some(path) = config.path {
                    let workspace = Workspace::new(name, path, config.init_commands.unwrap());
                    workspaces.add(workspace);
                } else {
                    println!("Path is required");
                }
            } else {
                println!("Name is required");
            }
        }
        "init" => {
            match workspaces.active_workspace {
                Some(workspace) => workspace.init(),
                None => {}
            };
        }
        "list" => {
            for workspace in &workspaces.workspaces {
                println!(
                    "{}: {}, runs: {}",
                    workspace.name,
                    workspace.path,
                    workspace.init_commands.join(", ")
                );
            }
        }
        "clear" => {
            workspaces.clear();
        }
        "modify" => {
            if let Some(name) = config.name {
                if let Some(path) = config.path {
                    let workspace = Workspace::new(name, path, config.init_commands.unwrap());
                    // Remove the old workspace from the file and add the new one
                    workspaces.remove_from_file(&workspace);

                    workspaces.add(workspace);
                } else {
                    println!("Path is required");
                }
            } else {
                println!("Name is required");
            }
        }
        _ => {
            println!("Unknown command");
        }
    }

    Ok(())
}
