use imgui::{Context, Key, MouseButton as ImGuiMouseButton};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent},
    keyboard::{KeyCode, ModifiersState, PhysicalKey},
    window::Window,
};

pub struct ImguiWinitPlatform {
    hidpi_factor: f32,
    modifiers: ModifiersState,
    cursor_pos: [f32; 2],
}

impl ImguiWinitPlatform {
    pub fn init(imgui: &mut Context, window: &Window) -> Self {
        let hidpi_factor = window.scale_factor() as f32;

        let io = imgui.io_mut();
        io.display_framebuffer_scale = [hidpi_factor, hidpi_factor];

        let window_size = window.inner_size();
        io.display_size = [
            window_size.width as f32 / hidpi_factor,
            window_size.height as f32 / hidpi_factor,
        ];

        io.backend_flags.insert(imgui::BackendFlags::HAS_MOUSE_CURSORS);
        io.backend_flags.insert(imgui::BackendFlags::HAS_SET_MOUSE_POS);

        Self {
            hidpi_factor,
            modifiers: ModifiersState::default(),
            cursor_pos: [0.0, 0.0],
        }
    }

    pub fn handle_event(&mut self, imgui: &mut Context, window: &Window, event: &Event<()>) {
        match event {
            Event::WindowEvent { event, .. } => self.handle_window_event(imgui, window, event),
            _ => {}
        }
    }

    fn handle_window_event(&mut self, imgui: &mut Context, window: &Window, event: &WindowEvent) {
        let io = imgui.io_mut();

        match event {
            WindowEvent::Resized(size) => {
                io.display_size = [
                    size.width as f32 / self.hidpi_factor,
                    size.height as f32 / self.hidpi_factor,
                ];
            }

            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.hidpi_factor = *scale_factor as f32;
                io.display_framebuffer_scale = [self.hidpi_factor, self.hidpi_factor];

                let window_size = window.inner_size();
                io.display_size = [
                    window_size.width as f32 / self.hidpi_factor,
                    window_size.height as f32 / self.hidpi_factor,
                ];
            }

            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = *state == ElementState::Pressed;

                match button {
                    MouseButton::Left => io.add_mouse_button_event(ImGuiMouseButton::Left, pressed),
                    MouseButton::Right => io.add_mouse_button_event(ImGuiMouseButton::Right, pressed),
                    MouseButton::Middle => io.add_mouse_button_event(ImGuiMouseButton::Middle, pressed),
                    MouseButton::Back => io.add_mouse_button_event(ImGuiMouseButton::Extra1, pressed),
                    MouseButton::Forward => io.add_mouse_button_event(ImGuiMouseButton::Extra2, pressed),
                    _ => {}
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_pos = [
                    position.x as f32 / self.hidpi_factor,
                    position.y as f32 / self.hidpi_factor,
                ];
                io.add_mouse_pos_event(self.cursor_pos);
            }

            WindowEvent::CursorLeft { .. } => {
                self.cursor_pos = [-f32::MAX, -f32::MAX];
                io.add_mouse_pos_event(self.cursor_pos);
            }

            WindowEvent::MouseWheel { delta, phase, .. } => {
                if *phase != TouchPhase::Moved {
                    return;
                }

                let (h, v) = match delta {
                    MouseScrollDelta::LineDelta(h, v) => (*h, *v),
                    MouseScrollDelta::PixelDelta(pos) => {
                        (pos.x as f32 / 20.0, pos.y as f32 / 20.0)
                    }
                };

                io.add_mouse_wheel_event([h, v]);
            }

            WindowEvent::KeyboardInput { event, .. } => {
                let pressed = event.state == ElementState::Pressed;

                if let PhysicalKey::Code(keycode) = event.physical_key {
                    if let Some(key) = Self::translate_key(keycode) {
                        io.add_key_event(key, pressed);
                    }

                    match keycode {
                        KeyCode::ControlLeft | KeyCode::ControlRight => {
                            io.add_key_event(Key::ModCtrl, pressed);
                        }
                        KeyCode::ShiftLeft | KeyCode::ShiftRight => {
                            io.add_key_event(Key::ModShift, pressed);
                        }
                        KeyCode::AltLeft | KeyCode::AltRight => {
                            io.add_key_event(Key::ModAlt, pressed);
                        }
                        KeyCode::SuperLeft | KeyCode::SuperRight => {
                            io.add_key_event(Key::ModSuper, pressed);
                        }
                        _ => {}
                    }
                }

                if pressed {
                    if let Some(text) = &event.text {
                        for ch in text.chars() {
                            if ch.is_control() {
                                continue;
                            }
                            io.add_input_character(ch);
                        }
                    }
                }
            }

            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = modifiers.state();
                io.add_key_event(Key::ModCtrl, self.modifiers.control_key());
                io.add_key_event(Key::ModShift, self.modifiers.shift_key());
                io.add_key_event(Key::ModAlt, self.modifiers.alt_key());
                io.add_key_event(Key::ModSuper, self.modifiers.super_key());
            }

