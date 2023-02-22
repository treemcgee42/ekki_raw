#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn get_descriptor<'a>() -> eframe::wgpu::VertexBufferLayout<'a> {
        eframe::wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as eframe::wgpu::BufferAddress,
            step_mode: eframe::wgpu::VertexStepMode::Vertex,
            attributes: &[
                eframe::wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: eframe::wgpu::VertexFormat::Float32x3,
                },
                eframe::wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as eframe::wgpu::BufferAddress,
                    shader_location: 1,
                    format: eframe::wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
