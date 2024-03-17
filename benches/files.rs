use launcher::util::find_files_from_env;
use std::ffi::OsStr;

fn main() {
    divan::main();
}

#[divan::bench]
fn find_desktop_files() {
    let _ = find_files_from_env("XDG_DATA_DIRS", &|path| {
        Some(OsStr::new("desktop")) == path.extension()
    });
}
