pub mod vulkantest;

use crate::{render::vulkantest::vk_reload_shaders, *};
use crate::shared::*;

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes, WindowId},
};

use vulkantest::{vk_handle_resize, vk_init, vk_render, vk_shutdown};

struct App {
    shared: SharedData,
    window: Option<Arc<Window>>,
    last_frame_time: Instant,
    frame_count: u32,
    fps_timer: Instant,
}

impl App {
    fn new(shared: SharedData) -> Self {
        Self {
            shared,
            window: None,
            last_frame_time: Instant::now(),
            frame_count: 0,
            fps_timer: Instant::now(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("Nova GE Window")
                        .with_decorations(true),
                )
                .unwrap(),
        );

        vk_init(&self.shared, &window);
        info!(self.shared, "Render thread initialized");

        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        let window = match &self.window {
            Some(w) => w,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => {
                popup_info!(
                    self.shared,
                    "being annoying on purpose",
                    "I see you wanted to close this app so we'll give you this annoying ass popup"
                );
                vk_shutdown();
                event_loop.exit();
            }

            WindowEvent::Resized(_) => {
                vk_handle_resize();
            }

            WindowEvent::KeyboardInput {
                event: KeyEvent { physical_key, .. },
                ..
            } => {
                if physical_key == PhysicalKey::Code(KeyCode::KeyS) {
                    let mut vs_data = Vec::<u32>::new();
                    let mut fs_data = Vec::<u32>::new();

                    match get_asset(&self.shared, "test_vs2") {
                        Asset::Shader(mut data) => vs_data.append(&mut data),
                        _ => err!(self.shared, "Failed to get the test vertex shader"),
                    }

                    match get_asset(&self.shared, "test_fs2") {
                        Asset::Shader(mut data) => fs_data.append(&mut data),
                        _ => err!(self.shared, "Failed to get the test fragment shader"),
                    }

                    vk_reload_shaders(&vs_data, &fs_data);
                }
            }

            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let delta = now.duration_since(self.last_frame_time);
                self.last_frame_time = now;
                self.frame_count += 1;

                if let Ok(mut stats) = self.shared.render_stats.lock() {
                    stats.frametime = delta.as_micros();
                    if self.fps_timer.elapsed() >= Duration::from_secs(1) {
                        stats.framerate =
                            self.frame_count as f32 / self.fps_timer.elapsed().as_secs_f32();
                        println!(
                            "FPS: {:.2}, Frametime: {} microseconds",
                            stats.framerate, stats.frametime
                        );
                        self.frame_count = 0;
                        self.fps_timer = Instant::now();
                    }
                }

                vk_render(window);
                window.request_redraw();
            }

            _ => {}
        }
    }
}

pub fn main(shared: SharedData) {
    drop(shared.logic_init.lock());
    drop(shared.io_init.lock());
    drop(shared.audio_init.lock());

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new(shared);
    event_loop.run_app(&mut app).unwrap();
}
