use std::{
    collections::HashSet,
    env,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

pub fn find_files_from_env<F: Fn(&Path) -> bool>(env_var: &str, filter: &F) -> Vec<PathBuf> {
    get_dirs(env_var)
        .into_iter()
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
        .filter(|e| {
            let p = e.path();
            // Filter & ignore hidden files
            p.is_file()
                && filter(p)
                && !e
                    .file_name()
                    .to_str()
                    .map(|s| s.starts_with('.'))
                    .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}
