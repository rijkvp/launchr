use crate::{
    component::{
        container::Container,
        text::{Text, TextEditor},
        Component,
    },
    mode::{Item, Mode},
    render::{Color, Rect, Renderer},
};
use cosmic_text::Action;
use std::{iter::once, sync::Arc, time::Instant};
use tracing::info;
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{WindowBuilder, WindowLevel},
};

const WIDTH: u64 = 800;
const HEIGHT: u64 = 400;
const MARGIN: u64 = 8;
const FONT_SIZE: u64 = 22;

pub struct App {
    pub mode: Box<dyn Mode>,
    pub matches: Vec<Item>,
    pub selected: usize,
}

impl App {
    pub fn new(mode: Box<dyn Mode>) -> Self {
        info!("Creating app");
        Self {
            mode,
            matches: Vec::new(),
            selected: 0,
        }
    }

    fn render_matches(&self, outer: Rect) -> Vec<Component> {
        let mut components = Vec::new();
        for (i, item) in self.matches.iter().enumerate() {
            let color = if i == self.selected {
                Color::from_rgba8(200, 200, 200, 255)
            } else {
                Color::from_rgba8(0, 0, 255, 255)
            };
            let y = outer.y + i as u64 * FONT_SIZE;
            if y > outer.height {
                break;
            }
            components.push(Component::Container(Container::new(
                Rect::new(outer.x, y, outer.width, FONT_SIZE),
                color,
            )));
            components.push(Component::Text(
                Text::new(
                    Rect::new(outer.x, y, outer.width, FONT_SIZE),
                    FONT_SIZE as f32,
                )
                .with_text(&item.display()),
            ));
        }
        components
    }

    pub fn run(&mut self) {
        info!("Starting app");
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);
        let window = Arc::new(
            WindowBuilder::new()
                .with_title("Launcher")
                .with_decorations(false)
                .with_resizable(false)
                .with_transparent(true)
                .with_window_level(WindowLevel::AlwaysOnTop)
                .build(&event_loop)
                .unwrap(),
        );
        let mut renderer = Renderer::from_window(window.clone());
        event_loop.set_control_flow(ControlFlow::Wait);
        self.matches = self.mode.matches("");

        let mut editor = TextEditor::new(
            Rect::new(MARGIN, MARGIN, WIDTH, FONT_SIZE),
            FONT_SIZE as f32,
        );
        event_loop
            .run(move |event, elwt| match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::Resized(_) => {
                        window.request_redraw();
                    }
                    WindowEvent::RedrawRequested => {
                        let time = Instant::now();

                        let background = Component::Container(Container::new(
                            Rect::new(0, 0, WIDTH, HEIGHT),
                            Color::from_rgba8(255, 0, 0, 255),
                        ));
                        let editor_bg = Component::Container(Container::new(
                            Rect::new(MARGIN, MARGIN, WIDTH - MARGIN * 2, FONT_SIZE),
                            Color::from_rgba8(0, 255, 0, 255),
                        ));
                        let editor = Component::Editor(&mut editor);
                        renderer.draw(
                            once(background)
                                .chain(once(editor_bg))
                                .chain(once(editor))
                                .chain(self.render_matches(Rect::new(
                                    MARGIN,
                                    MARGIN + FONT_SIZE,
                                    WIDTH - MARGIN * 2,
                                    800,
                                ))),
                        );
                        info!("Rendered in {:?}", time.elapsed());
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        let mut is_dirty = false;
                        if event.state == ElementState::Pressed {
                            if event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                                elwt.exit();
                            } else if event.physical_key == PhysicalKey::Code(KeyCode::Enter) {
                                self.matches[self.selected].exec();
                                is_dirty = true;
                            } else if event.physical_key == PhysicalKey::Code(KeyCode::ArrowDown) {
                                if self.selected < self.matches.len() - 1 {
                                    self.selected += 1;
                                    is_dirty = true;
                                }
                            } else if event.physical_key == PhysicalKey::Code(KeyCode::ArrowUp) {
                                self.selected = self.selected.saturating_sub(1);
                                is_dirty = true;
                            } else {
                                // Edtior input
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

// TODO: Multithreading
// pub fn _start_search_thread(
//     window: Arc<Window>,
//     matches: Arc<Mutex<Vec<String>>>,
// ) -> Sender<String> {
//     let (input_tx, input_rx) = crossbeam_channel::unbounded::<String>();
//     let mut mode = RunMode;
//     thread::spawn(move || {
//         let mut next_input = None;
//         loop {
//             let input = if let Some(input) = next_input.take() {
//                 input
//             } else {
//                 input_rx.recv().unwrap()
//             };
//             info!("Starting search for {:?}", input);
//             let time = Instant::now();
//             let new_matches: Vec<String> = file_mode.matches(&input);
//             if let Ok(new) = input_rx.try_recv() {
//                 // Iput has changed in the meantime
//                 info!("Search cancelled, replaced by {:?}", new);
//                 return;
//             }
//             info!("{} new matches in {:?}", new_matches.len(), time.elapsed());
//             {
//                 let mut matches = matches.lock().unwrap();
//                 *matches = new_matches;
//             }
//             window.request_redraw();
//         }
//     });
//     input_tx
// }
