use imgui::{Context, Key, MouseButton as ImGuiMouseButton};

pub struct ImguiGLFWSupport {
    hidpi_factor: (f32, f32),
    modifiers: glfw::Modifiers,
    cursor_pos: [f32; 2],
}

impl ImguiGLFWSupport {
    pub fn init(imgui: &mut Context, window: &glfw::PWindow) -> Self {
        let hidpi_factor = window.get_content_scale();

        let io = imgui.io_mut();
        io.display_framebuffer_scale = [hidpi_factor.0, hidpi_factor.1];

        let window_size = window.get_size();
        io.display_size = [
            window_size.0 as f32 / hidpi_factor.0,
            window_size.1 as f32 / hidpi_factor.1,
        ];

        io.backend_flags.insert(imgui::BackendFlags::HAS_MOUSE_CURSORS);
        io.backend_flags.insert(imgui::BackendFlags::HAS_SET_MOUSE_POS);

        Self {
            hidpi_factor,
            modifiers: glfw::Modifiers::empty(),
            cursor_pos: [0.0, 0.0],
        }
    }

    pub fn handle_window_event(&mut self, imgui: &mut Context, window: &glfw::PWindow, event: &glfw::WindowEvent) {
        let io = imgui.io_mut();

        match event {
            glfw::WindowEvent::Size(w,h) => {
                io.display_size = [
                    *w as f32 / self.hidpi_factor.0,
                    *h as f32 / self.hidpi_factor.1,
                ];
            }

            glfw::WindowEvent::ContentScale(x,y) => {
                self.hidpi_factor = (*x,*y);
                io.display_framebuffer_scale = [self.hidpi_factor.0, self.hidpi_factor.1];

                let window_size = window.get_size();
                io.display_size = [
                    window_size.0 as f32 / self.hidpi_factor.0,
                    window_size.1 as f32 / self.hidpi_factor.1,
                ];
            }

            glfw::WindowEvent::MouseButton(b,a,_) => {
                let pressed = *a == glfw::Action::Press;

                match *b {
                    glfw::MouseButton::Left => io.add_mouse_button_event(ImGuiMouseButton::Left, pressed),
                    glfw::MouseButton::Right => io.add_mouse_button_event(ImGuiMouseButton::Right, pressed),
                    glfw::MouseButton::Middle => io.add_mouse_button_event(ImGuiMouseButton::Middle, pressed),
                    glfw::MouseButton::Button4 => io.add_mouse_button_event(ImGuiMouseButton::Extra1, pressed),
                    glfw::MouseButton::Button6 => io.add_mouse_button_event(ImGuiMouseButton::Extra2, pressed),
                    _ => {}
                }
            }

            glfw::WindowEvent::CursorPos(x,y) => {
                self.cursor_pos = [
                    *x as f32 / self.hidpi_factor.0,
                    *y as f32 / self.hidpi_factor.1,
                ];
                io.add_mouse_pos_event(self.cursor_pos);
            }

            //glfw::WindowEvent:: { .. } => {
            //    self.cursor_pos = [-f32::MAX, -f32::MAX];
            //    io.add_mouse_pos_event(self.cursor_pos);
            //}

            glfw::WindowEvent::Scroll(h,v) => {
                io.add_mouse_wheel_event([*h as f32, *v as f32]);
            }

            glfw::WindowEvent::Key( k,_,a,m ) => {
                self.modifiers = *m;
                let pressed = *a == glfw::Action::Press;

                if let Some(key) = Self::translate_key(*k) {
                    io.add_key_event(key, pressed);
                }

                match k {
                    glfw::Key::LeftControl | glfw::Key::RightControl => {
                        io.add_key_event(Key::ModCtrl, pressed);
                    }
                    glfw::Key::LeftShift | glfw::Key::RightShift => {
                        io.add_key_event(Key::ModShift, pressed);
                    }
                    glfw::Key::LeftAlt | glfw::Key::RightAlt => {
                        io.add_key_event(Key::ModAlt, pressed);
                    }
                    glfw::Key::LeftSuper | glfw::Key::RightSuper => {
                        io.add_key_event(Key::ModSuper, pressed);
                    }
                    _ => {}
                }
            }

            glfw::WindowEvent::Char(c) => {
                if c.is_control() {
                    return;
                }
                io.add_input_character(*c);
            }

            glfw::WindowEvent::Focus(focused) => {
                io.app_focus_lost = !focused;
            }

            _ => {}
        }
    }

