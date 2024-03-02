use std::{ffi::OsStr, fs::File, io::BufReader, path::Path, time::Instant};
use log::info;
use super::Mode;
use crate::{item::Item, util};
use std::io::BufRead;

pub struct AppsMode {
    options: Vec<Item>,
}

impl AppsMode {
    pub fn load() -> Self {
        let start = Instant::now();
        let apps = load_apps();
        info!("Loaded {} apps in {:?}", apps.len(), start.elapsed());
        Self { options: apps }
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

fn load_apps() -> Vec<Item> {
    util::find_files_from_env("XDG_DATA_DIRS", &|path| {
        Some(OsStr::new("desktop")) == path.extension()
    })
    .into_iter()
    .filter_map(|path| read_desktop_file(&path))
    .collect()
}

fn read_desktop_file(path: &Path) -> Option<Item> {
    let mut name = None;
    let mut exec = None;

    let file = File::open(path).ok()?;
    for line in BufReader::new(file).lines() {
        let line = line.ok()?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=').map(|(k, v)| (k.trim(), v.trim())) {
            match key {
                "Name" => name = Some(value.to_string()),
                "Exec" => exec = Some(value.to_string()),
                _ => {}
            }
        }
    }
    let name = name?;
    let exec = exec?; // NOTE: Exec is not required per the spec
    Some(Item::Exec { name, exec })
}
