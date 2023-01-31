use std::rc::Rc;

use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub struct WindowState {
    pub raw_window: Rc<Window>,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl WindowState {
    pub fn initialize(window: Rc<Window>) -> Self {
        let size = window.inner_size();

        Self {
            raw_window: window,
            size,
        }
    }
}

pub fn create_window() -> (EventLoop<()>, Window) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("ekki")
        .build(&event_loop)
        .unwrap();

    (event_loop, window)
}
