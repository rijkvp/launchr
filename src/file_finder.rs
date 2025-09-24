use ignore::{WalkBuilder, WalkState};
use rayon::prelude::*;
use std::{
    collections::HashSet,
    env,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    thread,
};
use walkdir::WalkDir;

use crate::item::{Action, Item};

pub fn find_files_from_dirs<F: Fn(&Path) -> bool + Sync>(
    dirs: &[PathBuf],
    filter: &F,
) -> Vec<PathBuf> {
    dirs.into_par_iter()
        .flat_map(|dir| get_files(dir, filter))
        .collect::<HashSet<PathBuf>>() // Not very clean, but it prevents duplicates
        .into_iter()
        .collect()
}

pub fn get_dirs_from_env(env_var: &str) -> Vec<PathBuf> {
    if let Ok(path) = env::var(env_var) {
        return path.split(':').map(PathBuf::from).collect();
    }
    vec![]
}

fn get_files<F>(dir: &Path, filter: F) -> Vec<PathBuf>
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

pub struct FileResult {
    text: String,
    dir_entry: ignore::DirEntry,
}

impl From<FileResult> for Item {
    fn from(value: FileResult) -> Self {
        let path = value.dir_entry.path();
        let is_dir = path.is_dir();
        Item::new(
            value.text,
            Action::File {
                path: value.dir_entry.into_path(),
                is_dir,
            },
        )
    }
}

pub fn find_all_files(root: &Path, files_tx: mpsc::Sender<FileResult>) {
    let start_time = std::time::Instant::now();
    let counter = AtomicUsize::new(0);
    let threads = thread::available_parallelism()
        .expect("failed to get parallelism")
        .get();
    WalkBuilder::new(root)
        .threads(threads)
        .build_parallel()
        .run(|| {
            Box::new(|path| {
                if let Ok(entry) = path {
                    if entry.path() == root {
                        return WalkState::Continue;
                    }
                    files_tx
                        .send(FileResult {
                            text: entry
                                .path()
                                .strip_prefix(root)
                                .unwrap()
                                .to_string_lossy()
                                .to_string(),
                            dir_entry: entry,
                        })
                        .unwrap();
                    counter.fetch_add(1, Ordering::Relaxed);
                }
                WalkState::Continue
            })
        });
    log::info!(
        "found {} files/directories in {:?}",
        counter.load(Ordering::Relaxed),
        start_time.elapsed()
    );
}
