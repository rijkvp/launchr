use std::{path::PathBuf, process::Command};

#[derive(Debug, Clone)]
pub enum Item {
    File(PathBuf),
    Command(String),
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
            Item::Command(cmd) => cmd.to_string(),
            Item::Selection(s) => s.to_string(),
        }
    }

    pub fn text(&self) -> String {
        match self {
            Item::File(path) => path.file_name().unwrap().to_string_lossy().to_string(),
            Item::Command(cmd) => cmd.to_string(),
            Item::Selection(s) => s.to_string(),
        }
    }

    pub fn exec(&self) {
        match self {
            Item::File(path) => {
                // Open the file using default software
                if let Err(e) = open::that(&path) {
                    eprintln!("Failed to open {}: {}", path.display(), e);
                }
            }
            Item::Command(cmd) => {
                // Run the command
                if let Err(e) = Command::new(&cmd).spawn() {
                    eprintln!("Failed to run {}: {}", cmd, e);
                }
            }
            Item::Selection(_) => {
                // Print the selection
                println!("{}", self.text());
            }
        }
    }
}
