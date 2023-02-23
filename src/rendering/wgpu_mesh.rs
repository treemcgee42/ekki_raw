use std::sync::Arc;

use eframe::wgpu::util::DeviceExt;

use crate::meshes::Mesh;
use crate::vertex::Vertex;

/// A representation of the mesh that can basically be passed directly to WGPU.
/// It stores things directly in `wgpu::Buffer`s.
pub struct WgpuMesh {
    pub vertex_buffer: eframe::wgpu::Buffer,
    pub index_buffer: eframe::wgpu::Buffer,
    pub num_indices: u32,
}

impl WgpuMesh {
    /// Collects the vertex/index data for a collection of meshes into a
    /// single vertex/index buffer. This is a utility function for preparing
    /// a single draw call for a collection of meshes.
    pub fn from_meshes(device: &eframe::wgpu::Device, meshes: &Vec<Arc<Mesh>>) -> Self {
        let mut vertices = Vec::<Vertex>::new();
        let mut indices = Vec::<u16>::new();
        for mesh in meshes {
            // println!(
            //     "drawing {} verts, {} indices",
            //     mesh.vertices.len(),
            //     mesh.indices.len()
            // );
            // We need to copy the elements over anyways since we are
            // sending the data to the GPU. `append()` destructs the
            // parameter given to it, but cloning the entire vertex
            // is not any more work than we would need to do. Even
            // if we just copied references, we would have to perform
            // a copy when sending it to the GPU. Here, we copy up front
            // and just move later.
            vertices.append(&mut mesh.vertices.clone());

            // TODO: NEED TO ADJUST INDEX NUMBERS
            indices.append(&mut mesh.indices.clone());
        }

        let vertex_buffer = device.create_buffer_init(&eframe::wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices.as_slice()),
            usage: eframe::wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&eframe::wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices.as_slice()),
            usage: eframe::wgpu::BufferUsages::INDEX,
        });
        let num_indices = indices.len() as u32;

        WgpuMesh {
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }
}
