use glfw::PWindow;
use crate::render::vk::{handle_event, reload_shaders, render, shutdown, VkState};
use crate::shared::SharedData;

pub struct VulkanRenderer {
    state: VkState,
    shared: SharedData,
}

impl VulkanRenderer {
    pub fn new(state: VkState, shared: SharedData) -> VulkanRenderer {
        VulkanRenderer {
            state,
            shared
        }
    }

    pub fn with_imgui_ctx<F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce(&mut imgui::Context) -> R,
    {
        Some(f(&mut self.state.imgui))
    }

    pub fn render(&mut self, window: &PWindow) {
        render(&mut self.state, window);
    }

    pub fn with_imgui_ui<F>(&mut self, win: &PWindow,f: F)
    where
        F: FnOnce(&imgui::Ui),
    {

        self.state.imgui_platform
            .prepare_frame(&mut self.state.imgui, win);

        let ui = self.state.imgui.frame();

        f(&ui);

        let draw_data = self.state.imgui.render();
        self.state.imgui_draw_data = Some(draw_data as *const _);
    }

    pub fn reload_shaders(&mut self, vert_spirv: &[u32], frag_spirv: &[u32]) {
        reload_shaders(&mut self.state, vert_spirv, frag_spirv);
    }

    pub fn handle_event(&mut self, window: &PWindow, event: &glfw::WindowEvent) {
        handle_event(&mut self.state, window, event);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.state.recreate_swapchain = true;
    }

    pub fn destroy(&mut self) {
        shutdown(&mut self.state);
    }
}