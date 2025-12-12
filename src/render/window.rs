use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder},
};
use std::sync::{Arc};

use super::vulkantest::{vk_init, vk_render, vk_handle_resize};
use crate::shared::*;

pub fn create_window_app(shared: SharedData) {
    let event_loop = EventLoop::new();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Nova GE Window")
            .with_decorations(true)
            .build(&event_loop)
            .unwrap(),
    );

    vk_init(window.clone(), &event_loop);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

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

            // Drive the redraw loop from RedrawRequested instead of MainEventsCleared.
            Event::RedrawRequested(_) => {
                if let Ok(mut frame_count) = shared.frame_count.lock() {
                    *frame_count += 1;
                }

                vk_render(window.clone());
                window.request_redraw();
            }

            // Kick off the first frame once events are cleared.
            Event::MainEventsCleared => {
                window.request_redraw();
            }

            _ => (),
        }
    });
}