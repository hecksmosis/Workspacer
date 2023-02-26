use std::env;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::io::prelude::*;
use std::path::Path;

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

    pub fn clone(&self) -> Workspace {
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
        println!("Adding workspace {}", self.workspace_file.as_str());

        // Open the file in append mode without using openoptions
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
        println!("Removing workspace {}", self.workspace_file.as_str());

        // Open the file and clear it without using openoptions
        let mut file =
            fs::File::create(self.workspace_file.as_str()).expect("Unable to create file");

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
