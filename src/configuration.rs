use crate::Command;
use clap::Parser;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub enum EnvVar {
    SaveDir,
    WorkspacesFile,
    TestWorkspacesFile,
}

impl ToString for EnvVar {
    fn to_string(&self) -> String {
        match self {
            EnvVar::SaveDir => "SAVE_DIR".to_string(),
            EnvVar::WorkspacesFile => "WORKSPACES_FILE".to_string(),
            EnvVar::TestWorkspacesFile => "TEST_WORKSPACES_FILE".to_string(),
        }
    }
}

#[derive(Parser, Debug)]
pub struct Config {
    #[clap(subcommand)]
    /// The command to run
    pub command: Command,

    #[clap(short = 'n', long = "name")]
    /// Name of the workspace to run the command on
    pub name: Option<String>,

    #[clap(short = 'p', long = "path")]
    /// Path of the file to run (used with the add command to use the init commands from a file)
    pub path: Option<std::path::PathBuf>,

    // TODO
    pub args: Option<Vec<String>>,
}

impl Clone for Config {
    fn clone(&self) -> Self {
        Config {
            command: self.command.clone(),
            name: self.name.clone(),
            path: self.path.clone(),
            args: self.args.clone(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FileConfig {
    pub save_dir: String,
    pub workspaces_file: String,
    pub test_workspaces_file: String,
}

// General environment functions
impl FileConfig {
    pub fn build() -> Result<Self, String> {
        if let Some(user_dir) = ProjectDirs::from("dev", "ws", "ws") {
            let config_dir = user_dir.config_dir();
            fs::create_dir_all(&config_dir).unwrap();

            println!("Config dir: {:?}", config_dir);

            let config_file: FileConfig = fs::read_to_string(config_dir.join("configuration.toml"))
                .map(|file| toml::from_str(&file).unwrap())
                .unwrap_or_else(|_| {
                    let config_file_path = config_dir.join("configuration.toml");
                    let file = File::create(&config_file_path)
                        .map(|mut file| {
                            let config_file = FileConfig {
                                save_dir: config_dir.to_str().unwrap().to_string(),
                                workspaces_file: config_dir
                                    .join("workspaces.txt")
                                    .to_str()
                                    .unwrap()
                                    .to_string(),
                                test_workspaces_file: config_dir
                                    .join("test_workspaces.txt")
                                    .to_str()
                                    .unwrap()
                                    .to_string(),
                            };

                            let config_file_toml = toml::to_string(&config_file).unwrap();
                            println!("Config file: {:?}", config_file_toml);
                            file.write_all(config_file_toml.as_bytes()).unwrap();

                            println!("Created config file at {:?}", config_file_path);
                            return config_file;
                        })
                        .expect("Could not create config file");
                    file
                });

            if !Path::new(&config_file.save_dir).is_dir() {
                match fs::create_dir_all(&config_file.save_dir) {
                    Ok(_) => {}
                    Err(e) => return Err(format!("{:?}", e)),
                };
            }

            return Ok(config_file);
        } else {
            return Err("Could not find project directories".to_string());
        }
    }
}
