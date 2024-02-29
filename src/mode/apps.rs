use std::ffi::OsStr;

use super::Mode;
use crate::{item::Item, util};

pub struct AppsMode {
    options: Vec<Item>,
}

impl AppsMode {
    pub fn load() -> Self {
        Self {
            options: util::find_files_from_env("XDG_DATA_DIRS", &|path| {
                Some(OsStr::new("desktop")) == path.extension()
            })
            .into_iter()
            .map(|path| Item::Command(path.to_string_lossy().to_string()))
            .collect(),
        }
    }
}

impl Mode for AppsMode {
    fn name(&self) -> &str {
        "Applications"
    }

    fn options(&mut self) -> Vec<Item> {
        self.options.clone()
    }
}
