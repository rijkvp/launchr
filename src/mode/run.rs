use super::SimpleMode;
use crate::{
    file_finder,
    item::{Action, Item},
};
use std::path::PathBuf;

pub struct RunMode {
    executables: Vec<Item>,
}

impl RunMode {
    pub fn load() -> Self {
        // TODO: Filter on executable files
        let path_dirs = std::env::var("PATH")
            .expect("PATH is not set")
            .split(':')
            .map(PathBuf::from)
            .collect::<Vec<PathBuf>>();
        Self {
            executables: file_finder::find_files_from_dirs(&path_dirs, &|_| true)
                .into_iter()
                .map(|path| {
                    Item::new(
                        path.file_name().unwrap().to_string_lossy().to_string(),
                        Action::Exec {
                            program: path.to_string_lossy().to_string(),
                            args: Vec::new(),
                            terminal: false,
                        },
                    )
                })
                .collect(),
        }
    }
}

impl SimpleMode for RunMode {
    fn name(&self) -> &str {
        "Run"
    }

    fn get_items(&mut self) -> &Vec<Item> {
        &self.executables
    }
}
