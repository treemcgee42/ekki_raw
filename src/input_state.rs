use crate::application_state::ApplicationState;

impl ApplicationState {}

pub fn viewport_camera_rotate(egui_input_state: &eframe::egui::InputState) -> bool {
    egui_input_state.key_down(eframe::egui::Key::Z)
        && egui_input_state
            .pointer
            .button_down(eframe::egui::PointerButton::Primary)
}
