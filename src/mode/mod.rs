use ignore::Walk;
use std::{env, path::PathBuf, process::Command};

#[derive(Debug, Clone)]
pub enum Item {
    File(PathBuf),
    Command(String),
    Selection(String),
}

impl Item {
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

trait ItemDisplay {
    fn text(&self) -> String;
}

pub trait Mode {
    fn name(&self) -> &str;
    fn options(&mut self) -> Vec<Item>;
}

impl dyn Mode {
    pub fn matches(&mut self, input: &str) -> Vec<Item> {
        self.options()
            .into_iter()
            .filter(|i| i.text().contains(input))
            .take(100) // Max to display
            .collect()
    }
}

pub struct FileMode {
    root: PathBuf,
}

impl FileMode {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
}

impl Mode for FileMode {
    fn name(&self) -> &str {
        "Files"
    }

    fn options(&mut self) -> Vec<Item> {
        Walk::new(&self.root)
            .filter_map(Result::ok)
            .map(|entry| Item::File(entry.path().to_path_buf()))
            .collect()
    }
}

fn get_path_dirs() -> Vec<PathBuf> {
    let mut dirs = vec![];
    if let Ok(path) = env::var("PATH") {
        for dir in path.split(':') {
            dirs.push(PathBuf::from(dir));
        }
    }
    dirs
}

fn get_files(dir: PathBuf) -> Vec<String> {
    let Ok(read_dir) = dir.read_dir() else {
        return vec![];
    };
    read_dir
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_file() {
                return path.file_name().map(|n| n.to_string_lossy().to_string());
            }
            None
        })
        .collect()
}

pub struct RunMode;
impl Mode for RunMode {
    fn name(&self) -> &str {
        "Run"
    }

    fn options(&mut self) -> Vec<Item> {
        get_path_dirs()
            .into_iter()
            .flat_map(get_files)
            .map(Item::Command)
            .collect()
    }
}

pub struct DmenuMode {
    options: Vec<Item>,
}

impl DmenuMode {
    pub fn new(input: String) -> Self {
        let options = input
            .lines()
            .map(|s| Item::Selection(s.to_string()))
            .collect();
        Self { options }
    }
}

impl Mode for DmenuMode {
    fn name(&self) -> &str {
        "dmenu"
    }

    fn options(&mut self) -> Vec<Item> {
        self.options.clone()
    }
}
