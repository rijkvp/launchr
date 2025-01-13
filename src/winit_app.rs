use crate::{
    launcher::Launcher,
    render::{CpuRenderer, Renderer},
    ui::UVec2,
};
use std::{sync::Arc, time::Instant};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    platform::wayland::WindowAttributesExtWayland,
    window::{Window, WindowId, WindowLevel},
};

pub struct WinitApp {
    launcher: Launcher,
    window: AppState,
}

enum AppState {
    Starting,
    Running {
        window: Arc<Window>,
        renderer: Box<dyn Renderer>,
    },
}

impl WinitApp {
    pub fn new(launcher: Launcher) -> Self {
        Self {
            launcher,
            window: AppState::Starting,
        }
    }

    pub fn run(mut self) {
        log::info!("starting winit application");
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);
        event_loop.run_app(&mut self).unwrap();
    }
}

impl ApplicationHandler for WinitApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes()
            .with_title("Launcher")
            .with_decorations(false)
            .with_transparent(true)
            .with_window_level(WindowLevel::AlwaysOnTop)
            .with_inner_size(PhysicalSize::new(1300, 700))
            .with_name("launcher", "launcher");
        let window = Arc::new(event_loop.create_window(attributes).unwrap());
        self.window = AppState::Running {
            window: window.clone(),
            renderer: Box::new(CpuRenderer::new(window)),
        };
        self.launcher.update(); // initial update
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let AppState::Running { window, renderer } = &mut self.window {
            match event {
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::Resized(e) => {
                    log::debug!("resize window to {}x{}", e.width, e.height);
                    self.launcher.resize(UVec2::new(e.width, e.height));
                    window.request_redraw();
                }
                WindowEvent::RedrawRequested => {
                    let time = Instant::now();
                    renderer.render(&self.launcher.root());
                    log::info!("rendered in {:?}", time.elapsed());
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if self.launcher.key_input(event) {
                        self.launcher.update();
                        window.request_redraw();
                        // Required because ListContent changes layout internally
                        // TODO: move inside launcher.update()?
                        // TODO: Optimize ListContent to only relayout if necessary and handle its
                        // state internally
                        let window_size = window.inner_size();
                        self.launcher
                            .resize(UVec2::new(window_size.width, window_size.height));
                    }
                }
                _ => {}
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.launcher.close_requested() {
            event_loop.exit();
        }
    }
}
