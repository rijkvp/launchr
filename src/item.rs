use std::{path::PathBuf, process::Command};

#[derive(Debug, Clone)]
pub struct Exec {
    pub program: String,
    pub args: Vec<String>,
}

impl Exec {
    pub fn command(&self) -> String {
        let mut cmd = String::from(&self.program);
        if self.args.len() > 0 {
            cmd.push(' ');
            cmd.push_str(&self.args.join(" "));
        }
        cmd
    }
}

#[derive(Debug, Clone)]
pub enum Item {
    File(PathBuf),
    Exec { name: String, exec: Exec },
    Selection(String),
}

impl Item {
    pub fn display(&self) -> String {
        match self {
            Item::File(path) => {
                format!(
                    "[{}] {}",
                    if path.is_dir() { "DIR " } else { "FILE" },
                    path.file_name().unwrap().to_string_lossy()
                )
            }
            Item::Exec { name, .. } => name.to_string(),
            Item::Selection(s) => s.to_string(),
        }
    }

    pub fn text(&self) -> String {
        match self {
            Item::File(path) => path.file_name().unwrap().to_string_lossy().to_string(),
            Item::Exec { name, .. } => name.to_string(),
            Item::Selection(s) => s.to_string(),
        }
    }

    pub fn exec(&self) {
        match self {
            Item::File(path) => {
                // Open the file using default software
                log::info!("opening file: {}", path.display());
                if let Err(e) = open::that(&path) {
                    eprintln!("Failed to open {}: {}", path.display(), e);
                }
            }
            Item::Exec { name: _, exec } => {
                // Execute the command as child process
                let cmd = exec.command();
                log::info!("executing: '{cmd}'");
                if let Err(e) = Command::new(&exec.program).args(&exec.args).spawn() {
                    eprintln!("Failed to run '{cmd}': {e}");
                }
            }
            Item::Selection(_) => {
                // Print the selection
                println!("{}", self.text());
            }
        }
    }
}
