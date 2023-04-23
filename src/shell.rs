use std::path::PathBuf;

pub struct Shell {
    pub working_dir: String,
    pub name: String,
}

impl Shell {
    pub fn new(wdir: &PathBuf, shell: &str) -> Self {
        let working_dir = wdir.to_str().expect("Invalid path").to_string();
        Self {
            working_dir,
            name: shell.to_string(),
        }
    }

    pub fn get_input(&self) {
        self.run_command("".to_string());
    }

    pub fn run_command(&self, command: String) {
        let mut cmd = std::process::Command::new(self.name.as_str());
        if self.name.as_str() == "pwsh.exe" {
            cmd.arg("-NoLogo");
        } else {
            cmd.arg("-c");
            cmd.arg(command);
        }
        match self.name.as_str() {
            "pwsh.exe" => {
                cmd.arg("-wd");
                cmd.arg(self.working_dir.as_str());
            }
            _ => {
                cmd.current_dir(self.working_dir.as_str());
            }
        }
        cmd.status().unwrap();
    }
}
