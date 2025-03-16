use super::SimpleMode;
use crate::{file_finder, item::Item};
use std::path::PathBuf;

pub struct FilesMode {
    root: PathBuf,
    files: Vec<Item>,
}

impl FilesMode {
    pub fn new(root: PathBuf) -> Self {
        // TODO: make multi-threaded
        Self {
            files: file_finder::find_all_files(&root),
            root,
        }
    }
}

impl SimpleMode for FilesMode {
    fn name(&self) -> &str {
        "Files"
    }

    fn get_items(&mut self) -> &Vec<Item> {
        &self.files
    }
}
