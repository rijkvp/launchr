use criterion::{Criterion, criterion_group, criterion_main};
use launchr::file_finder;
use std::ffi::OsStr;

fn bench_desktop_files(c: &mut Criterion) {
    c.bench_function("desktop_files", |b| {
        b.iter(|| {
            let _ = file_finder::find_files_from_dirs(
                &file_finder::get_dirs_from_env("XDG_DATA_DIRS"),
                &|path| Some(OsStr::new("desktop")) == path.extension(),
            );
        });
    });
}

criterion_group! {
    name = desktop_files;
    config = Criterion::default().sample_size(20);
    targets = bench_desktop_files
}
criterion_main!(desktop_files);
