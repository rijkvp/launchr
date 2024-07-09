use criterion::{criterion_group, criterion_main, Criterion};
use launcher::util::find_files_from_env;
use std::ffi::OsStr;

fn bench_desktop_files(c: &mut Criterion) {
    c.bench_function("desktop_files", |b| {
        b.iter(|| {
            let _ = find_files_from_env("XDG_DATA_DIRS", &|path| {
                Some(OsStr::new("desktop")) == path.extension()
            });
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(20);
    targets = bench_desktop_files
}
criterion_main!(benches);
