use std::rc::Rc;

use camera::Camera;
use draw::draw_meshes;
// use event_loop::run_event_loop;
use meshes::MeshBank;
use wgpu_setup::{ApplicationState, RenderResources};
use windowing::{create_window, WindowState};

//mod bmesh;
mod camera;
mod draw;
mod edges;
// mod event_loop;
mod grid;
mod input_state;
mod math;
mod meshes;
mod vertex;
mod wgpu_setup;
mod windowing;

struct App {
    state: ApplicationState,
}

impl App {
    pub fn initialize<'a>(eframe_creation_context: &'a eframe::CreationContext<'a>) -> Self {
        let mut state = ApplicationState::initialize();
        RenderResources::initialize(
            eframe_creation_context,
            &state.camera,
            &state.drawing_stuff.camera_uniform,
        );

        draw_cube(&mut state);

        Self { state }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            eframe::egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("The triangle is being painted using ");
                        ui.hyperlink_to("WGPU", "https://wgpu.rs");
                        ui.label(" (Portable Rust graphics API awesomeness)");
                    });
                    ui.label("It's not a very impressive demo, but it shows you can embed 3D inside of egui.");

                    eframe::egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        self.state.custom_painting(ui);
                    });
                    ui.label("Drag to rotate!");
                });
        });
    }
}

fn draw_cube(state: &mut ApplicationState) {
    state
        .drawing_stuff
        .meshes_to_draw
        .push(state.mesh_bank.get(meshes::MeshBankKeys::Cube));
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::vec2(350.0, 380.0)),
        multisampling: 1,
        depth_buffer: 1, // bool
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        "Custom 3D painting in eframe using glow",
        options,
        Box::new(|cc| Box::new(App::initialize(cc))),
    )
}
