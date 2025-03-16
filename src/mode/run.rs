use super::SimpleMode;
use crate::{
    file_finder,
    item::{Action, Exec, Item},
};

pub struct RunMode {
    executables: Vec<Item>,
}

impl RunMode {
    pub fn load() -> Self {
        // TODO: Filter on executable files
        Self {
            executables: file_finder::find_files_from_env("PATH", &|_| true)
                .into_iter()
                .map(|path| {
                    Item::new(
                        path.file_name().unwrap().to_string_lossy().to_string(),
                        Action::Exec {
                            exec: Exec {
                                program: path.to_string_lossy().to_string(),
                                args: Vec::new(),
                            },
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
