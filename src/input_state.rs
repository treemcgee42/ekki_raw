pub struct InputState {
    pub mouse_state: MouseState,
}

impl InputState {
    pub fn initialize() -> Self {
        let mouse_state = MouseState::default();

        Self { mouse_state }
    }
}

// S==== MOUSE STATE {{{1

pub struct MouseState {
    pub position: (f64, f64),
    pub button_state: MouseButtonState,
}

impl Default for MouseState {
    fn default() -> Self {
        Self {
            position: (0.0, 0.0),
            button_state: MouseButtonState::default(),
        }
    }
}

pub struct MouseButtonState {
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub middle_pressed: bool,
}

impl Default for MouseButtonState {
    fn default() -> Self {
        Self {
            left_pressed: false,
            right_pressed: false,
            middle_pressed: false,
        }
    }
}

// E==== MOUSE STATE }}}1
