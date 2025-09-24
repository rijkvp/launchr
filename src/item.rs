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
    Exec {
        program: String,
        args: Vec<String>,
        terminal: bool,
    },
    File {
        path: PathBuf,
        is_dir: bool,
    },
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
            Action::Exec { program, args, .. } => write!(
                f,
                "{} ({})",
                self.text,
                std::iter::once(program.as_str())
                    .chain(args.iter().map(String::as_str))
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
            Action::Exec {
                program,
                args,
                terminal,
            } => {
                // Execute the command as child process
                if *terminal {
                    let mut cmd = program.clone();
                    if !args.is_empty() {
                        cmd.push(' ');
                        cmd.push_str(&args.join(" "));
                    }
                    log::info!("running command in terminal: {cmd}");
                    // TODO: Make terminal configurable
                    if let Err(e) = Command::new("alacritty").args(["-e", &cmd]).spawn() {
                        eprintln!("Failed to run command in terminal: {e}");
                    }
                } else {
                    log::info!("running program: {program}");
                    if let Err(e) = Command::new(&program).args(args).spawn() {
                        eprintln!("Failed to run command': {e}");
                    }
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
