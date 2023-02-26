use std::env;

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
