use crate::{editor::Editor, mode::FileMode, mode::Mode, render::Renderer, text::Text};
use cosmic_text::Action;
use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Instant,
};
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::WindowBuilder,
};

pub struct App {
    pub editor: Editor,
    pub text: Text,
    pub matches: Arc<Mutex<Vec<String>>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            editor: Editor::new(),
            text: Text::new(),
            matches: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn run(&mut self) {
        let (input_tx, result_rx) = start_executor(FileMode);
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
        {
            let window = window.clone();
            let matches = self.matches.clone();
            thread::spawn(move || loop {
                if let Ok(new_matches) = result_rx.recv() {
                    // TODO: Cancel previous search if new search is started
                    {
                        let mut matches = matches.lock().unwrap();
                        *matches = new_matches;
                    }
                    window.request_redraw();
                }
            });
        }
        event_loop
            .run(move |event, elwt| match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::Resized(_) => {
                        window.request_redraw();
                    }
                    WindowEvent::RedrawRequested => {
                        {
                            let matches = self.matches.lock().unwrap();
                            self.text.set_text(&matches.join("\n"));
                        }
                        renderer.draw(self)
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
                            input_tx.send(self.editor.text().to_string()).unwrap();
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
pub fn start_executor(mut file_mode: FileMode) -> (Sender<String>, Receiver<Vec<String>>) {
    let (input_tx, input_rx) = mpsc::channel::<String>();
    let (result_tx, result_rx) = mpsc::channel::<Vec<String>>();
    thread::spawn(move || loop {
        let input = input_rx.recv().unwrap();
        let time = Instant::now();
        let matches: Vec<String> = file_mode.run(&input);
        println!("{} matches in {:?}", matches.len(), time.elapsed());
        result_tx.send(matches).unwrap();
    });
    (input_tx, result_rx)
}
