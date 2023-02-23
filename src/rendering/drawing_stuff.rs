use std::sync::Arc;

use crate::{camera::Camera, math::vector::Vector2, meshes::Mesh};

use super::{grid::GridUniform, viewport::ViewportUniform};

/// This encapsulates things that relate to drawing but are still CPU-specific.
#[derive(Clone)]
pub struct DrawingStuff {
    pub meshes_to_draw: Vec<Arc<Mesh>>,
    pub drawing_region_size: Vector2,
    pub drawing_region_size_updated: bool,
    pub camera_uniform: ViewportUniform,
    pub grid_uniform: GridUniform,
    // depth_texture: DepthTexture,
}

impl DrawingStuff {
    pub fn initialize(camera: &Camera) -> Self {
        DrawingStuff {
            meshes_to_draw: Vec::new(),
            drawing_region_size: Vector2::new(0.0, 0.0),
            drawing_region_size_updated: true,
            camera_uniform: ViewportUniform::new(camera),
            grid_uniform: GridUniform::new(camera),
        }
    }
}
