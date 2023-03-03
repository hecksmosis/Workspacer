use std::env;

pub struct Config {
    pub command: String,
    pub name: Option<String>,
    pub path: Option<String>,
    pub init_commands: Option<Vec<String>>,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let command = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a command string"),
        };

        let name = match args.next() {
            Some(arg) => Some(arg),
            None => None,
        };

        let path = match args.next() {
            Some(arg) => Some(arg),
            None => None,
        };

        let init_commands = match args.next() {
            Some(arg) => Some(arg.split(",").map(|s| s.to_string()).collect()),
            None => None,
        };

        Ok(Config {
            command,
            name,
            path,
            init_commands,
        })
    }
}

pub struct FileConfig {
    pub save_dir: Option<String>,
    pub workspaces_file: Option<String>,
}

impl FileConfig {
    pub fn build(test: bool) -> Result<FileConfig, &'static str> {
        let save_dir = env::var("SAVE_DIR").ok();
        let workspaces_file = match test {
            false => env::var("WORKSPACES_FILE").ok(),
            true => env::var("TEST_WORKSPACES_FILE").ok(),
        };

        Ok(FileConfig {
            save_dir,
            workspaces_file,
        })
    }
}
