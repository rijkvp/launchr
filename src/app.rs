use crate::{render::Renderer, text::Text};
use std::rc::Rc;
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct App {
    pub text: Text,
}

impl App {
    pub fn new() -> Self {
        let text = Text::new();
        Self { text }
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
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }
                    WindowEvent::Resized(_) => {
                        window.request_redraw();
                    }
                    WindowEvent::RedrawRequested => renderer.draw(self),
                    WindowEvent::KeyboardInput { event, .. } => {
                        if event.state == ElementState::Pressed {
                            if let Some(input_text) = event.text {
                                self.text.add_text(&input_text);
                                window.request_redraw();
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            })
            .unwrap();
    }
}
