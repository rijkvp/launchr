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
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{WindowBuilder, WindowLevel},
};

pub struct App {
    mode: Box<dyn Mode>,
    matches: Vec<Item>,
    selected: usize,
    config: Config,
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

impl App {
    pub fn new(mode: Box<dyn Mode>) -> Self {
        let config = Config::default();
        Self {
            mode,
            matches: Vec::new(),
            selected: 0,
            config,
        }
    }

    pub fn run(&mut self) {
        log::info!("starting application");
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);
        let window = Arc::new(
            WindowBuilder::new()
                .with_title("Launcher")
                .with_decorations(false)
                .with_transparent(true)
                .with_window_level(WindowLevel::AlwaysOnTop)
                .build(&event_loop)
                .unwrap(),
        );
        let mut renderer = Renderer::from_window(window.clone());
        event_loop.set_control_flow(ControlFlow::Wait);
        self.matches = self.mode.matches(""); // initial matches

        let mut editor = Editor::new();
        let mut list_content = ListContent::new();
        let mut root = build_ui(
            self.mode.name(),
            &self.config,
            editor.clone(),
            list_content.clone(),
        );
        event_loop
            .run(move |event, elwt| match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::Resized(e) => {
                        log::debug!("resize window to {}x{}", e.width, e.height);
                        root.layout(UVec2::new(e.width as u64, e.height as u64));
                        window.request_redraw();
                    }
                    WindowEvent::RedrawRequested => {
                        let time = Instant::now();

                        renderer.draw(&root);
                        log::info!("rendered in {:?}", time.elapsed());
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        let mut is_dirty = false;
                        if event.state == ElementState::Pressed {
                            if event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                                elwt.exit();
                            } else if event.physical_key == PhysicalKey::Code(KeyCode::Enter) {
                                self.matches[self.selected].exec();
                                is_dirty = true;
                                elwt.exit();
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
                                    is_dirty = editor.handle_key(key);
                                }
                                if let Some(char) = event.text.and_then(|t| t.chars().next()) {
                                    editor.perform_action(Action::Insert(char));
                                    is_dirty = true;
                                }
                            }
                        }
                        if is_dirty {
                            self.matches = self.mode.matches(&editor.text());
                            list_content.update(self.matches.iter().enumerate().map(
                                |(i, item)| {
                                    if i == self.selected {
                                        container(text_box(&item.display(), 16.0))
                                            .bg(self.config.colors.primary)
                                            .into_element()
                                    } else {
                                        container(text_box(&item.display(), 16.0))
                                            .bg(self.config.colors.background_second)
                                            .into_element()
                                    }
                                },
                            ));
                            let window_size = window.inner_size();
                            root.layout(UVec2::new(
                                window_size.width as u64,
                                window_size.height as u64,
                            ));
                            window.request_redraw();
                        }
                    }
                    _ => (),
                },
                _ => (),
            })
            .unwrap();
    }
}
