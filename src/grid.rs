use std::sync::Arc;

use eframe::wgpu::util::DeviceExt;

use crate::{camera::Camera, math::matrix::Matrix4, wgpu_setup::DepthTexture};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GridUniform {
    view_projection_matrix: [[f32; 4]; 4],
    view_projection_matrix_inverse: [[f32; 4]; 4],
    z_near: f32,
    z_far: f32,
    // Warning: The alignment is sizeof([f32; 4]) = 16, but this does NOT mean that each f32
    // needs 12 bytes of padding. It seems them next to each other, as long as we end on the
    // right alignment size. For example, here we need 8 bytes of padding, but if we had a
    // single f32 followed by a vec4<f32>, then the f32 would need 12 bytes of padding.
    _padding: [i32; 2],
}

impl GridUniform {
    pub fn new(camera: &Camera) -> Self {
        Self {
            view_projection_matrix: camera.get_view_projection_matrix().into(),
            view_projection_matrix_inverse: camera.get_view_projection_matrix_inverse().into(),
            z_near: camera.get_z_near(),
            z_far: camera.get_z_far(),
            _padding: [0; 2],
        }
    }

    pub fn update_matrix(&mut self, camera: &Camera) {
        self.view_projection_matrix = camera.get_view_projection_matrix().into();
        self.view_projection_matrix_inverse = camera.get_view_projection_matrix_inverse().into();
        self.z_near = camera.get_z_near();
        self.z_far = camera.get_z_far();
    }
}

pub struct GridInitializeArgs<'a> {
    pub device: &'a Arc<eframe::wgpu::Device>,
    pub surface_format: eframe::wgpu::TextureFormat,
    pub view_projection_matrix: Matrix4,
    pub camera: &'a Camera,
}

pub struct GridRenderResources {
    pub pipeline: eframe::wgpu::RenderPipeline,
    pub bind_group: eframe::wgpu::BindGroup,
    pub index_buffer: eframe::wgpu::Buffer,
    // pub uniform: GridUniform,
    pub buffer: eframe::wgpu::Buffer,
}

impl GridRenderResources {
    pub fn initialize(args: GridInitializeArgs) -> Self {
        let uniform = GridUniform::new(args.camera);
        let buffer = args
            .device
            .create_buffer_init(&eframe::wgpu::util::BufferInitDescriptor {
                label: Some("grid buffer"),
                contents: bytemuck::cast_slice(&[uniform]),
                usage: eframe::wgpu::BufferUsages::UNIFORM | eframe::wgpu::BufferUsages::COPY_DST,
            });

        let shader = args
            .device
            .create_shader_module(eframe::wgpu::ShaderModuleDescriptor {
                label: Some("grid shader"),
                source: eframe::wgpu::ShaderSource::Wgsl(include_str!("grid.wgsl").into()),
            });

        let bind_group_layout =
            args.device
                .create_bind_group_layout(&eframe::wgpu::BindGroupLayoutDescriptor {
                    entries: &[eframe::wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: eframe::wgpu::ShaderStages::VERTEX
                            | eframe::wgpu::ShaderStages::FRAGMENT,
                        ty: eframe::wgpu::BindingType::Buffer {
                            ty: eframe::wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("grid bind group layout"),
                });
        let bind_group = args
            .device
            .create_bind_group(&eframe::wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[eframe::wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
                label: Some("grid bind group"),
            });

        let render_pipeline_layout =
            args.device
                .create_pipeline_layout(&eframe::wgpu::PipelineLayoutDescriptor {
                    label: Some("grid render pipeline layout"),
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

        let pipeline =
            args.device
                .create_render_pipeline(&eframe::wgpu::RenderPipelineDescriptor {
                    label: Some("grid render pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: eframe::wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_main",
                        buffers: &[],
                    },
                    fragment: Some(eframe::wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[Some(eframe::wgpu::ColorTargetState {
                            format: args.surface_format,
                            blend: Some(eframe::wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: eframe::wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: eframe::wgpu::PrimitiveState {
                        topology: eframe::wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: eframe::wgpu::FrontFace::Ccw,
                        cull_mode: Some(eframe::wgpu::Face::Back),
                        // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                        polygon_mode: eframe::wgpu::PolygonMode::Fill,
                        // Requires Features::DEPTH_CLIP_CONTROL
                        unclipped_depth: false,
                        // Requires Features::CONSERVATIVE_RASTERIZATION
                        conservative: false,
                    },
                    depth_stencil: Some(DepthTexture::create_depth_stencil_state()),
                    multisample: eframe::wgpu::MultisampleState::default(),
                    multiview: None,
                });

        let indices = [0, 1, 2, 3, 4, 5];
        let index_buffer =
            args.device
                .create_buffer_init(&eframe::wgpu::util::BufferInitDescriptor {
                    label: Some("grid index buffer"),
                    contents: bytemuck::cast_slice(&indices),
                    usage: eframe::wgpu::BufferUsages::INDEX,
                });

        Self {
            pipeline,
            bind_group,
            index_buffer,
            buffer,
        }
    }
}
