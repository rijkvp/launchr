use criterion::{criterion_group, criterion_main, Criterion};
use launchr::file_finder;

fn bench_files(c: &mut Criterion) {
    c.bench_function("files", |b| {
        b.iter(|| {
            let _ = file_finder::find_all_files(&dirs::home_dir().unwrap());
        });
    });
}

criterion_group! {
    name = files;
    config = Criterion::default().sample_size(20);
    targets = bench_files
}
criterion_main!(files);
