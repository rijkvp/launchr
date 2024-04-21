use criterion::{black_box, criterion_group, criterion_main, Criterion};
use launcher::{
    render::{Color, RenderBuffer},
    ui::*,
};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

fn render_ui(pixmap: &mut RenderBuffer) {
    let mut elements = Vec::new();
    for _ in 0..29 {
        elements.push(
            container(text(LOREM_IPSUM))
                .width(Length::Fill)
                .height(Length::Fixed(2 * 18))
                .into_element(),
        );
    }
    let mut root = container(column(elements))
        .width(Length::Fill)
        .height(Length::Fill)
        .bg(Color::from_rgba8(50, 50, 50, 255))
        .padding(18)
        .into_element();

    root.layout(UVec2::new(WIDTH as u64, HEIGHT as u64));
    root.render(UVec2::zero(), pixmap);
}

fn bench_text_render(c: &mut Criterion) {
    c.bench_function("text_render", |b| {
        b.iter(|| {
            let mut buffer = vec![0; WIDTH as usize * HEIGHT as usize * 4];
            let mut pixmap = RenderBuffer::from_bytes(&mut buffer, WIDTH, HEIGHT);
            render_ui(black_box(&mut pixmap));
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_text_render
}
criterion_main!(benches);

const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.";
