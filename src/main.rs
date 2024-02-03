use cosmic_text::{Attrs, Family, FontSystem, Metrics, Shaping, SwashCache};
use softbuffer::{Context, Surface};
use std::{num::NonZeroU32, rc::Rc};
use tiny_skia::{Color, Paint, PixmapMut, Rect, Transform};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

// Setup based on: https://github.com/pop-os/cosmic-text/blob/main/examples/multiview/src/main.rs

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    let window = Rc::new(
        WindowBuilder::new()
            .with_title("Launcher")
            .build(&event_loop)
            .unwrap(),
    );
    let context = Context::new(window.clone()).unwrap();
    let mut surface = Surface::new(&context, window.clone()).unwrap();

    let mut font_system = FontSystem::new();
    let mut text_buffer = cosmic_text::Buffer::new_empty(Metrics::new(64.0, 74.0));
    let mut buffer = text_buffer.borrow_with(&mut font_system);
    let mut swash_cache = SwashCache::new();
    let attrs = Attrs::new().family(Family::Monospace);
    buffer.set_text("Hello world!", attrs, Shaping::Advanced);

    event_loop
        .run(move |event, elwt| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    elwt.exit();
                }
                Event::AboutToWait => {
                    // TODO: Only if state has changed
                    window.request_redraw();
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    // Redraw the application.
                    let (width, height) = {
                        let size = window.inner_size();
                        (size.width, size.height)
                    };
                    surface
                        .resize(
                            NonZeroU32::new(width).unwrap(),
                            NonZeroU32::new(height).unwrap(),
                        )
                        .unwrap();

                    let mut surface_buffer = surface.buffer_mut().unwrap();
                    let surface_buffer_u8 = unsafe {
                        std::slice::from_raw_parts_mut(
                            surface_buffer.as_mut_ptr() as *mut u8,
                            surface_buffer.len() * 4,
                        )
                    };
                    let mut pixmap =
                        PixmapMut::from_bytes(surface_buffer_u8, width, height).unwrap();
                    pixmap.fill(Color::from_rgba8(0, 0, 0, 0xFF));

                    buffer.set_size(width as f32, height as f32);

                    let mut paint = Paint::default();
                    let transform = Transform::identity();
                    buffer.draw(
                        &mut swash_cache,
                        cosmic_text::Color::rgb(0xFF, 0xFF, 0xFF),
                        |x, y, w, h, color| {
                            paint.set_color_rgba8(color.r(), color.g(), color.b(), color.a());
                            pixmap.fill_rect(
                                Rect::from_xywh(x as f32, y as f32, w as f32, h as f32).unwrap(),
                                &paint,
                                transform,
                                None,
                            );
                        },
                    );

                    // Present the surface
                    surface_buffer.present().unwrap();
                }
                _ => (),
            }
        })
        .unwrap();
}
