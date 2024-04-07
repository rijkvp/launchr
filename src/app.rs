use crate::{
    component::{container::Container, text::TextEditor, Component},
    config::Config,
    item::Item,
    mode::Mode,
    render::Renderer,
};
use cosmic_text::Action;
use std::{cell::RefCell, rc::Rc, sync::Arc, time::Instant};
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

fn build_ui(config: &Config) -> (Container, Rc<RefCell<TextEditor>>) {
    let editor = Rc::new(RefCell::new(TextEditor::new(config.font_size)));
    let editor_container = Container::from(editor.clone())
        .with_padding(4)
        .with_background(config.colors.background_second);
    let root = Container::new(editor_container)
        .with_padding(16)
        .with_background(config.colors.background);
    (root, editor)
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

        let (mut root, editor) = build_ui(&self.config);
        event_loop
            .run(move |event, elwt| match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::Resized(e) => {
                        root.layout(e.width as u64, e.height as u64);
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
                                    is_dirty = editor.borrow_mut().handle_key(key);
                                }
                                if let Some(char) = event.text.and_then(|t| t.chars().next()) {
                                    editor.borrow_mut().perform_action(Action::Insert(char));
                                    is_dirty = true;
                                }
                            }
                        }
                        if is_dirty {
                            self.matches = self.mode.matches(&editor.borrow_mut().text());
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
