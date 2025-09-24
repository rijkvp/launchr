use criterion::{Criterion, criterion_group, criterion_main};
use launchr::{
    render::{DrawHandle, OnwedBuffer},
    ui::*,
};
use std::hint::black_box;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

fn create_ui() -> DynWidget {
    let mut texts = Vec::new();
    texts.push(
        container(
            TextBuilder::new(TEXT)
                .font(Some("Noto Sans"))
                .size(24.0)
                .build(),
        )
        .width(Length::Fill)
        .height(Length::Fill),
    );
    let mut root = container(column(texts))
        .width(Length::Fill)
        .height(Length::Fill)
        .bg(Color::from_rgba(50, 50, 50, 255))
        .padding_all(18)
        .into_dyn();

    root.layout(UVec2::new(WIDTH, HEIGHT));
    root
}

fn render_ui(root: &DynWidget, draw_handle: &mut DrawHandle) {
    root.render(UVec2::ZERO, draw_handle);
}

fn bench_text_render(c: &mut Criterion) {
    let root = create_ui();
    let mut draw_handle: DrawHandle = DrawHandle::from(OnwedBuffer::new(WIDTH, HEIGHT));
    render_ui(&root, &mut draw_handle);

    let bytes = draw_handle.get_bytes();
    image::ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        let i = (x + y * WIDTH) as usize * 4;
        let mut color_arr = [0u8; 4];
        color_arr.copy_from_slice(&bytes[i..i + 4]);
        image::Rgba(color_arr)
    })
    .save("text_render.png")
    .unwrap();

    c.bench_function("text_render", |b| {
        b.iter(|| {
            render_ui(&root, black_box(&mut draw_handle));
        });
    });
}

criterion_group! {
    name = text_render;
    config = Criterion::default().sample_size(10);
    targets = bench_text_render
}
criterion_main!(text_render);

const TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.

😀😀😀


ধারা ১ সমস্ত মানুষ স্বাধীনভাবে সমান মর্যাদা এবং অধিকার নিয়ে জন্মগ্রহণ করে। তাঁদের বিবেক এবং বুদ্ধি আছে; সুতরাং সকলেরই একে অপরের প্রতি ভ্রাতৃত্বসুলভ মনোভাব নিয়ে আচরণ করা উচিত।

I want more terminals to be able to handle ZWJ sequence emoji characters. For example, the service dog emoji 🐕‍🦺 is actually 3 Unicode characters. Kitty handles this fairly well. All VTE-based terminals, however, show '🐶🦺'.

כאשר העולם רוצה לדבר, הוא מדבר ב־Unicode. הירשמו כעת לכנס Unicode הבינלאומי העשירי, שייערך בין התאריכים 12־10 במרץ 1997, בְּמָיְינְץ שבגרמניה. בכנס ישתתפו מומחים מכל ענפי התעשייה בנושא האינטרנט העולמי וה־Unicode, בהתאמה לשוק הבינלאומי והמקומי, ביישום Unicode במערכות הפעלה וביישומים, בגופנים, בפריסת טקסט ובמחשוב רב־לשוני.

Many computer programs fail to display bidirectional text correctly. For example, this page is mostly LTR English script, and here is the RTL Hebrew name Sarah: שרה, spelled sin (ש) on the right, resh (ר) in the middle, and heh (ה) on the left.
This is some text.
";
