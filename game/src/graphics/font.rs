use crate::graphics::{colors, Color};

// TODO: This can replace some of color image generation functions
struct CpuImage {
    pub bytes: Vec<u8>,
    pub width: u16,
    pub height: u16,
}

impl CpuImage {
    pub fn gen_image_color(width: u16, height: u16, color: Color) -> Self {
        let mut bytes = vec![0; width as usize * height as usize * 4];
        for i in 0..width as usize * height as usize {
            let c = color.as_u8();
            bytes[i * 4 + 0] = c[0];
            bytes[i * 4 + 1] = c[1];
            bytes[i * 4 + 2] = c[2];
            bytes[i * 4 + 3] = c[3];
        }

        Self {
            bytes,
            width,
            height,
        }
    }
}
struct CharInfo {
    offset_x: i32,
    offset_y: i32,
    advance: f32,

    glyph_x: u32,
    glyph_y: u32,
    glyph_w: u32,
    glyph_h: u32,
}

// TODO(jhurstwright): I kind of don't think I need to store the text in main memory, I think I can just load and unload it in bulk from gpu later
pub struct Font {
    font: fontdue::Font,
    font_image: CpuImage,
}

impl Font {
    pub fn load(bytes: &[u8]) -> Self {
        let font = fontdue::Font::from_bytes(&bytes[..], fontdue::FontSettings::default()).unwrap();
        let font_image = CpuImage::gen_image_color(512, 512, colors::CLEAR);
        Self { font, font_image }
    }
}
