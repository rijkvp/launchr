use crate::{
    config::Config,
    mode::{AppsMode, DmenuMode, FilesMode, Match, Mode, RunMode},
    recent::RecentItems,
    render::{CpuRenderer, Renderer},
    ui::{
        column, container, DynamicList, Editor, Element, Length, ListContent, SizedBox,
        TextBuilder, TextEditor, UVec2, Widget,
    },
    winit_app::WinitApp,
};
use clap::Parser;
use cosmic_text::Action;
use std::{
    io::{self, Read},
    sync::Arc,
    time::Instant,
};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    platform::wayland::WindowAttributesExtWayland,
    window::{Window, WindowLevel},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// dmenu mode
    #[arg(short, long)]
    dmenu: bool,
    /// Prompt to display in dmenu mode
    #[arg(short, long)]
    prompt: Option<String>,
    /// Mode to use
    #[arg(short, long, default_value = "run")]
    mode: String,
}

pub struct App {
    mode: Box<dyn Mode>,
    window: Arc<Window>,
    renderer: Box<dyn Renderer>,
    selected: usize,
    config: Config,
    exit: bool,
    root: Element,
    recent: RecentItems,
    list_content: ListContent,
    matches: Vec<Match>,
    editor: Editor,
}

impl WinitApp for App {
    fn start(event_loop: &winit::event_loop::ActiveEventLoop) -> Self {
        let args: Args = Args::parse();
        let mut mode: Box<dyn Mode> = if args.dmenu {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("Failed to read from stdin");
            Box::new(DmenuMode::new(args.prompt, buffer))
        } else {
            match args.mode.as_str() {
                "apps" => Box::new(AppsMode::load()),
                "run" => Box::new(RunMode::load()),
                "files" => Box::new(FilesMode::new(dirs::home_dir().unwrap())),
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
            .with_window_level(WindowLevel::AlwaysOnTop)
            .with_inner_size(PhysicalSize::new(1300, 700))
            .with_name("launcher", "launcher");

        let window = Arc::new(event_loop.create_window(attributes).unwrap());

        let config = Config::default();
        let renderer: Box<dyn Renderer> = Box::new(CpuRenderer::new(window.clone()));
        let editor = Editor::new();
        let list_content = ListContent::new();
        let root = build_ui(mode.name(), &config, editor.clone(), list_content.clone());
        let matches = mode.run(""); // initial
        let recent = RecentItems::load_or_default().unwrap();
        let mut app = App {
            window,
            mode,
            renderer,
            selected: 0,
            config,
            exit: false,
            root,
            recent,
            list_content,
            editor,
            matches,
        };
        app.update(); // initial update
        app
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
                self.root.layout(UVec2::new(e.width, e.height));
                self.window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                let time = Instant::now();
                self.renderer.render(&self.root);
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
                is_dirty = true;
                self.exit = true;
                if let Err(e) = self.recent.add_and_save(
                    &self.mode.name(),
                    self.matches[self.selected].item().clone(),
                ) {
                    log::error!("Failed to save recent items: {e}");
                }
                self.mode.exec(&self.matches[self.selected].item().clone());
            } else if event.physical_key == PhysicalKey::Code(KeyCode::ArrowDown) {
                self.selected =
                    (self.selected as i64 + 1).rem_euclid(self.matches.len() as i64) as usize;
                log::info!("selected: {}", self.selected);
                is_dirty = true;
            } else if event.physical_key == PhysicalKey::Code(KeyCode::ArrowUp) {
                self.selected =
                    (self.selected as i64 - 1).rem_euclid(self.matches.len() as i64) as usize;
                log::info!("selected: {}", self.selected);
                is_dirty = true;
            } else {
                // Editor input
                if let PhysicalKey::Code(key) = event.physical_key {
                    is_dirty = self.editor.handle_key(key);
                }
                if let Some(char) = event.text.and_then(|t| t.chars().next()) {
                    self.editor.perform_action(Action::Insert(char));
                    self.selected = 0;
                    is_dirty = true;
                }
            }
        }
        is_dirty
    }

    fn update(&mut self) {
        let input = self.editor.text();
        if input.is_empty() {
            self.matches = self.recent.get_matches(&self.mode.name());
        } else {
            self.matches = self.mode.run(&input);
        }
        self.list_content
            .update(self.matches.iter().enumerate().map(|(i, r#match)| {
                let item_text = format!("{match}");

                container(
                    TextBuilder::new(&item_text)
                        .size(self.config.font_size.normal)
                        .build(),
                )
                .bg(if i == self.selected {
                    self.config.colors.primary
                } else {
                    self.config.colors.background
                })
                .width(Length::Fill)
                .padding(4)
                .into_element()
            }));
        let window_size = self.window.inner_size();
        self.root
            .layout(UVec2::new(window_size.width, window_size.height));
    }
}

fn build_ui(mode_name: &str, config: &Config, editor: Editor, content: ListContent) -> Element {
    let editor = TextEditor::new(editor, config.font_size.normal);
    let editor_container = container(editor).height(Length::Fixed(config.font_size.normal as u32));
    let root = container(column([
        TextBuilder::new(mode_name)
            .size(config.font_size.large)
            .bold(true)
            .build()
            .into_element(),
        container(editor_container)
            .padding(4)
            .bg(config.colors.background_second)
            .into_element(),
        SizedBox::new()
            .color(config.colors.foreground)
            .width(Length::Fill)
            .height(Length::Fixed(2))
            .into_element(),
        // note that the item_height must be large enough to fit the text
        DynamicList::new(content, 28).spacing(4).into_element(),
    ]))
    .padding(32)
    .bg(config.colors.background)
    .width(Length::Fill)
    .height(Length::Fill);
    root.into_element()
}
