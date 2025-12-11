use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use std::sync::OnceLock;
use crate::shared::*;
use std::sync::{Arc, Mutex};

static WINDOW: OnceLock<&'static Window> = OnceLock::new();

#[derive(Default)]
pub struct App<'a> {
    window: Option<Window>,
    pub shared: Option<&'a Arc<Mutex<shared_data>>>
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Window::default_attributes()).unwrap();

        let static_win: &'static Window = Box::leak(Box::new(window));
        WINDOW.set(static_win).expect("Failed to set WINDOW");
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.

                let mut shared_handle = &mut (self.shared.as_mut().unwrap().lock().unwrap());
                println!("framecount: {}", shared_handle.frame_count);
                shared_handle.frame_count += 1;
                get_window().request_redraw();
            }
            _ => (),
        }
    }
}

pub fn create_window_app<'a>(shared: &'a Arc<Mutex<shared_data>>) {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll); // according to the docs this is the best for games

    let mut app = App::default();

    app.shared = Option::Some(shared);

    event_loop.run_app(&mut app).unwrap();
}

pub fn get_window() -> &'static Window {
    WINDOW.get().expect("Window not initialized")
}