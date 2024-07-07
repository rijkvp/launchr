use crate::{
    config::Config,
    item::Item,
    mode::{AppsMode, DmenuMode, FileMode, Mode, RunMode},
    render::Renderer,
    ui::{
        column, container, text_box, DynamicList, Editor, Element, Length, ListContent, SizedBox,
        TextEditor, UVec2, Widget,
    }, winit_app::WinitApp,
};
use clap::Parser;
use cosmic_text::Action;
use std::{
    io::{self, Read},
    sync::Arc,
    time::Instant,
};
use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowLevel},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// dmenu mode
    #[arg(short, long)]
    dmenu: bool,
    /// Mode to use
    #[arg(short, long, default_value = "run")]
    mode: String,
}

pub struct App {
    mode: Box<dyn Mode>,
    window: Arc<Window>,
    renderer: Renderer,
    matches: Vec<Item>,
    selected: usize,
    config: Config,
    exit: bool,
    root: Element,
    list_content: ListContent,
    editor: Editor,
}

impl WinitApp for App {
    fn new(event_loop: &winit::event_loop::ActiveEventLoop) -> Self {
        let args: Args = Args::parse();
        let mut mode: Box<dyn Mode> = if args.dmenu {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("Failed to read from stdin");
            Box::new(DmenuMode::new(buffer))
        } else {
            match args.mode.as_str() {
                "apps" => Box::new(AppsMode::load()),
                "run" => Box::new(RunMode::load()),
                "file" => Box::new(FileMode::new(dirs::home_dir().unwrap())),
                other => {
                    eprintln!("Unknown mode: {}", other);
                    std::process::exit(1);
                }
            }
        };

        let attributes = Window::default_attributes()
            .with_title("Launcher")
            .with_decorations(false)
            .with_transparent(true)
            .with_window_level(WindowLevel::AlwaysOnTop);
        let window = Arc::new(event_loop.create_window(attributes).unwrap());

        let config = Config::default();
        let matches = mode.matches(""); // initial matches
        let renderer = Renderer::from_window(window.clone());
        let editor = Editor::new();
        let list_content = ListContent::new();
        let root = build_ui(mode.name(), &config, editor.clone(), list_content.clone());
        App {
            window,
            mode,
            renderer,
            matches,
            selected: 0,
            config,
            exit: false,
            root,
            list_content,
            editor,
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(e) => {
                log::debug!("resize window to {}x{}", e.width, e.height);
                self.root
                    .layout(UVec2::new(e.width as u64, e.height as u64));
                self.window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                let time = Instant::now();

                self.renderer.draw(&self.root);
                log::info!("rendered in {:?}", time.elapsed());
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let is_dirty = self.key_input(event);
                if is_dirty {
                    self.update();
                    self.window.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn exit(&self) -> bool {
        self.exit
    }
}

impl App {
    fn key_input(&mut self, event: KeyEvent) -> bool {
        let mut is_dirty = false;
        if event.state == ElementState::Pressed {
            if event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                self.exit = true;
            } else if event.physical_key == PhysicalKey::Code(KeyCode::Enter) {
                self.matches[self.selected].exec();
                is_dirty = true;
                self.exit = true;
            } else if event.physical_key == PhysicalKey::Code(KeyCode::ArrowDown) {
                if self.selected < self.matches.len() - 1 {
                    self.selected += 1;
                    log::info!("selected: {}", self.selected);
                    is_dirty = true;
                }
            } else if event.physical_key == PhysicalKey::Code(KeyCode::ArrowUp) {
                self.selected = self.selected.saturating_sub(1);
                log::info!("selected: {}", self.selected);
                is_dirty = true;
            } else {
                // Editor input
                if let PhysicalKey::Code(key) = event.physical_key {
                    is_dirty = self.editor.handle_key(key);
                }
                if let Some(char) = event.text.and_then(|t| t.chars().next()) {
                    self.editor.perform_action(Action::Insert(char));
                    is_dirty = true;
                }
            }
        }
        is_dirty
    }

    fn update(&mut self) {
        self.matches = self.mode.matches(&self.editor.text());
        self.list_content
            .update(self.matches.iter().enumerate().map(|(i, item)| {
                if i == self.selected {
                    container(text_box(&item.display(), 16.0))
                        .bg(self.config.colors.primary)
                        .into_element()
                } else {
                    container(text_box(&item.display(), 16.0))
                        .bg(self.config.colors.background_second)
                        .into_element()
                }
            }));
        let window_size = self.window.inner_size();
        self.root.layout(UVec2::new(
            window_size.width as u64,
            window_size.height as u64,
        ));
    }
}

fn build_ui(mode_name: &str, config: &Config, editor: Editor, content: ListContent) -> Element {
    let editor = TextEditor::new(editor, config.font_size);
    let editor_container = container(editor).height(Length::Fixed(config.font_size as u64));
    let root = container(column([
        text_box(mode_name, config.font_size),
        container(editor_container)
            .padding(4)
            .bg(config.colors.background_second)
            .into_element(),
        SizedBox::new()
            .color(config.colors.foreground)
            .width(Length::Fill)
            .height(Length::Fixed(2))
            .into_element(),
        DynamicList::new(content, 20).spacing(8).into_element(),
    ]))
    .padding(32)
    .bg(config.colors.background)
    .width(Length::Fill)
    .height(Length::Fill);
    root.into_element()
}
