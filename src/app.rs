use crate::{
    mode::Mode,
    mode::RunMode,
    render::Renderer,
    text::{Editor, Rect, Text},
};
use cosmic_text::Action;
use crossbeam_channel::Sender;
use std::{
    sync::{Arc, Mutex},
    thread::{self},
    time::Instant,
};
use tracing::info;
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::{Window, WindowBuilder},
};

pub struct App {
    pub editor: Editor,
    pub text: Text,
    pub matches: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        info!("Creating app");
        Self {
            editor: Editor::new(Rect::new(8, 8, 800, 56), 20.0),
            text: Text::new(Rect::new(8, 64, 800, 600), 20.0),
            matches: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        info!("Starting app");
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);
        let window = Arc::new(
            WindowBuilder::new()
                .with_title("Launcher")
                .build(&event_loop)
                .unwrap(),
        );
        let mut renderer = Renderer::from_window(window.clone());
        event_loop.set_control_flow(ControlFlow::Wait);
        event_loop
            .run(move |event, elwt| match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::Resized(_) => {
                        window.request_redraw();
                    }
                    WindowEvent::RedrawRequested => {
                        info!("Start render");
                        let time = Instant::now();
                        {
                            self.text.set_text(&self.matches.join("\n"));
                        }
                        renderer.draw(self);
                        info!("Rendered in {:?}", time.elapsed());
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        let mut is_dirty = false;
                        if event.state == ElementState::Pressed {
                            if let PhysicalKey::Code(key) = event.physical_key {
                                is_dirty = self.editor.handle_key(key);
                            }
                            if let Some(char) = event.text.and_then(|t| t.chars().next()) {
                                self.editor.perform_action(Action::Insert(char));
                                is_dirty = true;
                            }
                        }
                        if is_dirty {
                            self.matches = RunMode.run(&self.editor.text());
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
pub fn _start_search_thread(
    window: Arc<Window>,
    matches: Arc<Mutex<Vec<String>>>,
) -> Sender<String> {
    let (input_tx, input_rx) = crossbeam_channel::unbounded::<String>();
    let mut file_mode = RunMode;
    thread::spawn(move || {
        let mut next_input = None;
        loop {
            let input = if let Some(input) = next_input.take() {
                input
            } else {
                input_rx.recv().unwrap()
            };
            info!("Starting search for {:?}", input);
            let time = Instant::now();
            let new_matches: Vec<String> = file_mode.run(&input);
            if let Ok(new) = input_rx.try_recv() {
                // Iput has changed in the meantime
                info!("Search cancelled, replaced by {:?}", new);
                return;
            }
            info!("{} new matches in {:?}", new_matches.len(), time.elapsed());
            {
                let mut matches = matches.lock().unwrap();
                *matches = new_matches;
            }
            window.request_redraw();
        }
    });
    input_tx
}
