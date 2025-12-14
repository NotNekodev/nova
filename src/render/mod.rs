pub mod vk;
pub mod imgui_glfw_support; // lets pretend this is the crate okay?

use crate::{*,shared::*};

pub fn main(shared: SharedData) {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap_or_else(|e| {
        fatal!(shared, "Failed to initialize GLFW: {e}");
    });

    //we are not using OpenGL bruv
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

    let (mut window, events) = glfw.create_window(800, 600, "Nova GE window", glfw::WindowMode::Windowed)
        .unwrap_or_else(|| {
            fatal!(shared,"Failed to create GLFW window");
        });

    window.set_cursor_mode(glfw::CursorMode::Normal);
    window.set_all_polling(true);

    vk::init(&shared, &window);

    info!(shared,"Render thread initialized");
    info!(shared,"Nova engine v{VERSION} initialized successfully!");

    let mut last_frame_time = Instant::now();
    let mut frame_count = 0;
    let mut fps_timer = Instant::now();

    while !window.should_close() {
        glfw.poll_events();

        for (_,e) in glfw::flush_messages(&events) {
            vk::handle_event(&window, &e);
            match e {
                glfw::WindowEvent::Close => {
                    window.set_should_close(true);
                },
                glfw::WindowEvent::Key(k,_,_,_)  => {
                    if k == glfw::Key::S {
                        let mut vs_data = Vec::<u32>::new();
                        let mut fs_data = Vec::<u32>::new();

                        match get_asset(&shared, "test_vs2") {
                            Asset::Shader(mut data) => vs_data.append(&mut data),
                            _ => err!(shared, "Failed to get the test vertex shader"),
                        }

                        match get_asset(&shared, "test_fs2") {
                            Asset::Shader(mut data) => fs_data.append(&mut data),
                            _ => err!(shared, "Failed to get the test fragment shader"),
                        }

                        vk::reload_shaders(&vs_data, &fs_data);
                    }
                },
                glfw::WindowEvent::Size(_,_) => vk::handle_resize(),
                _ => ()
            }
        }

        let now = Instant::now();
        let delta = now.duration_since(last_frame_time);
        last_frame_time = now;
        frame_count += 1;

        if fps_timer.elapsed() >= Duration::from_secs(1) {

            //FIXME: stats updates may block this thread and make it wait
            if let Ok(mut stats) = shared.render_stats.lock() {
                stats.frametime = delta.as_micros();
                stats.framerate =
                    frame_count as f32 / fps_timer.elapsed().as_secs_f32();
                println!(
                    "FPS: {:.2}, Frametime: {} microseconds",
                    stats.framerate, stats.frametime
                );
                frame_count = 0;
                fps_timer = Instant::now();
            }
        }

        vk::with_imgui_ctx(|ctx| {
            ctx.io_mut().delta_time = delta.as_secs_f32();
        });

        vk::with_imgui_ui(&window,|ui| {
            let mut opened = false;
            ui.show_demo_window(&mut opened);
        });

        vk::render(&window);
    }

    vk::shutdown();

    //I'll still be a little troll
    popup_info!(
        shared,
        "being annoying on purpose",
        "I see you wanted to close this app so we'll give you this annoying ass popup"
    );

    //drop takes care of destroying the window
}
