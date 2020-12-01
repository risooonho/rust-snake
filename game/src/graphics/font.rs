use crate::graphics::{colors, Color};

// TODO: This can replace some of color image generation functions
pub struct CpuImage {
    pub bytes: Vec<u8>,
    pub width: u16,
    pub height: u16,
}

impl core::fmt::Debug for CpuImage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        f.debug_struct("CpuImage")
            .field("bytes", &"Vec<u8>")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
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
    fn get_image_data_mut(&mut self) -> &mut [[u8; 4]] {
        use core::slice;

        unsafe {
            slice::from_raw_parts_mut(
                self.bytes.as_mut_ptr() as *mut [u8; 4],
                self.width as usize * self.height as usize,
            )
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let width = self.width as usize;

        self.get_image_data_mut()[y * width + x] = color.into();
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CharInfo {
    pub offset_x: i32,
    pub offset_y: i32,
    pub advance: f32,

    pub glyph_x: u32,
    pub glyph_y: u32,
    pub glyph_w: u32,
    pub glyph_h: u32,
}

pub fn ascii_character_list() -> Vec<char> {
    (0..255).filter_map(::core::char::from_u32).collect()
}

// TODO(jhurstwright): I kind of don't think I need to store the text in main memory, I think I can just load and unload it in bulk from gpu later
pub struct Font {
    pub name: String,
    pub font: fontdue::Font,
    pub font_image: CpuImage,
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub max_line_height: u16,

    pub glyphs: std::collections::HashMap<char, CharInfo>,
}

impl core::fmt::Debug for Font {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        f.debug_struct("Font")
            .field("font", &"fontdue::Font")
            .field("font_image", &self.font_image)
            .field("cursor_x", &self.cursor_x)
            .field("cursor_y", &self.cursor_y)
            .finish()
    }
}

impl Font {
    const GAP: u16 = 16;
    pub fn load<T: Into<String>>(name: T, bytes: &[u8]) -> Self {
        let font = fontdue::Font::from_bytes(&bytes[..], fontdue::FontSettings::default()).unwrap();
        let font_image = CpuImage::gen_image_color(1024, 1024, colors::CLEAR);
        Self {
            name: name.into(),
            font,
            font_image,
            cursor_x: 0,
            cursor_y: 0,
            max_line_height: 0,
            glyphs: std::collections::HashMap::new(),
        }
    }

    pub fn image_dimensions(&self) -> (u16, u16) {
        (self.font_image.width, self.font_image.height)
    }

    pub fn cache_glyph(&mut self, character: char) {
        let (metrics, bitmap) = self.font.rasterize(character, 48.);
        if metrics.advance_height != 0.0 {
            panic!("Vertical fonts are not yet supported");
        }

        let (width, height) = (metrics.width as u16, metrics.height as u16);
        let advance = metrics.advance_width;
        let (offset_x, offset_y) = (metrics.xmin, metrics.ymin);
        let x = if self.cursor_x + (Font::GAP + width as u16) < self.font_image.width {
            if height as u16 > self.max_line_height {
                self.max_line_height = height;
            }
            let res = self.cursor_x;
            self.cursor_x += width;
            res
        } else {
            self.cursor_y += self.max_line_height + Self::GAP;
            self.cursor_x = width + Self::GAP;
            self.max_line_height = height;
            Self::GAP
        };
        let y = self.cursor_y;

        let character_info = CharInfo {
            offset_x,
            offset_y,
            advance,
            glyph_x: x as u32,
            glyph_y: y as u32,
            glyph_w: width as u32,
            glyph_h: height as u32,

        };

        self.glyphs.insert(character, character_info);

        if self.cursor_y + height as u16 > self.font_image.height {
            panic!("Does not yet support Render Text expansion");
        } else {
            for j in 0..height {
                for i in 0..width {
                    let coverage = bitmap[(j * width + i) as usize] as f32 / 255.0;
                    self.font_image
                        .set_pixel((x + i) as usize, (y + j) as usize, Color::new(1., 1., 1., coverage))
                }
            }
        }
    }

    pub fn texture(&self, ctx: &mut miniquad::Context) -> miniquad::Texture {
        miniquad::Texture::from_data_and_format(
            ctx,
            self.font_image.bytes.as_slice(),
            miniquad::TextureParams {
                format: miniquad::TextureFormat::RGBA8,
                wrap: miniquad::TextureWrap::Clamp,
                filter: miniquad::FilterMode::Nearest,
                width: self.font_image.width as u32,
                height: self.font_image.height as u32,

            }
        )
    }
}
