use wgpu::util::DeviceExt;

use crate::meshes::Mesh;
use crate::vertex::Vertex;
use std::rc::Rc;

pub struct WgpuMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

pub struct DrawCommand {
    pub wgpu_mesh: WgpuMesh,
    pub kind: DrawCommandKind,
}

pub enum DrawCommandKind {
    // Do a "draw indexed" call on all the indices
    DrawIndexedAll,
}

/// Prepare a collection of meshes to be drawn. The output is a single
/// `DrawCommand` that can be executed to draw the specified meshes.
/// The order in which the meshes are drawn (within the same GPU draw
/// call) is not well-defined.
pub fn draw_meshes(device: &wgpu::Device, meshes: &Vec<Rc<Mesh>>) -> DrawCommand {
    DrawCommand {
        wgpu_mesh: collect_meshes_into_buffers(device, meshes),
        kind: DrawCommandKind::DrawIndexedAll,
    }
}

/// Collects the vertex/index data for a collection of meshes into a
/// single vertex/index buffer. This is a utility function for preparing
/// a single draw call for a collection of meshes.
fn collect_meshes_into_buffers(device: &wgpu::Device, meshes: &Vec<Rc<Mesh>>) -> WgpuMesh {
    let mut vertices = Vec::<Vertex>::new();
    let mut indices = Vec::<u16>::new();
    for mesh in meshes {
        println!(
            "drawing {} verts, {} indices",
            mesh.vertices.len(),
            mesh.indices.len()
        );
        // We need to copy the elements over anyways since we are
        // sending the data to the GPU. `append()` destructs the
        // parameter given to it, but cloning the entire vertex
        // is not any more work than we would need to do. Even
        // if we just copied references, we would have to perform
        // a copy when sending it to the GPU. Here, we copy up front
        // and just move later.
        vertices.append(&mut mesh.vertices.clone());
        indices.append(&mut mesh.indices.clone());
    }

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(vertices.as_slice()),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(indices.as_slice()),
        usage: wgpu::BufferUsages::INDEX,
    });
    let num_indices = indices.len() as u32;

    WgpuMesh {
        vertex_buffer,
        index_buffer,
        num_indices,
    }
}
