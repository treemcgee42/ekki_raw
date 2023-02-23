use std::sync::Arc;

use eframe::wgpu::util::DeviceExt;

use crate::camera::Camera;

/// The CPU version of the uniform, which can be directly copied over to the GPU uniform
/// by bytemucking it.
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ViewportUniform {
    view_projection_matrix: [[f32; 4]; 4],
}

impl ViewportUniform {
    pub fn new(camera: &Camera) -> Self {
        Self {
            view_projection_matrix: camera.get_view_projection_matrix().into(),
        }
    }

    /// Copies the view projection matrix from the camera into itself, overwriting the
    /// previous value.
    pub fn update_view_projection_matrix(&mut self, camera: &Camera) {
        self.view_projection_matrix = camera.get_view_projection_matrix().into();
    }
}

/// Rendering resources related to applying the proper perspective to the scene. Does not include
/// buffers for the actual objects in the scene, pretty much just the view projection matrix one.
pub struct ViewportRenderResources {
    pub buffer: eframe::wgpu::Buffer,
    pub bind_group_layout: eframe::wgpu::BindGroupLayout,
    pub bind_group: eframe::wgpu::BindGroup,
}

impl ViewportRenderResources {
    pub fn initialize(
        device: &Arc<eframe::wgpu::Device>,
        camera_uniform: &ViewportUniform,
    ) -> Self {
        let buffer = device.create_buffer_init(&eframe::wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[*camera_uniform]),
            usage: eframe::wgpu::BufferUsages::UNIFORM | eframe::wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout =
            device.create_bind_group_layout(&eframe::wgpu::BindGroupLayoutDescriptor {
                entries: &[eframe::wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: eframe::wgpu::ShaderStages::VERTEX,
                    ty: eframe::wgpu::BindingType::Buffer {
                        ty: eframe::wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("bind_group_layout"),
            });
        let bind_group = device.create_bind_group(&eframe::wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[eframe::wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        Self {
            buffer,
            bind_group_layout,
            bind_group,
        }
    }
}
