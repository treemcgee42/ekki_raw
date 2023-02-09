use std::rc::Rc;
use wgpu::{
    util::DeviceExt, CommandEncoder, RenderPassDescriptor, RenderPipelineDescriptor, TextureView,
};
use winit::{event::WindowEvent, window::Window};

use crate::camera::{Camera, CameraUniform};
use crate::draw::{DrawCommand, DrawCommandKind};
use crate::vertex::Vertex;
use crate::ApplicationState;

/// This encapsulates GPU-specific functionality.
pub struct Renderer {
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    pub to_draw: Vec<DrawCommand>,
    camera_info: CameraInfo,
}

impl Renderer {
    pub async fn initialize(window: Rc<Window>, camera: &Camera) -> Self {
        let size = window.inner_size();

        // Create instance --> create surface & adapter
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window.as_ref()) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // Use adapter to create device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        // Configure surface
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &surface_config);

        // Render pipeline
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // Camera
        let camera_info = CameraInfo::initialize(&device, camera);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera_info.bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::get_descriptor()],
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        Self {
            surface,
            surface_config,
            device,
            queue,
            render_pipeline,
            to_draw: Vec::new(),
            camera_info,
        }
    }

    pub fn resize(&mut self, camera: &mut Camera, new_size: winit::dpi::PhysicalSize<u32>) {
        if !(new_size.width > 0 && new_size.height > 0) {
            return;
        }

        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        self.surface.configure(&self.device, &self.surface_config);

        camera.handle_window_resize(new_size.width as f32, new_size.height as f32);
    }

    pub fn event_was_processed(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self, camera: &Camera) {
        self.camera_info
            .uniform
            .update_view_projection_matrix(&camera);
        self.queue.write_buffer(
            &self.camera_info.buffer,
            0,
            bytemuck::cast_slice(&[self.camera_info.uniform]),
        );
    }

    fn begin_render_pass(
        &mut self,
        command_encoder: &mut CommandEncoder,
        texture_view: &TextureView,
    ) {
        let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[
                // @location(0) target in fragment shader
                Some(wgpu::RenderPassColorAttachment {
                    view: texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }),
            ],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.camera_info.bind_group, &[]);

        for command in &self.to_draw {
            render_pass.set_vertex_buffer(0, command.wgpu_mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                command.wgpu_mesh.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );

            match command.kind {
                DrawCommandKind::DrawIndexedAll => {
                    render_pass.draw_indexed(0..command.wgpu_mesh.num_indices, 0, 0..1);
                }
            }
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // Get frame to render to.
        let output = self.surface.get_current_texture()?;

        let texture_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut command_encoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
        self.begin_render_pass(&mut command_encoder, &texture_view);

        self.queue.submit(std::iter::once(command_encoder.finish()));
        output.present();

        Ok(())
    }
}

struct CameraInfo {
    uniform: CameraUniform,
    buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}

impl CameraInfo {
    fn initialize(device: &wgpu::Device, camera: &Camera) -> Self {
        let uniform = CameraUniform::new(camera);

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("bind_group_layout"),
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        Self {
            uniform,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }
}
