use super::Mode;
use crate::{file_finder, item::Item};
use std::path::PathBuf;

pub struct FilesMode {
    root: PathBuf,
    files: Vec<Item>,
}

impl FilesMode {
    pub fn new(root: PathBuf) -> Self {
        Self {
            files: file_finder::find_all_files(&root),
            root,
        }
    }
}

impl Mode for FilesMode {
    fn name(&self) -> &str {
        "Files"
    }

    fn options(&mut self) -> &Vec<Item> {
        &self.files
    }

    fn exec(&self, item: &Item) {
        // Open the file using default software
        log::info!("opening: {}", item);
        if let Err(e) = open::that_detached(self.root.join(item.as_ref())) {
            eprintln!("Failed to open {}: {}", item, e);
        }
    }
}
