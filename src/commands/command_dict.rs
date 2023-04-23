use crate::{CommandReturn, Config, Severity, Workspace, WorkspaceError, Workspaces};
use std::{io, io::Write, path::PathBuf};
use text_io::scan;

// Command functions
pub fn init(workspaces: Workspaces, name: String) -> CommandReturn {
    if let Some(workspace) = workspaces.workspaces.iter().find(|w| w.name == name) {
        workspace.init().into()
    } else {
        "Workspace not found".into()
    }
}

pub fn add(
    mut workspaces: Workspaces,
    name: String,
    path: PathBuf,
    shell_executable: Option<String>,
    text: Option<String>,
    cmd_path: Option<PathBuf>,
) -> CommandReturn {
    let shell = match shell_executable {
        Some(shell) => shell,
        None => {
            if cfg!(target_os = "windows") {
                return "cmd".into();
            } else {
                return "sh".into();
            }
        }
    };

    if let Some(command_text) = text {
        let workspace = Workspace::new(
            name,
            path,
            shell,
            command_text
                .split(";")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        );
        workspaces.add(workspace).into()
    } else if let Some(command_path) = cmd_path {
        let workspace = Workspace::new(
            name,
            path,
            shell,
            vec![command_path.to_str().unwrap().to_string()],
        );
        workspaces.add(workspace).into()
    } else {
        "You must provide a value for either text or file based init commands".into()
    }
}

// TODO: Filter by name or other criteria
pub fn list(workspaces: Workspaces) -> CommandReturn {
    if workspaces.workspaces.len() == 0 {
        return "No workspaces".into();
    }

    for workspace in workspaces.workspaces {
        println!(
            "Workspace: {}, shell: {}, with working directory: {}, runs: {}",
            workspace.name,
            workspace.shell,
            workspace.path.display(),
            workspace.init_commands.join(", ")
        )
    }
    .into()
}

pub fn delete(mut workspaces: Workspaces, name: Option<String>, confirm: bool) -> CommandReturn {
    if workspaces.workspaces.len() == 0 {
        return "No workspaces".into();
    }

    if let Some(workspace_name) = name {
        // delete the specified workspace
        if let Some(workspace) = workspaces
            .workspaces
            .clone()
            .iter()
            .find(|w| w.name == workspace_name)
        {
            if confirm {
                return workspaces.remove_from_file(workspace).into();
            }

            let input: String;
            print!(
                "Are you sure you want to delete workspace {}? (y/n) ",
                workspace_name
            );
            if let Err(err) = io::stdout().flush() {
                return err.to_string().as_str().into();
            }
            scan!("{}", input);
            if input != "y" {
                return WorkspaceError::new("Clear cancelled".into(), Severity::Message).into();
            }

            return workspaces.remove_from_file(workspace).into();
        } else {
            return "Workspace not found".into();
        }
    }

    if confirm {
        return workspaces.clear().into();
    }

    let input: String;
    print!("Are you sure you want to clear all workspaces? (y/n) ");
    if let Err(err) = io::stdout().flush() {
        return err.to_string().as_str().into();
    }
    scan!("{}", input);
    if input != "y" {
        return WorkspaceError::new("Clear cancelled".into(), Severity::Message).into();
    }

    workspaces.clear().into()
}

pub fn modify(
    mut workspaces: Workspaces,
    name: String,
    path: PathBuf,
    shell: Option<String>,
    text: Option<String>,
    cmd_path: Option<PathBuf>,
) -> CommandReturn {
    delete(workspaces.clone(), Some(name.clone()), false);
    workspaces.workspaces.retain(|w| w.name != name);
    add(workspaces, name, path, shell, text, cmd_path).into()
}

// pub fn set_config(config: Config) -> CommandReturn {
//     if let Some(name) = config.name {
//         if let Some(args) = config.args {
//             if name == "save_dir" {
//                 if args.len() == 1 {
//                     FileConfig::set_save_dir(args[0].clone()).into()
//                 } else {
//                     "save_dir is not set".into()
//                 }
//             } else if name == "workspaces_file" {
//                 if args.len() == 1 {
//                     FileConfig::set_workspaces_file(args[0].clone()).into()
//                 } else {
//                     "workspaces_file is not set".into()
//                 }
//             } else if name == "test_workspaces_file" {
//                 if args.len() == 1 {
//                     FileConfig::set_test_workspaces_file(args[0].clone()).into()
//                 } else {
//                     "test_workspaces_file is not set".into()
//                 }
//             } else {
//                 "Invalid config option".into()
//             }
//         } else {
//             "Config option is required".into()
//         }
//     } else {
//         "Config option is required".into()
//     }
// }

pub fn help(config: Config) -> CommandReturn {
    if let Some(args) = config.args {
        if args.len() > 1 {
            println!("Help for {}", args[1]).into()
        } else {
            println!("Help for all commands").into()
        }
    } else {
        println!("Help for all commands").into()
    }
}