            WindowEvent::Focused(focused) => {
                io.app_focus_lost = !focused;
            }

            _ => {}
        }
    }

    fn translate_key(keycode: KeyCode) -> Option<Key> {
        match keycode {
            KeyCode::Tab => Some(Key::Tab),
            KeyCode::ArrowLeft => Some(Key::LeftArrow),
            KeyCode::ArrowRight => Some(Key::RightArrow),
            KeyCode::ArrowUp => Some(Key::UpArrow),
            KeyCode::ArrowDown => Some(Key::DownArrow),
            KeyCode::PageUp => Some(Key::PageUp),
            KeyCode::PageDown => Some(Key::PageDown),
            KeyCode::Home => Some(Key::Home),
            KeyCode::End => Some(Key::End),
            KeyCode::Insert => Some(Key::Insert),
            KeyCode::Delete => Some(Key::Delete),
            KeyCode::Backspace => Some(Key::Backspace),
            KeyCode::Space => Some(Key::Space),
            KeyCode::Enter => Some(Key::Enter),
            KeyCode::Escape => Some(Key::Escape),
            KeyCode::Quote => Some(Key::Apostrophe),
            KeyCode::Comma => Some(Key::Comma),
            KeyCode::Minus => Some(Key::Minus),
            KeyCode::Period => Some(Key::Period),
            KeyCode::Slash => Some(Key::Slash),
            KeyCode::Semicolon => Some(Key::Semicolon),
            KeyCode::Equal => Some(Key::Equal),
            KeyCode::BracketLeft => Some(Key::LeftBracket),
            KeyCode::Backslash => Some(Key::Backslash),
            KeyCode::BracketRight => Some(Key::RightBracket),
            KeyCode::Backquote => Some(Key::GraveAccent),
            KeyCode::CapsLock => Some(Key::CapsLock),
            KeyCode::ScrollLock => Some(Key::ScrollLock),
            KeyCode::NumLock => Some(Key::NumLock),
            KeyCode::PrintScreen => Some(Key::PrintScreen),
            KeyCode::Pause => Some(Key::Pause),
            KeyCode::Numpad0 => Some(Key::Keypad0),
            KeyCode::Numpad1 => Some(Key::Keypad1),
            KeyCode::Numpad2 => Some(Key::Keypad2),
            KeyCode::Numpad3 => Some(Key::Keypad3),
            KeyCode::Numpad4 => Some(Key::Keypad4),
            KeyCode::Numpad5 => Some(Key::Keypad5),
            KeyCode::Numpad6 => Some(Key::Keypad6),
            KeyCode::Numpad7 => Some(Key::Keypad7),
            KeyCode::Numpad8 => Some(Key::Keypad8),
            KeyCode::Numpad9 => Some(Key::Keypad9),
            KeyCode::NumpadDecimal => Some(Key::KeypadDecimal),
            KeyCode::NumpadDivide => Some(Key::KeypadDivide),
            KeyCode::NumpadMultiply => Some(Key::KeypadMultiply),
            KeyCode::NumpadSubtract => Some(Key::KeypadSubtract),
            KeyCode::NumpadAdd => Some(Key::KeypadAdd),
            KeyCode::NumpadEnter => Some(Key::KeypadEnter),
            KeyCode::NumpadEqual => Some(Key::KeypadEqual),
            KeyCode::ControlLeft => Some(Key::LeftCtrl),
            KeyCode::ShiftLeft => Some(Key::LeftShift),
            KeyCode::AltLeft => Some(Key::LeftAlt),
            KeyCode::SuperLeft => Some(Key::LeftSuper),
            KeyCode::ControlRight => Some(Key::RightCtrl),
            KeyCode::ShiftRight => Some(Key::RightShift),
            KeyCode::AltRight => Some(Key::RightAlt),
            KeyCode::SuperRight => Some(Key::RightSuper),
            KeyCode::ContextMenu => Some(Key::Menu),
            KeyCode::Digit0 => Some(Key::Alpha0),
            KeyCode::Digit1 => Some(Key::Alpha1),
            KeyCode::Digit2 => Some(Key::Alpha2),
            KeyCode::Digit3 => Some(Key::Alpha3),
            KeyCode::Digit4 => Some(Key::Alpha4),
            KeyCode::Digit5 => Some(Key::Alpha5),
            KeyCode::Digit6 => Some(Key::Alpha6),
            KeyCode::Digit7 => Some(Key::Alpha7),
            KeyCode::Digit8 => Some(Key::Alpha8),
            KeyCode::Digit9 => Some(Key::Alpha9),
            KeyCode::KeyA => Some(Key::A),
            KeyCode::KeyB => Some(Key::B),
            KeyCode::KeyC => Some(Key::C),
            KeyCode::KeyD => Some(Key::D),
            KeyCode::KeyE => Some(Key::E),
            KeyCode::KeyF => Some(Key::F),
            KeyCode::KeyG => Some(Key::G),
            KeyCode::KeyH => Some(Key::H),
            KeyCode::KeyI => Some(Key::I),
            KeyCode::KeyJ => Some(Key::J),
            KeyCode::KeyK => Some(Key::K),
            KeyCode::KeyL => Some(Key::L),
            KeyCode::KeyM => Some(Key::M),
            KeyCode::KeyN => Some(Key::N),
            KeyCode::KeyO => Some(Key::O),
            KeyCode::KeyP => Some(Key::P),
            KeyCode::KeyQ => Some(Key::Q),
            KeyCode::KeyR => Some(Key::R),
            KeyCode::KeyS => Some(Key::S),
            KeyCode::KeyT => Some(Key::T),
            KeyCode::KeyU => Some(Key::U),
            KeyCode::KeyV => Some(Key::V),
            KeyCode::KeyW => Some(Key::W),
            KeyCode::KeyX => Some(Key::X),
            KeyCode::KeyY => Some(Key::Y),
            KeyCode::KeyZ => Some(Key::Z),
            KeyCode::F1 => Some(Key::F1),
            KeyCode::F2 => Some(Key::F2),
            KeyCode::F3 => Some(Key::F3),
            KeyCode::F4 => Some(Key::F4),
            KeyCode::F5 => Some(Key::F5),
            KeyCode::F6 => Some(Key::F6),
            KeyCode::F7 => Some(Key::F7),
            KeyCode::F8 => Some(Key::F8),
            KeyCode::F9 => Some(Key::F9),
            KeyCode::F10 => Some(Key::F10),
            KeyCode::F11 => Some(Key::F11),
            KeyCode::F12 => Some(Key::F12),
            _ => None,
        }
    }

    pub fn prepare_frame(&mut self, imgui: &mut Context, window: &Window) {
        let io = imgui.io_mut();
        let window_size = window.inner_size();
        io.display_size = [
            window_size.width as f32 / self.hidpi_factor,
            window_size.height as f32 / self.hidpi_factor,
        ];
    }
}