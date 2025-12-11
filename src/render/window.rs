use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use std::sync::{Arc, Mutex, OnceLock};

use super::vulkantest::{vk_init, vk_render, vk_handle_resize};
use crate::shared::*;

static WINDOW: OnceLock<Arc<Window>> = OnceLock::new();

pub struct App {
    pub shared: Option<Arc<Mutex<SharedData>>>,
}

pub fn create_window_app(shared: Arc<Mutex<SharedData>>) {
    let event_loop = EventLoop::new();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Nova GE Window")
            .with_decorations(true)
            .build(&event_loop)
            .unwrap(),
    );

    WINDOW.set(window.clone()).expect("WINDOW already initialized");

    vk_init(window.clone(), &event_loop);

    let mut app = App {
        shared: Some(shared.clone()),
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("The close button was pressed; stopping");
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(_) => {
                    vk_handle_resize();
                }
                _ => (),
            },

            Event::MainEventsCleared => {
                if let Some(shared_ref) = &app.shared {
                    let mut shared = shared_ref.lock().unwrap();
                    println!("framecount: {}", shared.frame_count);
                    shared.frame_count += 1;
                }

                vk_render(get_window());
                get_window().request_redraw();
            }

            _ => (),
        }
    });
}

// Safe getter used by rendering code
pub fn get_window() -> Arc<Window> {
    WINDOW.get().expect("Window not initialized").clone()
}