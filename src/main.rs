use std::rc::Rc;

use camera::Camera;
use draw::draw_meshes;
use event_loop::run_event_loop;
use input_state::InputState;
use meshes::MeshBank;
use wgpu_setup::Renderer;
use windowing::{create_window, WindowState};
use winit::event_loop::EventLoop;

mod camera;
mod draw;
mod event_loop;
mod input_state;
mod math;
mod meshes;
mod vertex;
mod wgpu_setup;
mod windowing;

pub struct ApplicationState {
    event_loop: Option<EventLoop<()>>,
    camera: Camera,
    renderer: Renderer,
    window_state: WindowState,
    input_state: InputState,
    mesh_bank: MeshBank,
}

impl ApplicationState {
    pub async fn initialize() -> Self {
        let (event_loop, window_) = create_window();

        let window = Rc::new(window_);
        let window_state = WindowState::initialize(window.clone());

        let camera = Camera::initialize(
            window_state.size.width as f32,
            window_state.size.height as f32,
        );

        let renderer = Renderer::initialize(window.clone(), &camera).await;
        let mesh_bank = MeshBank::initialize();

        let input_state = InputState::initialize();

        Self {
            event_loop: Some(event_loop),
            camera,
            renderer,
            window_state,
            input_state,
            mesh_bank,
        }
    }
}

async fn run() {
    env_logger::init();

    let mut app_state = ApplicationState::initialize().await;
    let event_loop = app_state.event_loop.unwrap();
    app_state.event_loop = None;

    draw_cube(&mut app_state);
    run_event_loop(event_loop, app_state).await;
}

fn draw_cube(state: &mut ApplicationState) {
    let meshes_to_draw = vec![state.mesh_bank.get(meshes::MeshBankKeys::Cube)];
    let draw_command = draw_meshes(&state.renderer.device, &meshes_to_draw);
    state.renderer.to_draw.push(draw_command);
}

fn main() {
    pollster::block_on(run());
}
