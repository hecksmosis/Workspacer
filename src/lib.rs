use std::env;
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub struct Config {
    pub command: String,
    pub ignore_case: bool,
    pub name: Option<String>,
    pub path: Option<String>,
    pub init_commands: Option<Vec<String>>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let command = args[1].clone();
        let name = if args.len() > 2 {
            Some(args[2].clone())
        } else {
            None
        };
        let path = if args.len() > 3 {
            Some(args[3].clone())
        } else {
            None
        };
        let init_commands = if args.len() > 4 {
            Some(
                args[4..]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            )
        } else {
            None
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            command,
            name,
            path,
            init_commands,
            ignore_case,
        })
    }
}

pub struct Workspace {
    pub name: String,
    pub path: String,
    pub init_commands: Vec<String>,
}

impl Workspace {
    pub fn new(name: String, path: String, init_commands: Vec<String>) -> Workspace {
        Workspace {
            name,
            path,
            init_commands,
        }
    }

    pub fn init(&self) {
        println!("Initializing workspace {} at {}", self.name, self.path);
        for command in &self.init_commands {
            // run the command in the corresponding terminal to the os
            if cfg!(target_os = "windows") {
                let output = std::process::Command::new("cmd")
                    .args(&["/C", command])
                    .output()
                    .expect("failed to execute process");
                println!("{}", String::from_utf8_lossy(&output.stdout));
            } else {
                let output = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(command)
                    .output()
                    .expect("failed to execute process");
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
        }
    }

    fn clone(&self) -> Workspace {
        Workspace {
            name: self.name.clone(),
            path: self.path.clone(),
            init_commands: self.init_commands.clone(),
        }
    }
}

impl PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.path == other.path
    }
}

impl Debug for Workspace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Workspace")
            .field("name", &self.name)
            .field("path", &self.path)
            .field("init_commands", &self.init_commands)
            .finish()
    }
}

pub struct Workspaces {
    pub active_workspace: Option<Workspace>,
    pub workspaces: Vec<Workspace>,
    pub workspace_file: String,
}

impl Workspaces {
    pub fn new(file_path: &str) -> Workspaces {
        // If file doesn't exist, create it
        if !Path::new(file_path).exists() {
            fs::File::create(file_path).expect("Unable to create file");
        }

        read_from_file(file_path)
    }

    pub fn add(&mut self, workspace: Workspace) {
        self.add_to_file(&workspace);
        self.workspaces.push(workspace);
    }

    pub fn init(&self) {
        for workspace in &self.workspaces {
            workspace.init();
        }
    }

    pub fn add_to_file(&self, workspace: &Workspace) {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(self.workspace_file.as_str())
            .expect("Unable to open file");

        // If the workspace already exists, print a message to the console
        if self
            .workspaces
            .iter()
            .any(|w| w.name == workspace.name && w.path == workspace.path)
        {
            println!("Workspace already exists");
            return;
        }

        // Write everything into file, including init commands
        if let Err(e) = writeln!(
            file,
            "{};{};{}",
            workspace.name,
            workspace.path,
            workspace.init_commands.join(";")
        ) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    pub fn clear(&mut self) {
        self.workspaces.clear();
        self.active_workspace = None;

        fs::write(self.workspace_file.as_str(), "").expect("Unable to clear file");
    }

    pub fn remove_from_file(&mut self, workspace: &Workspace) {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(self.workspace_file.as_str())
            .expect("Unable to open file");

        // Remove the workspace from the vector
        self.workspaces
            .retain(|w| w.name != workspace.name && w.path != workspace.path);

        // Write everything into file, including init commands
        for workspace in &self.workspaces {
            if let Err(e) = writeln!(
                file,
                "{};{};{}",
                workspace.name,
                workspace.path,
                workspace.init_commands.join(";")
            ) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }
}

pub fn read_from_file(file_path: &str) -> Workspaces {
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut workspaces = Vec::new();
    let mut active_workspace = None;

    for line in contents.lines() {
        let mut parts = line.split(";");
        let name = parts.next().unwrap();
        let path = parts.next().unwrap();
        let init_commands = parts.map(|s| s.to_string()).collect();

        let workspace = Workspace::new(name.to_string(), path.to_string(), init_commands);

        // If the current path is equal to the workspace path or is a subfolder, set it as active
        if let Ok(current_path) = env::current_dir() {
            if current_path.starts_with(&workspace.path) {
                active_workspace = Some(workspace.clone());
            }
        }
        workspaces.push(workspace);
    }

    Workspaces {
        active_workspace,
        workspaces,
        workspace_file: file_path.to_string(),
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut workspaces = Workspaces::new("workspaces.txt");

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

// Create tests for add, remove and read_from_file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let mut workspaces = Workspaces::new("test-workspaces.txt");
        let workspace = Workspace::new("test".to_string(), "test".to_string(), vec![]);
        workspaces.add(workspace.clone());

        assert_eq!(workspaces.workspaces[0], workspace);
    }

    #[test]
    fn remove() {
        let mut workspaces = Workspaces::new("test-workspaces.txt");
        let workspace = Workspace::new("test".to_string(), "test".to_string(), vec![]);
        workspaces.add(workspace.clone());
        workspaces.remove_from_file(&workspace);

        assert_eq!(workspaces.workspaces.len(), 0);
    }

    #[test]
    fn read_from_file() {
        let workspaces = Workspaces::new("test-workspaces.txt");
        assert_eq!(workspaces.workspaces.len(), 0);
    }
}