    fn translate_key(keycode: glfw::Key) -> Option<Key> {
        match keycode {
            glfw::Key::Space => Some(Key::Space),
            glfw::Key::Apostrophe => Some(Key::Apostrophe),
            glfw::Key::Comma => Some(Key::Comma),
            glfw::Key::Minus => Some(Key::Minus),
            glfw::Key::Period => Some(Key::Period),
            glfw::Key::Slash => Some(Key::Slash),
            glfw::Key::Num0 => Some(Key::Alpha0),
            glfw::Key::Num1 => Some(Key::Alpha1),
            glfw::Key::Num2 => Some(Key::Alpha2),
            glfw::Key::Num3 => Some(Key::Alpha3),
            glfw::Key::Num4 => Some(Key::Alpha4),
            glfw::Key::Num5 => Some(Key::Alpha5),
            glfw::Key::Num6 => Some(Key::Alpha6),
            glfw::Key::Num7 => Some(Key::Alpha7),
            glfw::Key::Num8 => Some(Key::Alpha8),
            glfw::Key::Num9 => Some(Key::Alpha9),
            glfw::Key::Semicolon => Some(Key::Semicolon),
            glfw::Key::Equal => Some(Key::Equal),
            glfw::Key::A => Some(Key::A),
            glfw::Key::B => Some(Key::B),
            glfw::Key::C => Some(Key::C),
            glfw::Key::D => Some(Key::D),
            glfw::Key::E => Some(Key::E),
            glfw::Key::F => Some(Key::F),
            glfw::Key::G => Some(Key::G),
            glfw::Key::H => Some(Key::H),
            glfw::Key::I => Some(Key::I),
            glfw::Key::J => Some(Key::J),
            glfw::Key::K => Some(Key::K),
            glfw::Key::L => Some(Key::L),
            glfw::Key::M => Some(Key::M),
            glfw::Key::N => Some(Key::N),
            glfw::Key::O => Some(Key::O),
            glfw::Key::P => Some(Key::P),
            glfw::Key::Q => Some(Key::Q),
            glfw::Key::R => Some(Key::R),
            glfw::Key::S => Some(Key::S),
            glfw::Key::T => Some(Key::T),
            glfw::Key::U => Some(Key::U),
            glfw::Key::V => Some(Key::V),
            glfw::Key::W => Some(Key::W),
            glfw::Key::X => Some(Key::X),
            glfw::Key::Y => Some(Key::Y),
            glfw::Key::Z => Some(Key::Z),
            glfw::Key::LeftBracket => Some(Key::LeftBracket),
            glfw::Key::Backslash => Some(Key::Backslash),
            glfw::Key::RightBracket => Some(Key::RightBracket),
            glfw::Key::GraveAccent => Some(Key::GraveAccent),
            //glfw::Key::World1 => Some(Key::World1), Doubt ImGui supports these
            //glfw::Key::World2 => Some(Key::World2),
            glfw::Key::Escape => Some(Key::Escape),
            glfw::Key::Enter => Some(Key::Enter),
            glfw::Key::Tab => Some(Key::Tab),
            glfw::Key::Backspace => Some(Key::Backspace),
            glfw::Key::Insert => Some(Key::Insert),
            glfw::Key::Delete => Some(Key::Delete),
            glfw::Key::Right => Some(Key::RightArrow),
            glfw::Key::Left => Some(Key::LeftArrow),
            glfw::Key::Down => Some(Key::DownArrow),
            glfw::Key::Up => Some(Key::UpArrow),
            glfw::Key::PageUp => Some(Key::PageUp),
            glfw::Key::PageDown => Some(Key::PageDown),
            glfw::Key::Home => Some(Key::Home),
            glfw::Key::End => Some(Key::End),
            glfw::Key::CapsLock => Some(Key::CapsLock),
            glfw::Key::ScrollLock => Some(Key::ScrollLock),
            glfw::Key::NumLock => Some(Key::NumLock),
            glfw::Key::PrintScreen => Some(Key::PrintScreen),
            glfw::Key::Pause => Some(Key::Pause),
            glfw::Key::F1 => Some(Key::F1),
            glfw::Key::F2 => Some(Key::F2),
            glfw::Key::F3 => Some(Key::F3),
            glfw::Key::F4 => Some(Key::F4),
            glfw::Key::F5 => Some(Key::F5),
            glfw::Key::F6 => Some(Key::F6),
            glfw::Key::F7 => Some(Key::F7),
            glfw::Key::F8 => Some(Key::F8),
            glfw::Key::F9 => Some(Key::F9),
            glfw::Key::F10 => Some(Key::F10),
            glfw::Key::F11 => Some(Key::F11),
            glfw::Key::F12 => Some(Key::F12),
            glfw::Key::KpDecimal => Some(Key::KeypadDecimal),
            glfw::Key::KpDivide => Some(Key::KeypadDivide),
            glfw::Key::KpMultiply => Some(Key::KeypadMultiply),
            glfw::Key::KpSubtract => Some(Key::KeypadSubtract),
            glfw::Key::KpAdd => Some(Key::KeypadAdd),
            glfw::Key::KpEnter => Some(Key::KeypadEnter),
            glfw::Key::KpEqual => Some(Key::Equal),
            glfw::Key::LeftShift => Some(Key::LeftShift),
            glfw::Key::LeftControl => Some(Key::LeftCtrl),
            glfw::Key::LeftAlt => Some(Key::LeftAlt),
            glfw::Key::LeftSuper => Some(Key::LeftSuper),
            glfw::Key::RightShift => Some(Key::RightShift),
            glfw::Key::RightControl => Some(Key::RightCtrl),
            glfw::Key::RightAlt => Some(Key::RightAlt),
            glfw::Key::RightSuper => Some(Key::RightSuper),
            glfw::Key::Menu => Some(Key::Menu),
            _ => None
        }
    }

    pub fn prepare_frame(&mut self, imgui: &mut Context, window: &glfw::PWindow) {
        let io = imgui.io_mut();
        let window_size = window.get_size();
        io.display_size = [
            window_size.0 as f32 / self.hidpi_factor.0,
            window_size.1 as f32 / self.hidpi_factor.1,
        ];
    }
}