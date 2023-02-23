use std::sync::Arc;

use crate::{
    camera::Camera,
    input_state,
    math::vector::Vector2,
    meshes::MeshBank,
    rendering::{drawing_stuff::DrawingStuff, render_resources::RenderResources},
};

pub struct ApplicationState {
    pub camera: Camera,
    pub mesh_bank: MeshBank,
    pub drawing_stuff: DrawingStuff,
    pub doing_turntable: bool,
}

impl ApplicationState {
    pub fn initialize() -> Self {
        let camera = Camera::initialize(0.0, 0.0);
        let mesh_bank = MeshBank::initialize();
        let drawing_stuff = DrawingStuff::initialize(&camera);

        Self {
            camera,
            mesh_bank,
            drawing_stuff,
            doing_turntable: false,
        }
    }

    /// This is where everything on the CPU side should be updated. Updating will be done in the
    /// `prepare()` function once we get the render resources from eframe. This function will call
    /// that as well, after having updated everything CPU side.
    pub fn custom_painting(&mut self, ui: &mut eframe::egui::Ui) {
        let (id, rect) = ui.allocate_space(ui.available_size());

        let rect_size = Vector2::from(rect.size());
        if !Vector2::are_approximately_equal(&rect_size, &self.drawing_stuff.drawing_region_size) {
            self.drawing_stuff.drawing_region_size = rect_size.clone();
            self.drawing_stuff.drawing_region_size_updated = true;
        }

        if self.drawing_stuff.drawing_region_size_updated {
            // update camera width/height and update depth texture
            self.camera.handle_window_resize(
                self.drawing_stuff.drawing_region_size.x(),
                self.drawing_stuff.drawing_region_size.y(),
            );
            self.drawing_stuff.drawing_region_size_updated = false;
        }

        // Take user input and update camera accordingly
        ui.input(|i| self.handle_shortcut_viewport_camera_rotate(i));

        if self.doing_turntable {
            // Tell the UI to listen for drags.
            let _response = ui.interact(rect, id, eframe::egui::Sense::drag());

            let p_start = ui.input(|i| i.pointer.press_origin());
            let p_end = ui.input(|i| i.pointer.hover_pos());

            // Need to be careful that these values are not `None`, which may occur will panning
            // and the cursor leaves the screen, for example.
            if let (Some(start), Some(end)) = (p_start, p_end) {
                let delta_mouse = Vector2::new(end.x - start.x, end.y - start.y);

                self.camera
                    .turntable_rotate(delta_mouse, (rect_size.x(), rect_size.y()));
            }
        }

        // Update CPU side uniforms
        self.drawing_stuff
            .camera_uniform
            .update_view_projection_matrix(&self.camera);
        self.drawing_stuff.grid_uniform.update_matrix(&self.camera);

        // Handle GPU side things TODO
        let meshes_to_draw = self.drawing_stuff.clone();

        let cb = eframe::egui_wgpu::CallbackFn::new()
            .prepare(move |device, queue, _encoder, paint_callback_resources| {
                let resources: &mut RenderResources = paint_callback_resources.get_mut().unwrap();
                resources.prepare(device, queue, meshes_to_draw.clone());
                Vec::new()
            })
            .paint(move |_info, render_pass, paint_callback_resources| {
                let resources: &RenderResources = paint_callback_resources.get().unwrap();
                resources.paint(render_pass);
            });

        let callback = eframe::egui::PaintCallback {
            rect,
            callback: Arc::new(cb),
        };

        ui.painter().add(callback);
    }

    pub fn handle_shortcut_viewport_camera_rotate(
        &mut self,
        egui_input_state: &eframe::egui::InputState,
    ) {
        if input_state::viewport_camera_rotate(egui_input_state) {
            self.doing_turntable = true;
            return;
        }

        if self.doing_turntable {
            self.camera.solidify_view_info();
        }

        self.doing_turntable = false;
    }
}
