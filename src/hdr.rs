use wgpu::Operations;

use crate::{
    renderer::create_render_pipeline,
    textures::{self, Texture},
};
pub struct HdrPipeline {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    texture: textures::Texture,
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    layout: wgpu::BindGroupLayout,
}
impl HdrPipeline {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let width = config.width;
        let height = config.height;
        let format = wgpu::TextureFormat::Rgba16Float;
        let texture = Texture::create_2d_texture(
            device,
            Some("HDR Texture Creation"),
            width,
            height,
            format,
            wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
            wgpu::FilterMode::Linear,
        );
        // Hdr Bind group layout
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("HDR Pipeline Bind Group Layout"),
            entries: &[
                //Literal Texture colour information
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // Texture Sampling Data
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("HDR Pipeline Bind Group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        });
        let shader = wgpu::include_wgsl!("../shaders/hdr.wgsl");
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("HDR Pipeline Layout"),
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });
        let pipeline = create_render_pipeline(
            device,
            &pipeline_layout,
            config.format,
            None,
            &[],
            shader,
            wgpu::PrimitiveTopology::TriangleList,
            "HDR Pipeline",
        );
        Self {
            pipeline,
            bind_group,
            layout,
            texture,
            width,
            height,
            format,
        }
    }
    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.texture = Texture::create_2d_texture(
            device,
            Some("HDR Texture Creation"),
            width,
            height,
            self.format,
            wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
            wgpu::FilterMode::Linear,
        );
        self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("HDR Pipeline Bind Group"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.texture.sampler),
                },
            ],
        });
        self.width = width;
        self.height = height;
    }
    pub fn view(&self) -> &wgpu::TextureView {
        &self.texture.view
    }
    pub fn format(&self) -> wgpu::TextureFormat {
        self.format
    }
    pub fn process(&self, encoder: &mut wgpu::CommandEncoder, output: &wgpu::TextureView) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("HDR Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: output,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            ..Default::default()
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.draw(0..3, 0..1);
    }
}
