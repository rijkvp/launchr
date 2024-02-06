use crate::{editor::Editor, mode::FileMode, mode::Mode, render::Renderer, text::Text};
use cosmic_text::Action;
use std::rc::Rc;
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::WindowBuilder,
};

pub struct App {
    pub editor: Editor,
    pub file_mode: FileMode,
    pub text: Text,
}

impl App {
    pub fn new() -> Self {
        Self {
            editor: Editor::new(),
            file_mode: FileMode {},
            text: Text::new(),
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);
        let window = Rc::new(
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
                    WindowEvent::RedrawRequested => renderer.draw(self),
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
                            let matches: Vec<String> = self
                                .file_mode
                                .run(self.editor.text())
                                .into_iter()
                                .take(10)
                                .collect();
                            println!("matches: {:?}", matches);
                            self.text.set_text(matches.join("\n").as_str());
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
