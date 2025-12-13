pub mod vulkantest;

use crate::{render::vulkantest::vk_reload_shaders, *};
use crate::shared::*;

use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use std::{sync::Arc, time::Instant};

use vulkantest::{vk_init, vk_render, vk_handle_resize};


fn main_loop(shared: &SharedData, win: &Arc<Window>){
    vk_render(&win);
}

pub fn main(shared: SharedData){
    //wait for everything to initialize
    drop(shared.logic_init.lock());
    drop(shared.io_init.lock());
    drop(shared.audio_init.lock());

    //initialize render thread
    let event_loop = EventLoop::new();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Nova GE Window")
            .with_decorations(true)
            .build(&event_loop)
            .unwrap(),
    );

    vk_init(&shared,&window, &event_loop);

    info!(shared, "Render thread initialized");

    let mut last_frame_time = Instant::now();
    let mut frame_count = 0;
    let mut fps_timer = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    popup_info!(shared,"being annoying on purpuse","I see you wanted to close this app so we'll give you this annoying ass popup");
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(_) => {
                    vk_handle_resize();
                }
                WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {
                    if (input.virtual_keycode == Some(VirtualKeyCode::S)) {
                        let mut vs_data = Vec::<u32>::new();
                        let mut fs_data = Vec::<u32>::new();

                        let vs_ass = get_asset(&shared, "test_vs2");
                        let fs_ass = get_asset(&shared, "test_fs2");

                        if let Asset::Shader(mut data) = vs_ass {
                            vs_data.append(&mut data);
                        } else {
                            err!(shared,"Failed to get the test vertex shader");
                        }

                        if let Asset::Shader(mut data) = fs_ass {
                            fs_data.append(&mut data);
                        } else {
                            err!(shared,"Failed to get the test fragment shader");
                        }

                        vk_reload_shaders(vs_data.as_slice(), fs_data.as_slice());
                    }
                }
                _ => (),
            },

            // Drive the redraw loop from RedrawRequested instead of MainEventsCleared.
            Event::RedrawRequested(_) => {
                // measure stuff
                let now = Instant::now();
                let delta = now.duration_since(last_frame_time);
                last_frame_time = now;
                frame_count += 1;

                if let Ok(mut stats) = shared.render_stats.lock() {
                    stats.frametime = delta.as_micros();
                    // only update every second
                    if fps_timer.elapsed() >= Duration::from_secs(1) {
                        stats.framerate = frame_count as f32 / fps_timer.elapsed().as_secs_f32();
                        println!("FPS: {:.2}, Frametime: {} microseconds", stats.framerate, stats.frametime);
                        frame_count = 0;
                        fps_timer = Instant::now();
                    }
                }

                main_loop(&shared, &window);
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