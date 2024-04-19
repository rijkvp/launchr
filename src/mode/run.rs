use super::Mode;
use crate::{
    item::{Exec, Item},
    util,
};

pub struct RunMode {
    options: Vec<Item>,
}

impl RunMode {
    pub fn load() -> Self {
        // TODO: Filter on executable files
        Self {
            options: util::find_files_from_env("PATH", &|_| true)
                .into_iter()
                .map(|path| Item::Exec {
                    name: path.file_name().unwrap().to_string_lossy().to_string(),
                    exec: Exec {
                        program: path.to_string_lossy().to_string(),
                        args: Vec::new(),
                    },
                })
                .collect(),
        }
    }
}
impl Mode for RunMode {
    fn name(&self) -> &str {
        "Run"
    }

    fn options(&mut self) -> Vec<Item> {
        self.options.clone()
    }
}
