use std::path::PathBuf;

use super::Mode;
use crate::item::Item;
use ignore::Walk;

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
