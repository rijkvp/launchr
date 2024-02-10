use std::{env, path::PathBuf};

use ignore::Walk;

pub trait Mode {
    fn name(&self) -> &str;
    fn run(&mut self, input: &str) -> Vec<String>;
}

pub struct FileMode;
impl Mode for FileMode {
    fn name(&self) -> &str {
        "Files"
    }

    fn run(&mut self, input: &str) -> Vec<String> {
        let home_dir = dirs::home_dir().unwrap();
        Walk::new(home_dir)
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let path = entry.path();
                if !path.is_file() {
                    return None;
                }
                let name = path.file_name().and_then(|name| name.to_str());
                if let Some(name) = name {
                    if name.contains(input) {
                        return Some(path.display().to_string());
                    }
                }
                None
            })
            .take(10)
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

    fn run(&mut self, input: &str) -> Vec<String> {
        get_path_dirs()
            .into_iter()
            .flat_map(get_files)
            .filter(|p| p.starts_with(input))
            .collect()
    }
}
