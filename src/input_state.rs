pub struct PressedBool {
    pub is_pressed: bool,
}

impl Default for PressedBool {
    fn default() -> Self {
        Self { is_pressed: false }
    }
}

pub struct InputState {
    pub keyboard_state: KeyboardState,
    pub mouse_state: MouseState,
}

impl InputState {
    pub fn initialize() -> Self {
        Self {
            keyboard_state: KeyboardState::default(),
            mouse_state: MouseState::default(),
        }
    }
}

// keybindings
impl InputState {
    pub fn viewport_camera_rotate(&self) -> bool {
        self.keyboard_state.option_alt_key.is_pressed
            && self.mouse_state.button_state.left_button.is_pressed
    }
}

// S==== KEYBOARD STATE {{{1

pub struct KeyboardState {
    pub option_alt_key: PressedBool,
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self {
            option_alt_key: PressedBool::default(),
        }
    }
}

// E==== KEYBOARD STATE }}}1

// S==== MOUSE STATE {{{1

pub struct MouseState {
    pub previous_position: (f64, f64),
    pub current_position: (f64, f64),
    pub button_state: MouseButtonState,
}

impl Default for MouseState {
    fn default() -> Self {
        Self {
            previous_position: (0.0, 0.0),
            current_position: (0.0, 0.0),
            button_state: MouseButtonState::default(),
        }
    }
}

pub struct MouseButtonState {
    pub left_button: ButtonPressedInfo,
    pub right_pressed: bool,
    pub middle_pressed: bool,
}

impl Default for MouseButtonState {
    fn default() -> Self {
        Self {
            left_button: ButtonPressedInfo::default(),
            right_pressed: false,
            middle_pressed: false,
        }
    }
}

pub struct ButtonPressedInfo {
    pub is_pressed: bool,
    pub position_when_pressed: Option<(f64, f64)>,
}

impl Default for ButtonPressedInfo {
    fn default() -> Self {
        Self {
            is_pressed: false,
            position_when_pressed: None,
        }
    }
}

// E==== MOUSE STATE }}}1
