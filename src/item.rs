use std::{
    fmt::{self, Display, Formatter},
    process::Command,
};

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
pub struct Item {
    text: String,
    item_type: ItemType,
}

#[derive(Debug, Clone)]
pub enum ItemType {
    Selection,
    Exec { exec: Exec },
    File { is_dir: bool },
}

impl AsRef<str> for Item {
    fn as_ref(&self) -> &str {
        &self.text
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.item_type {
            ItemType::Selection => write!(f, "{}", self.text),
            ItemType::Exec { .. } => write!(f, "[EXEC] {}", self.text),
            ItemType::File { is_dir } => {
                write!(f, "[{}] {}", if is_dir { "DIR" } else { "FILE" }, self.text)
            }
        }
    }
}

impl Item {
    pub fn new(text: String, item_type: ItemType) -> Self {
        Self { text, item_type }
    }

    pub fn new_selection(text: String) -> Self {
        Self::new(text, ItemType::Selection)
    }

    pub fn exec(&self) {
        match self.item_type {
            ItemType::Selection => {
                // Print the selection
                println!("{}", self.text);
            }
            ItemType::File { .. } => {
                // Open the file using default software
                log::info!("opening file: {}", self.text);
                if let Err(e) = open::that(&self.text) {
                    eprintln!("Failed to open {}: {}", self.text, e);
                }
            }
            ItemType::Exec { ref exec } => {
                // Execute the command as child process
                let cmd = exec.command();
                log::info!("executing: '{cmd}'");
                if let Err(e) = Command::new(&exec.program).args(&exec.args).spawn() {
                    eprintln!("Failed to run '{cmd}': {e}");
                }
            }
        }
    }
}
