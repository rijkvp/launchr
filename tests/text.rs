use launchr::{
    render::{DrawHandle, OnwedBuffer},
    ui::*,
};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

fn create_ui() -> DynWidget {
    let mut texts = Vec::new();
    let items = &[
        ("This is a test text", None, None),
        ("Emojis 🙃🙃🙃🇳🇱🌶️🤯", Some("Noto Emoji"), None),
        (
            "Ligatures: -> => =>> ->> ->=> ->=>> ->-> ->--> ->-->>",
            Some("FiraCode Nerd Font"),
            None,
        ),
        ("Emojis 🙃🙃🙃🇳🇱🌶️🤯", Some("Noto Emoji"), None),
        (
            "The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog.",
            Some("DejaVu Serif"),
            None,
        ),
        (
            "The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog.",
            None,
            Some(84.0),
        ),
    ];
    for (n, (text, font, line_height)) in items.iter().enumerate() {
        let mut text_builder = TextBuilder::new(*text).size(56.0).font(*font);
        if let Some(line_height) = line_height {
            text_builder = text_builder.line_height(*line_height);
        }
        let i = (20 + n * 10) as u8;
        texts.push(
            container(text_builder.build())
                .bg(Color::from_rgb(i, i, i))
                .into_dyn(),
        )
    }
    let mut root = container(column(texts))
        .width(Length::Fill)
        .height(Length::Fill)
        .bg(Color::from_rgb(200, 200, 200))
        .padding_all(24)
        .into_dyn();

    root.layout(UVec2::new(WIDTH, HEIGHT));
    root
}

fn render_ui(root: &DynWidget, draw_handle: &mut DrawHandle) {
    root.render(UVec2::ZERO, draw_handle);
}

#[test]
fn test_text_render() {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .init();
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
}
