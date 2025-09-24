use std::{
    fmt::{self, Display, Formatter},
    path::PathBuf,
    process::Command,
};

use bincode::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub struct Item {
    pub text: String,
    pub action: Action,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub enum Action {
    Selection,
    Exec { exec: Exec },
    File { path: PathBuf, is_dir: bool },
}

impl AsRef<str> for Item {
    fn as_ref(&self) -> &str {
        &self.text
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.action {
            Action::Selection => write!(f, "{}", self.text),
            Action::Exec { exec } => write!(
                f,
                "{} ({})",
                self.text,
                std::iter::once(exec.program.as_str())
                    .chain(exec.args.iter().map(String::as_str))
                    .collect::<Vec<_>>()
                    .join(" "),
            ),
            Action::File { is_dir, .. } => {
                write!(f, "{} {}", if *is_dir { 'D' } else { 'F' }, self.text)
            }
        }
    }
}

impl Item {
    pub fn new(text: String, action: Action) -> Self {
        Self { text, action }
    }

    pub fn new_selection(text: String) -> Self {
        Self::new(text, Action::Selection)
    }

    pub fn exec(&self) {
        match &self.action {
            Action::Exec { exec } => {
                // Execute the command as child process
                let cmd = exec.command();
                log::info!("executing: '{cmd}'");
                if let Err(e) = Command::new(&exec.program).args(&exec.args).spawn() {
                    eprintln!("Failed to run '{cmd}': {e}");
                }
            }
            Action::Selection => {
                // Print the selected item
                println!("{}", self.text);
            }
            Action::File { path, .. } => {
                // Open the file using default software
                log::info!("opening: '{}'", path.display());
                if let Err(e) = open::that_detached(&path) {
                    eprintln!("Failed to open '{}': {}", path.display(), e);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, Hash)]
pub struct Exec {
    pub program: String,
    pub args: Vec<String>,
}

impl Exec {
    pub fn command(&self) -> String {
        let mut cmd = String::from(&self.program);
        if !self.args.is_empty() {
            cmd.push(' ');
            cmd.push_str(&self.args.join(" "));
        }
        cmd
    }
}
