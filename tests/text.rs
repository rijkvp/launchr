use launcher::{
    render::{DrawHandle, OnwedBuffer},
    ui::*,
};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

fn create_ui() -> Element {
    let mut texts = Vec::new();
    let items = &[
        ("This is a test text", None),
        ("🙃🙃🙃🇳🇱🌶️🤯", Some("Noto Emoji")),
        (
            "Ligatures: -> => =>> ->> ->=> ->=>> ->-> ->--> ->-->>",
            Some("Fira Code"),
        ),
    ];
    for (text, font) in items {
        let mut text_builder = TextBuilder::new(*text).size(64.0);
        if let Some(font) = font {
            text_builder = text_builder.font(*font);
        }
        texts.push(
            container(text_builder.build())
                .bg(Color::from_rgb(20, 20, 20))
                .into_element(),
        )
    }
    let mut root = container(column(texts))
        .width(Length::Fill)
        .height(Length::Fill)
        .bg(Color::from_rgb(50, 50, 50))
        .padding(24)
        .into_element();

    root.layout(UVec2::new(WIDTH, HEIGHT));
    root
}

fn render_ui(root: &Element, draw_handle: &mut DrawHandle) {
    root.render(UVec2::zero(), draw_handle);
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
