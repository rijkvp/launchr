use crate::item::{Action, Item};
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::{
    collections::HashSet,
    env,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn find_files_from_env<F: Fn(&Path) -> bool + Sync>(env_var: &str, filter: &F) -> Vec<PathBuf> {
    get_dirs(env_var)
        .into_par_iter()
        .flat_map(|dir| get_files(dir, filter))
        .collect::<HashSet<PathBuf>>() // Not very clean, but it prevents duplicates
        .into_iter()
        .collect()
}

fn get_dirs(env_var: &str) -> Vec<PathBuf> {
    if let Ok(path) = env::var(env_var) {
        return path.split(':').map(|d| PathBuf::from(d)).collect();
    }
    vec![]
}

fn get_files<F>(dir: PathBuf, filter: F) -> Vec<PathBuf>
where
    F: Fn(&Path) -> bool,
{
    WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|e| {
            let p = e.path();
            if filter(p) {
                Some(p.to_path_buf())
            } else {
                None
            }
        })
        .collect()
}

pub fn find_all_files(root: &Path) -> Vec<Item> {
    let start_time = std::time::Instant::now();
    let mut items: Vec<Item> = WalkBuilder::new(&root)
        .build()
        .filter_map(Result::ok)
        .map(|entry| {
            Item::new(
                entry
                    .path()
                    .strip_prefix(&root)
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                Action::File {
                    path: root.join(entry.path()),
                    is_dir: entry.file_type().map(|f| f.is_dir()).unwrap_or(false),
                },
            )
        })
        .collect();
    items.swap_remove(0); // Remove the root directory
    log::info!(
        "found {} files/directories in {:?}",
        items.len(),
        start_time.elapsed()
    );
    items
}
