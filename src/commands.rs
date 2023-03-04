use crate::workspaces::{Workspace, Workspaces};
use crate::Config;

pub fn serialize_commands() -> Vec<Command> {
    vec![
        Command::new("help".to_string(), help),
        Command::new("init".to_string(), init),
        Command::new("add".to_string(), add),
        Command::new("list".to_string(), list),
        Command::new("clear".to_string(), clear),
        Command::new("modify".to_string(), modify),
    ]
}

pub struct CommandConfigurator {
    pub commands: Vec<Command>,
    pub workspaces_instance: Workspaces,
    pub config: Config,
}

impl CommandConfigurator {
    pub fn new(instance: Workspaces, config: Config) -> CommandConfigurator {
        CommandConfigurator {
            commands: Vec::new(),
            workspaces_instance: instance,
            config,
        }
    }

    pub fn build(
        commands: Vec<Command>,
        instance: Workspaces,
        config: Config,
    ) -> Option<CommandConfigurator> {
        if commands.is_empty() {
            None
        } else {
            Some(CommandConfigurator {
                commands,
                workspaces_instance: instance,
                config,
            })
        }
    }

    pub fn run(&mut self, command: String) {
        let command = match self.commands.iter().find(|c| c.name == command) {
            Some(command) => command,
            None => &self.commands[0], // help command
        };

        (command.function)(self.workspaces_instance.clone(), self.config.clone());
    }

    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn add_commands(&mut self, commands: Vec<Command>) {
        self.commands.extend(commands);
    }

    pub fn get_commands(&self) -> Vec<Command> {
        self.commands.clone()
    }
}

// Command is a class that has a unique identifier, and a runnable function the user can call
pub struct Command {
    pub name: String,
    pub function: fn(Workspaces, Config),
}

impl Command {
    pub fn new(name: String, function: fn(Workspaces, Config)) -> Command {
        Command { name, function }
    }
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Clone for Command {
    fn clone(&self) -> Self {
        Command {
            name: self.name.clone(),
            function: self.function,
        }
    }
}

// Command functions
fn init(workspaces: Workspaces, _: Config) {
    match workspaces.active_workspace {
        Some(workspace) => workspace.init(),
        None => {}
    };
}

fn add(mut workspaces: Workspaces, config: Config) {
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

fn list(workspaces: Workspaces, _: Config) {
    for workspace in workspaces.workspaces {
        println!(
            "{}: {}, runs: {}",
            workspace.name,
            workspace.path,
            workspace.init_commands.join(", ")
        );
    }
}

fn clear(mut workspaces: Workspaces, _: Config) {
    workspaces.clear();
}

fn modify(mut workspaces: Workspaces, config: Config) {
    if let Some(name) = config.name {
        if let Some(args) = config.args {
            if args.len() > 0 {
                let workspace = Workspace::new(name, args[1], args[2..].to_vec());
                // Remove the old workspace from the file and add the new one
                workspaces.remove_from_file(&workspace);

                workspaces.add(workspace);
            } else {
                println!("Path and init commands are required");
            }
        } else {
            config.args = Some(vec!["modify".to_string()]);
            help(workspaces, config)
        }
    } else {
        println!("Name is required");
    }
}

fn help(_: Workspaces, config: Config) {
    // Print a general help message if there are no args or a specific help message if there are args
    if let Some(args) = config.args {
        if args.len() > 1 {
            println!("Help for {}", args[1]);
        } else {
        }
    } else {
    }
}
