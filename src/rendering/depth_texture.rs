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
