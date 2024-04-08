use crate::{
    config::Config,
    item::Item,
    mode::Mode,
    render::Renderer,
    ui::{column, container, Editor, Element, Length, SizedBox, TextEditor, UVec2, Widget},
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

fn build_ui(config: &Config, editor: Editor) -> Element {
    let editor = TextEditor::new(editor, config.font_size);
    let editor_container = container(editor)
        .padding(4)
        .height(Length::Fixed(200))
        .bg(config.colors.background_second);
    let root = container(column(vec![
        editor_container.into_element(),
        SizedBox::new()
            .color(config.colors.primary)
            .width(Length::Fixed(50))
            .height(Length::Fixed(100))
            .into_element(),
        SizedBox::new()
            .color(config.colors.secondary)
            .width(Length::Fixed(100))
            .height(Length::Fixed(100))
            .into_element(),
    ]))
    .padding(32)
    .bg(config.colors.background)
    .width(Length::Fill)
    .height(Length::Fill);
    root.into_element()
}

impl App {
    pub fn new(mode: Box<dyn Mode>) -> Self {
        log::info!("Creating app");
        let config = Config::default();
        Self {
            mode,
            matches: Vec::new(),
            selected: 0,
            config,
        }
    }

    // fn render_matches(&self, outer: Rect) -> Vec<Component> {
    //     let mut components = Vec::new();
    //     // TODO: Don't render too much
    //     for (i, item) in self.matches.iter().enumerate().take(20) {
    //         let bg_color = if i == self.selected {
    //             self.config.colors.primary
    //         } else {
    //             self.config.colors.background_second
    //         };
    //         let y = outer.y + i as u64 * FONT_SIZE;
    //         if y > outer.height {
    //             break;
    //         }
    //         components.push(Component::Container(Container::new(
    //             Rect::new(outer.x, y, outer.width, FONT_SIZE),
    //             bg_color,
    //         )));
    //         components.push(Component::Text(
    //             Text::new(
    //                 Rect::new(outer.x, y, outer.width, FONT_SIZE),
    //                 FONT_SIZE as f32,
    //             )
    //             .with_text(&item.display()),
    //         ));
    //     }
    //     components
    // }

    pub fn run(&mut self) {
        log::info!("Starting app");
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
        let mut root = build_ui(&self.config, editor.clone());
        event_loop
            .run(move |event, elwt| match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::Resized(e) => {
                        log::info!("resize window to {}x{}", e.width, e.height);
                        root.layout(UVec2::new(e.width as u64, e.height as u64));
                        window.request_redraw();
                    }
                    WindowEvent::RedrawRequested => {
                        let time = Instant::now();

                        renderer.draw(&root);
                        log::info!("Rendered in {:?}", time.elapsed());
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
                                    is_dirty = true;
                                }
                            } else if event.physical_key == PhysicalKey::Code(KeyCode::ArrowUp) {
                                self.selected = self.selected.saturating_sub(1);
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
