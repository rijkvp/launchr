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
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy},
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

pub enum UserEvent {
    Update,
}

#[derive(Debug, Clone)]
pub struct EventHandle {
    proxy: EventLoopProxy<UserEvent>,
}

impl EventHandle {
    pub fn send_update(&self) {
        if let Err(e) = self.proxy.send_event(UserEvent::Update) {
            log::error!("failed to send update event: {}", e);
        }
    }
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
        let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);
        let proxy = event_loop.create_proxy();
        self.launcher.run(EventHandle { proxy });
        event_loop.run_app(&mut self).unwrap();
    }
}

impl ApplicationHandler<UserEvent> for WinitApp {
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

    fn user_event(&mut self, _: &ActiveEventLoop, event: UserEvent) {
        if let AppState::Running { window, .. } = &mut self.window {
            self.launcher.user_event(event);
            window.request_redraw();
        }
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
                    renderer.render(self.launcher.root());
                    log::info!("rendered in {:?}", time.elapsed());
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if self.launcher.key_input(event) {
                        self.launcher.update();
                        window.request_redraw();
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
