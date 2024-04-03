use std::{path::PathBuf, process::Command};

#[derive(Debug, Clone)]
pub enum Item {
    File(PathBuf),
    Exec { name: String, exec: String },
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
                log::info!("Opening file: {}", path.display());
                if let Err(e) = open::that(&path) {
                    eprintln!("Failed to open {}: {}", path.display(), e);
                }
            }
            Item::Exec { name: _, exec } => {
                // Execute the command
                log::info!("Executing: '{exec}'");
                if let Err(e) = Command::new(&exec).spawn() {
                    eprintln!("Failed to run {}: {}", exec, e);
                }
            }
            Item::Selection(_) => {
                // Print the selection
                println!("{}", self.text());
            }
        }
    }
}
