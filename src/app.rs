use crate::{
    config::Config,
    item::Item,
    mode::Mode,
    render::Renderer,
    ui::{
        column, container, text_box, DynamicList, Editor, Element, Length, ListContent, SizedBox,
        TextEditor, UVec2, Widget,
    },
};
use cosmic_text::Action;
use std::{sync::Arc, time::Instant};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowLevel},
};

pub struct App {
    starting: Option<Box<dyn Mode>>,
    running: Option<RunningApp>,
}

impl App {
    pub fn new(mode: Box<dyn Mode>) -> Self {
        App {
            starting: Some(mode),
            running: None,
        }
    }

    pub fn run(&mut self) {
        log::info!("starting application");
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);
        event_loop.run_app(self).unwrap();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.running.is_some() {
            return;
        }
        if let Some(mode) = self.starting.take() {
            let attributes = Window::default_attributes()
                .with_title("Launcher")
                .with_decorations(false)
                .with_transparent(true)
                .with_window_level(WindowLevel::AlwaysOnTop);
            let window = event_loop.create_window(attributes).unwrap();
            self.running = Some(RunningApp::new(window, mode));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let Some(running_app) = &mut self.running else {
            return;
        };
        // TODO: Fix ungly code here
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(e) => {
                log::debug!("resize window to {}x{}", e.width, e.height);
                running_app
                    .root
                    .layout(UVec2::new(e.width as u64, e.height as u64));
                running_app.window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                let time = Instant::now();

                running_app.renderer.draw(&running_app.root);
                log::info!("rendered in {:?}", time.elapsed());
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let is_dirty = running_app.input(event);
                if is_dirty {
                    running_app.update();
                    running_app.window.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let Some(ref running_app) = self.running else {
            return;
        };
        if running_app.exit {
            event_loop.exit();
        }
    }
}

pub struct RunningApp {
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

impl RunningApp {
    pub fn new(window: Window, mut mode: Box<dyn Mode>) -> RunningApp {
        let window = Arc::new(window);
        let config = Config::default();
        let matches = mode.matches(""); // initial matches
        let renderer = Renderer::from_window(window.clone());
        let editor = Editor::new();
        let list_content = ListContent::new();
        let root = build_ui(mode.name(), &config, editor.clone(), list_content.clone());
        RunningApp {
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

    fn input(&mut self, event: KeyEvent) -> bool {
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
