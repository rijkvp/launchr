use crate::{editor::Editor, render::Renderer};
use cosmic_text::Action;
use std::rc::Rc;
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder, keyboard::{KeyCode, PhysicalKey},
};

pub struct App {
    pub editor: Editor,
}

impl App {
    pub fn new() -> Self {
        let editor = Editor::new();
        Self { editor }
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
                        if event.state == ElementState::Pressed {
                            match event.physical_key {
                                PhysicalKey::Code(KeyCode::Backspace) => {
                                    self.editor.perform_action(Action::Backspace);
                                }
                                PhysicalKey::Code(KeyCode::Enter) => {
                                    self.editor.perform_action(Action::Insert('\n'));
                                }
                                PhysicalKey::Code(KeyCode::Space) => {
                                    self.editor.perform_action(Action::Insert(' '));
                                }
                                PhysicalKey::Code(KeyCode::ArrowLeft) => {
                                    self.editor.perform_action(Action::Left);
                                }
                                PhysicalKey::Code(KeyCode::ArrowRight) => {
                                    self.editor.perform_action(Action::Right);
                                }
                                PhysicalKey::Code(KeyCode::ArrowUp) => {
                                    self.editor.perform_action(Action::Up);
                                }
                                PhysicalKey::Code(KeyCode::ArrowDown) => {
                                    self.editor.perform_action(Action::Down);
                                }
                                _ => (),
                            }
                            if let Some(char) = event.text.and_then(|t| t.chars().next()) {
                                self.editor.perform_action(Action::Insert(char));
                                window.request_redraw();
                            }
                        }
                        window.request_redraw();
                    }
                    _ => (),
                },
                _ => (),
            })
            .unwrap();
    }
}
