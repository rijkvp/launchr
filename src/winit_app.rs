use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
};

pub enum WinitAppLauncher<App: WinitApp> {
    Starting,
    Running(App),
}

pub trait WinitApp {
    fn new(event_loop: &ActiveEventLoop) -> Self;
    fn window_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent);
    fn exit(&self) -> bool;
}

pub fn launch_winit_app<App: WinitApp>() {
    log::info!("starting application");
    let mut app: WinitAppLauncher<App> = WinitAppLauncher::Starting;
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut app).unwrap();
}

impl<App: WinitApp> ApplicationHandler for WinitAppLauncher<App> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        *self = Self::Running(App::new(event_loop));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let Self::Running(app) = self {
            app.window_event(event_loop, event);
        }
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Self::Running(app) = self {
            if app.exit() {
                event_loop.exit();
            }
        }
    }
}
