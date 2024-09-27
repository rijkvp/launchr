use std::path::{Path, PathBuf};

use super::Mode;
use crate::item::{Item, ItemType};
use ignore::Walk;

pub struct FilesMode {
    root: PathBuf,
    files: Option<Vec<Item>>,
}

impl FilesMode {
    pub fn new(root: PathBuf) -> Self {
        Self { root, files: None }
    }

    fn load_files(root: &Path) -> Vec<Item> {
        let start_time = std::time::Instant::now();
        let items: Vec<Item> = Walk::new(&root)
            .filter_map(Result::ok)
            .map(|entry| {
                Item::new(
                    entry.path().to_string_lossy().to_string(),
                    // TODO: Investigate whether checking for is_dir is necessary since it can be
                    // very slow
                    ItemType::File { is_dir: false },
                )
            })
            .collect();
        log::info!("found {} files in {:?}", items.len(), start_time.elapsed());
        items
    }
}

impl Mode for FilesMode {
    fn name(&self) -> &str {
        "Files"
    }

    fn options(&mut self) -> &Vec<Item> {
        self.files
            .get_or_insert_with(|| Self::load_files(&self.root))
    }
}
