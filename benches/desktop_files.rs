use criterion::{criterion_group, criterion_main, Criterion};
use launcher::file_finder;
use std::ffi::OsStr;

fn bench_desktop_files(c: &mut Criterion) {
    c.bench_function("desktop_files", |b| {
        b.iter(|| {
            let _ = file_finder::find_files_from_env("XDG_DATA_DIRS", &|path| {
                Some(OsStr::new("desktop")) == path.extension()
            });
        });
    });
}

criterion_group! {
    name = desktop_files;
    config = Criterion::default().sample_size(20);
    targets = bench_desktop_files
}
criterion_main!(desktop_files);
