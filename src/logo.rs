use eframe::egui::{
    Color32, ColorImage, Context, IconData, TextureHandle, TextureId, TextureOptions,
};
use image::{imageops, GrayImage, Luma};

pub const SIZE: f32 = 34.0;

pub struct SidebarLogo {
    mask: GrayImage,
    texture: Option<TextureHandle>,
}

impl SidebarLogo {
    pub fn new(icon: &IconData) -> Self {
        let mask = GrayImage::from_fn(icon.width, icon.height, |x, y| {
            let offset = 4 * (y * icon.width + x) as usize;
            let rgba = &icon.rgba[offset..offset + 4];
            let luma = 0.2126 * f32::from(rgba[0])
                + 0.7152 * f32::from(rgba[1])
                + 0.0722 * f32::from(rgba[2]);
            // The dark purple tile and its transparent corners sit below 16.
            let coverage = (luma * f32::from(rgba[3]) / 255.0 - 16.0) / (246.0 - 16.0);
            Luma([(coverage.clamp(0.0, 1.0) * 255.0).round() as u8])
        });
        Self {
            mask,
            texture: None,
        }
    }

    pub fn texture(&mut self, ctx: &Context) -> TextureId {
        let side = (SIZE * ctx.pixels_per_point()).round().max(1.0) as u32;
        let size = [side as usize; 2];
        if let Some(texture) = &self.texture {
            if texture.size() == size {
                return texture.id();
            }
        }

        let mask = imageops::resize(&self.mask, side, side, imageops::FilterType::Lanczos3);
        let image = ColorImage {
            size,
            pixels: mask
                .into_raw()
                .into_iter()
                .map(Color32::from_white_alpha)
                .collect(),
        };
        let texture = ctx.load_texture("gmcanvas_sidebar_logo", image, TextureOptions::LINEAR);
        let id = texture.id();
        self.texture = Some(texture);
        id
    }
}
