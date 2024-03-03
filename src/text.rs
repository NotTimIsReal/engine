use glyphon::{
    Attrs, Buffer, Color, Family, FontSystem, Metrics, Resolution, Shaping, SwashCache, TextArea,
    TextAtlas, TextBounds, TextRenderer,
};
use wgpu::{MultisampleState, TextureFormat};

use crate::textures::Texture;
pub struct TextEngine {
    buffers: Vec<Buffer>,
    font_system: FontSystem,
    cache: SwashCache,
}
impl TextEngine {
    pub fn new() -> Self {
        Self {
            buffers: vec![],

            font_system: FontSystem::new(),

            cache: SwashCache::new(),
        }
    }
    pub fn add_buffer(&mut self, font_size: f32, line_height: f32) -> i32 {
        self.buffers.push(Buffer::new(
            &mut self.font_system,
            Metrics::new(font_size, line_height),
        ));
        self.buffers.len() as i32 - 1
    }
    pub fn set_text(
        &mut self,
        text: &str,
        colour: [u8; 4],
        scale_factor: f64,
        config: &wgpu::SurfaceConfiguration,
        id: i32,
    ) {
        let buffer = &mut self.buffers[id as usize];
        buffer.set_size(
            &mut self.font_system,
            (config.width as f64 * scale_factor) as f32,
            (config.height as f64 * scale_factor) as f32,
        );
        let render_time = std::time::Instant::now();
        let attrs = Attrs::new()
            .color(Color::rgba(colour[0], colour[1], colour[2], colour[3]))
            .family(Family::SansSerif);

        buffer.set_text(&mut self.font_system, text, attrs, Shaping::Advanced);
    }
    pub fn render<'a, 'b, 'c>(
        &mut self,
        id: i32,
        renderer: &'a mut TextRenderer,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        mut atlas: &'b mut TextAtlas,
        config: &wgpu::SurfaceConfiguration,
        mut pass: &'c mut wgpu::RenderPass<'a>,
    ) where
        'b: 'a,
    {
        {
            let mut atlas = &mut atlas;
            renderer
                .prepare(
                    device,
                    queue,
                    &mut self.font_system,
                    &mut atlas,
                    Resolution {
                        width: config.width,
                        height: config.height,
                    },
                    [TextArea {
                        buffer: &self.buffers[id as usize],
                        left: 10.0,
                        top: 10.0,
                        scale: 1.0,
                        bounds: TextBounds {
                            left: 0,
                            top: 0,
                            right: 600,
                            bottom: 160,
                        },
                        default_color: Color::rgb(255, 255, 255),
                    }],
                    &mut self.cache,
                )
                .unwrap();
        }
        renderer.render(atlas, &mut pass).unwrap();
    }
}
