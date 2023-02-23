use crate::meshes::Mesh;
use std::sync::Arc;

use super::wgpu_mesh::WgpuMesh;

pub struct DrawCommand {
    pub wgpu_mesh: WgpuMesh,
    pub kind: DrawCommandKind,
}

pub enum DrawCommandKind {
    // Do a "draw indexed" call on all the indices
    DrawIndexedAll,
}

impl DrawCommand {
    /// Prepare a collection of meshes to be drawn. The output is a single
    /// `DrawCommand` that can be executed to draw the specified meshes.
    /// The order in which the meshes are drawn (within the same GPU draw
    /// call) is not well-defined.
    pub fn from_meshes(device: &eframe::wgpu::Device, meshes: &Vec<Arc<Mesh>>) -> DrawCommand {
        DrawCommand {
            wgpu_mesh: WgpuMesh::from_meshes(device, meshes),
            kind: DrawCommandKind::DrawIndexedAll,
        }
    }
}
