use crate::{camera::Camera, vertex::Vertex};

use super::{
    depth_texture::DepthTexture,
    draw_command::{DrawCommand, DrawCommandKind},
    drawing_stuff::DrawingStuff,
    grid::{GridRenderResources, GridRenderResourcesInitializeArgs},
    viewport::{ViewportRenderResources, ViewportUniform},
};

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
        camera_uniform: &ViewportUniform,
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

        let grid = GridRenderResources::initialize(GridRenderResourcesInitializeArgs {
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
    pub fn prepare(
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
            let draw_command = DrawCommand::from_meshes(device, &drawing_stuff.meshes_to_draw);
            self.draw_commands.push(draw_command);
        }

        // TODO
        // if renderer.drawing_region_resized {
        //     renderer.depth_texture = DepthTexture::new(device, renderer.drawing_region_size);
        // }
    }

    /// This is called after `prepare()` when eframe gives us the render pass. This is where we do the
    /// draw calls.
    pub fn paint<'rp>(&'rp self, render_pass: &mut eframe::wgpu::RenderPass<'rp>) {
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
