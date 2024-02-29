use std::{
    env,
    path::{Path, PathBuf},
};

pub fn find_files_from_env<F: Fn(&Path) -> bool>(env_var: &str, filter: &F) -> Vec<PathBuf> {
    get_dirs(env_var)
        .into_iter()
        .map(|dir| get_files(dir, filter))
        .flatten()
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
    let Ok(read_dir) = dir.read_dir() else {
        return vec![];
    };
    read_dir
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let path = entry.path();
            println!("File: {path:?}");
            if path.is_file() && filter(path.as_path()) {
                return Some(path);
            }
            None
        })
        .collect()
}
