use crate::input_state;
use crate::meshes::Mesh;
use eframe::wgpu::util::DeviceExt;
use std::sync::Arc;

use crate::camera::{Camera, CameraUniform};
use crate::draw::{draw_meshes, DrawCommand, DrawCommandKind};
use crate::grid::{GridInitializeArgs, GridRenderResources, GridUniform};
use crate::math::vector::Vector2;
use crate::meshes::MeshBank;
use crate::vertex::Vertex;

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

        // TODO: take user input and update camera accordingly
        ui.input(|i| self.handle_shortcut_viewport_camera_rotate(i));

        if self.doing_turntable {
            let response = ui.interact(rect, id, eframe::egui::Sense::drag());
            let p_start = ui.input(|i| i.pointer.press_origin());
            if let Some(start) = p_start {
                let end = ui.input(|i| i.pointer.hover_pos()).unwrap();
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

#[derive(Clone)]
/// This encapsulates things that relate to drawing but are still CPU-specific.
pub struct DrawingStuff {
    pub meshes_to_draw: Vec<Arc<Mesh>>,
    pub drawing_region_size: Vector2,
    pub drawing_region_size_updated: bool,
    pub camera_uniform: CameraUniform,
    pub grid_uniform: GridUniform,
    // depth_texture: DepthTexture,
}

impl DrawingStuff {
    fn initialize(camera: &Camera) -> Self {
        DrawingStuff {
            meshes_to_draw: Vec::new(),
            drawing_region_size: Vector2::new(0.0, 0.0),
            drawing_region_size_updated: true,
            camera_uniform: CameraUniform::new(camera),
            grid_uniform: GridUniform::new(camera),
        }
    }
}

/// These are the things that eframe will give us an *immutable* reference to each frame. The only things
/// that should go here are those whose lifetime needs to be the same as the egui render pass.
pub struct RenderResources {
    render_pipeline: eframe::wgpu::RenderPipeline,
    camera_info: ViewportRenderResources,
    grid: GridRenderResources,
    draw_commands: Vec<DrawCommand>,
    depth_texture: DepthTexture,
    // TODO: store vertex/index buffers for reuse
}

impl RenderResources {
    /// Creates all the render resources and gives them to eframe.
    ///
    /// - `camera`: needed to initialize the grid uniform buffer (TODO: pass grid_uniform)
    /// - `camera_uniform`: needed so that we can initialize the corresponding uniform buffer.
    pub fn initialize<'a>(
        eframe_creation_context: &'a eframe::CreationContext<'a>,
        camera: &Camera,
        camera_uniform: &CameraUniform,
    ) {
        // Get the WGPU render state from the eframe creation context. This can also be retrieved
        // from `eframe::Frame` when you don't have a `CreationContext` available.
        let wgpu_render_state = eframe_creation_context.wgpu_render_state.as_ref().unwrap();

        let device = &wgpu_render_state.device;
        let window_size = eframe_creation_context.integration_info.window_info.size;
        let surface_format = wgpu_render_state.target_format;

        // Render pipeline
        let shader = device.create_shader_module(eframe::wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: eframe::wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // Camera
        let camera_info = ViewportRenderResources::initialize(&device, camera_uniform);

        // Depth buffer
        let depth_texture = DepthTexture::new(&device, &window_size);

        // Main render pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&eframe::wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera_info.bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline =
            device.create_render_pipeline(&eframe::wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: eframe::wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::get_descriptor()],
                },
                fragment: Some(eframe::wgpu::FragmentState {
                    // 3.
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(eframe::wgpu::ColorTargetState {
                        // 4.
                        format: surface_format, //surface_config.format,
                        blend: Some(eframe::wgpu::BlendState::REPLACE),
                        write_mask: eframe::wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: eframe::wgpu::PrimitiveState {
                    topology: eframe::wgpu::PrimitiveTopology::TriangleList, // 1.
                    strip_index_format: None,
                    front_face: eframe::wgpu::FrontFace::Ccw, // 2.
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
                multiview: None, // 5.
            });

        let grid = GridRenderResources::initialize(GridInitializeArgs {
            device: &device,
            surface_format: surface_format.clone(),
            view_projection_matrix: camera.get_view_projection_matrix(),
            camera: &camera,
        });

        // Because the graphics pipeline must have the same lifetime as the egui render pass,
        // instead of storing the pipeline in our `Custom3D` struct, we insert it into the
        // `paint_callback_resources` type map, which is stored alongside the render pass.
        wgpu_render_state
            .renderer
            .write()
            .paint_callback_resources
            .insert(RenderResources {
                render_pipeline,
                camera_info,
                grid,
                draw_commands: Vec::new(),
                depth_texture,
            });
    }

    /// The app calls this function after eframe gives it an immutable reference to this struct (it does
    /// this every frame). This is called before painting/drawing, and is supposed to be for updating
    /// things such as buffers and uniforms before rendering.
    fn prepare(
        &mut self,
        device: &eframe::wgpu::Device,
        queue: &eframe::wgpu::Queue,
        drawing_stuff: DrawingStuff,
    ) {
        // Update the uniform with the view projection matrix. The CPU-side version (`CameraUniform`)
        // is assumed to already have been updated. We just need to copy that into the GPU uniform.
        queue.write_buffer(
            &self.camera_info.buffer,
            0,
            bytemuck::cast_slice(&[drawing_stuff.camera_uniform]),
        );

        // The grid also needs the latest view projection matrix. The CPU-side version (`GridUniform`)
        // is assumed to already have been updated...
        queue.write_buffer(
            &self.grid.buffer,
            0,
            bytemuck::cast_slice(&[drawing_stuff.grid_uniform]),
        );

        if drawing_stuff.meshes_to_draw.len() > 0 {
            let draw_command = draw_meshes(device, &drawing_stuff.meshes_to_draw);
            self.draw_commands.push(draw_command);
        }

        // TODO
        // if renderer.drawing_region_resized {
        //     renderer.depth_texture = DepthTexture::new(device, renderer.drawing_region_size);
        // }
    }

    /// This is called after `prepare()` when eframe gives us the render pass. This is where we do the
    /// draw calls.
    fn paint<'rp>(&'rp self, render_pass: &mut eframe::wgpu::RenderPass<'rp>) {
        // SHAPES : i.e. everything in the draw list maintained by the app
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.camera_info.bind_group, &[]);

        for command in &self.draw_commands {
            render_pass.set_vertex_buffer(0, command.wgpu_mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                command.wgpu_mesh.index_buffer.slice(..),
                eframe::wgpu::IndexFormat::Uint16,
            );

            match command.kind {
                DrawCommandKind::DrawIndexedAll => {
                    render_pass.draw_indexed(0..command.wgpu_mesh.num_indices, 0, 0..1);
                }
            }
        }

        // GRID
        render_pass.set_pipeline(&self.grid.pipeline);
        render_pass.set_bind_group(0, &self.grid.bind_group, &[]);
        render_pass.set_index_buffer(
            self.grid.index_buffer.slice(..),
            eframe::wgpu::IndexFormat::Uint32,
        );
        render_pass.draw_indexed(0..6, 0, 0..1);
    }
}

// impl Renderer {
//     pub async fn initialize<'a>(
//         window: Rc<Window>,
//         camera: &Camera,
//         eframe_creation_context: &'a eframe::CreationContext<'a>,
//     ) -> Self {
//         // Get the WGPU render state from the eframe creation context. This can also be retrieved
//         // from `eframe::Frame` when you don't have a `CreationContext` available.
//         let wgpu_render_state = eframe_creation_context.wgpu_render_state.as_ref().unwrap();
//
//         let device = &wgpu_render_state.device;
//         let window_size = eframe_creation_context.integration_info.window_info.size;
//         let surface_format = wgpu_render_state.target_format;
//
//         // Render pipeline
//         let shader = device.create_shader_module(eframe::wgpu::ShaderModuleDescriptor {
//             label: Some("Shader"),
//             source: eframe::wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
//         });
//
//         // Camera
//         let camera_info = CameraInfo::initialize(&device, camera);
//
//         // Depth buffer
//         let depth_texture = DepthTexture::new(&device, &window_size);
//
//         let render_pipeline_layout =
//             device.create_pipeline_layout(&eframe::wgpu::PipelineLayoutDescriptor {
//                 label: Some("Render Pipeline Layout"),
//                 bind_group_layouts: &[&camera_info.bind_group_layout],
//                 push_constant_ranges: &[],
//             });
//
//         let render_pipeline =
//             device.create_render_pipeline(&eframe::wgpu::RenderPipelineDescriptor {
//                 label: Some("Render Pipeline"),
//                 layout: Some(&render_pipeline_layout),
//                 vertex: eframe::wgpu::VertexState {
//                     module: &shader,
//                     entry_point: "vs_main",
//                     buffers: &[Vertex::get_descriptor()],
//                 },
//                 fragment: Some(eframe::wgpu::FragmentState {
//                     // 3.
//                     module: &shader,
//                     entry_point: "fs_main",
//                     targets: &[Some(eframe::wgpu::ColorTargetState {
//                         // 4.
//                         format: surface_format, //surface_config.format,
//                         blend: Some(eframe::wgpu::BlendState::REPLACE),
//                         write_mask: eframe::wgpu::ColorWrites::ALL,
//                     })],
//                 }),
//                 primitive: eframe::wgpu::PrimitiveState {
//                     topology: eframe::wgpu::PrimitiveTopology::TriangleList, // 1.
//                     strip_index_format: None,
//                     front_face: eframe::wgpu::FrontFace::Ccw, // 2.
//                     cull_mode: Some(eframe::wgpu::Face::Back),
//                     // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
//                     polygon_mode: eframe::wgpu::PolygonMode::Fill,
//                     // Requires Features::DEPTH_CLIP_CONTROL
//                     unclipped_depth: false,
//                     // Requires Features::CONSERVATIVE_RASTERIZATION
//                     conservative: false,
//                 },
//                 depth_stencil: Some(DepthTexture::create_depth_stencil_state()),
//                 multisample: eframe::wgpu::MultisampleState {
//                     count: 1,                         // 2.
//                     mask: !0,                         // 3.
//                     alpha_to_coverage_enabled: false, // 4.
//                 },
//                 multiview: None, // 5.
//             });
//
//         let grid = Grid::initialize(GridInitializeArgs {
//             device: &device,
//             surface_format: surface_format.clone(),
//             view_projection_matrix: camera.get_view_projection_matrix(),
//             camera: &camera,
//         });
//
//         // Because the graphics pipeline must have the same lifetime as the egui render pass,
//         // instead of storing the pipeline in our `Custom3D` struct, we insert it into the
//         // `paint_callback_resources` type map, which is stored alongside the render pass.
//         wgpu_render_state
//             .renderer
//             .write()
//             .paint_callback_resources
//             .insert(RenderResources {
//                 render_pipeline,
//                 camera_info,
//                 grid,
//                 depth_texture,
//             });
//
//         Self {
//             to_draw: Vec::new(),
//             drawing_region_size: Vector2::new(0.0, 0.0),
//             drawing_region_size_updated: false,
//         }
//     }
//
//     /// This is where everything on the CPU side should be updated. Updating will be done in the
//     /// `prepare()` function once we get the render resources from eframe. This function will call
//     /// that as well, after having updated everything CPU side.
//     fn custom_painting(&mut self, ui: &mut eframe::egui::Ui) {
//         let (_, rect) = ui.allocate_space(ui.available_size());
//
//         let rect_size = Vector2::from(rect.size());
//         if !Vector2::are_approximately_equal(&rect_size, &self.drawing_region_size) {
//             self.drawing_region_size = rect_size;
//             self.drawing_region_size_updated = true;
//         }
//
//         if self.drawing_region_size_updated {
//             // update camera width/height and update depth texture
//             self.camera
//                 .handle_window_resize(self.drawing_region_size.x(), self.drawing_region_size.y());
//         }
//
//         // TODO: take user input and update camera accordingly
//
//         // Handle GPU side things
//         let cb = eframe::egui_wgpu::CallbackFn::new()
//             .prepare(move |device, queue, _encoder, paint_callback_resources| {
//                 let resources: &RenderResources = paint_callback_resources.get().unwrap();
//                 resources.prepare(device, queue, &mut self);
//                 Vec::new()
//             })
//             .paint(move |_info, render_pass, paint_callback_resources| {
//                 let resources: &RenderResources = paint_callback_resources.get().unwrap();
//                 resources.paint(render_pass, &self);
//             });
//
//         let callback = eframe::egui::PaintCallback {
//             rect,
//             callback: Arc::new(cb),
//         };
//
//         ui.painter().add(callback);
//     }
// }

//    pub fn resize(&mut self, camera: &mut Camera, new_size: winit::dpi::PhysicalSize<u32>) {
//        if !(new_size.width > 0 && new_size.height > 0) {
//            return;
//        }
//
//        self.surface_config.width = new_size.width;
//        self.surface_config.height = new_size.height;
//        self.surface.configure(&self.device, &self.surface_config);
//
//        self.depth_texture = DepthTexture::new(&self.device, &self.surface_config);
//
//        camera.handle_window_resize(new_size.width as f32, new_size.height as f32);
//    }
//
//    pub fn update(&mut self, camera: &Camera) {
//        self.camera_info
//            .uniform
//            .update_view_projection_matrix(&camera);
//        self.queue.write_buffer(
//            &self.camera_info.buffer,
//            0,
//            bytemuck::cast_slice(&[self.camera_info.uniform]),
//        );
//
//        self.grid.uniform.update_matrix(camera);
//        self.queue.write_buffer(
//            &self.grid.buffer,
//            0,
//            bytemuck::cast_slice(&[self.grid.uniform]),
//        );
//    }
//
//    fn begin_render_pass(
//        &mut self,
//        command_encoder: &mut CommandEncoder,
//        texture_view: &TextureView,
//    ) {
//        let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//            label: Some("Render Pass"),
//            color_attachments: &[
//                // @location(0) target in fragment shader
//                Some(wgpu::RenderPassColorAttachment {
//                    view: texture_view,
//                    resolve_target: None,
//                    ops: wgpu::Operations {
//                        load: wgpu::LoadOp::Clear(wgpu::Color {
//                            r: 0.1,
//                            g: 0.2,
//                            b: 0.3,
//                            a: 1.0,
//                        }),
//                        store: true,
//                    },
//                }),
//            ],
//            depth_stencil_attachment: Some(self.depth_texture.create_depth_stencil_attachment()),
//        });
//
//        // SHAPES
//        render_pass.set_pipeline(&self.render_pipeline);
//        render_pass.set_bind_group(0, &self.camera_info.bind_group, &[]);
//
//        for command in &self.to_draw {
//            render_pass.set_vertex_buffer(0, command.wgpu_mesh.vertex_buffer.slice(..));
//            render_pass.set_index_buffer(
//                command.wgpu_mesh.index_buffer.slice(..),
//                wgpu::IndexFormat::Uint16,
//            );
//
//            match command.kind {
//                DrawCommandKind::DrawIndexedAll => {
//                    render_pass.draw_indexed(0..command.wgpu_mesh.num_indices, 0, 0..1);
//                }
//            }
//        }
//
//        // GRID
//        render_pass.set_pipeline(&self.grid.pipeline);
//        render_pass.set_bind_group(0, &self.grid.bind_group, &[]);
//        render_pass.set_index_buffer(self.grid.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
//        render_pass.draw_indexed(0..6, 0, 0..1);
//    }
//
//    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
//        // Get frame to render to.
//        let output = self.surface.get_current_texture()?;
//
//        let texture_view = output
//            .texture
//            .create_view(&wgpu::TextureViewDescriptor::default());
//        let mut command_encoder =
//            self.device
//                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
//                    label: Some("Render Encoder"),
//                });
//        self.begin_render_pass(&mut command_encoder, &texture_view);
//
//        self.queue.submit(std::iter::once(command_encoder.finish()));
//        output.present();
//
//        Ok(())
//    }
//}

/// Rendering resources related to applying the proper perspective to the scene. Does not include
/// buffers for the actual objects in the scene, pretty much just the view projection matrix one.
struct ViewportRenderResources {
    buffer: eframe::wgpu::Buffer,
    bind_group_layout: eframe::wgpu::BindGroupLayout,
    bind_group: eframe::wgpu::BindGroup,
}

impl ViewportRenderResources {
    fn initialize(device: &Arc<eframe::wgpu::Device>, camera_uniform: &CameraUniform) -> Self {
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

pub struct DepthTexture {
    pub texture: eframe::wgpu::Texture,
    pub texture_view: eframe::wgpu::TextureView,
    pub sampler: eframe::wgpu::Sampler,
}

impl DepthTexture {
    const DEPTH_FORMAT: eframe::wgpu::TextureFormat = eframe::wgpu::TextureFormat::Depth32Float;
    const COMPARE_FUNCTION: eframe::wgpu::CompareFunction = eframe::wgpu::CompareFunction::Less;

    pub fn new(device: &eframe::wgpu::Device, window_size: &eframe::egui::Vec2) -> Self {
        let size = eframe::wgpu::Extent3d {
            width: window_size.x as u32,
            height: window_size.y as u32,
            depth_or_array_layers: 1,
        };
        let desc = eframe::wgpu::TextureDescriptor {
            label: Some("depth texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: eframe::wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: eframe::wgpu::TextureUsages::RENDER_ATTACHMENT
                | eframe::wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);

        let texture_view = texture.create_view(&eframe::wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&eframe::wgpu::SamplerDescriptor {
            address_mode_u: eframe::wgpu::AddressMode::ClampToEdge,
            address_mode_v: eframe::wgpu::AddressMode::ClampToEdge,
            address_mode_w: eframe::wgpu::AddressMode::ClampToEdge,
            mag_filter: eframe::wgpu::FilterMode::Linear,
            min_filter: eframe::wgpu::FilterMode::Linear,
            mipmap_filter: eframe::wgpu::FilterMode::Nearest,
            compare: Some(eframe::wgpu::CompareFunction::LessEqual),
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Self {
            texture,
            texture_view,
            sampler,
        }
    }

    pub fn create_depth_stencil_state() -> eframe::wgpu::DepthStencilState {
        eframe::wgpu::DepthStencilState {
            format: Self::DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: eframe::wgpu::CompareFunction::LessEqual,
            stencil: eframe::wgpu::StencilState::default(),
            bias: eframe::wgpu::DepthBiasState::default(),
        }
    }

    pub fn create_depth_stencil_attachment(
        &self,
    ) -> eframe::wgpu::RenderPassDepthStencilAttachment {
        eframe::wgpu::RenderPassDepthStencilAttachment {
            view: &self.texture_view,
            depth_ops: Some(eframe::wgpu::Operations {
                load: eframe::wgpu::LoadOp::Clear(1.0),
                store: true,
            }),
            stencil_ops: None,
        }
    }
}
