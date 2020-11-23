use glam::Vec4;
use miniquad::{Context, Texture};

#[derive(Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Color(Vec4);

impl Color {
    pub fn as_u8(&self) -> [u8; 4] {
        [
            (self.0.x * 255.0).max(0.0).min(255.0) as u8,
            (self.0.y * 255.0).max(0.0).min(255.0) as u8,
            (self.0.z * 255.0).max(0.0).min(255.0) as u8,
            (self.0.w * 255.0).max(0.0).min(255.0) as u8,
        ]
    }

    pub fn light_gray() -> Color {
        (200, 200, 200, 255).into()
    }

    pub fn gray() -> Color {
        (130, 130, 130, 255).into()
    }

    pub fn dark_gray() -> Color {
        (80, 80, 80, 255).into()
    }
    pub fn yellow() -> Color {
        (253, 249, 0, 255).into()
    }

    pub fn gold() -> Color {
        (255, 203, 0, 255).into()
    }

    pub fn pink() -> Color {
        (255, 109, 194, 255).into()
    }
    pub fn red() -> Color {
        (250, 41, 55, 255).into()
    }

    pub fn maroon() -> Color {
        (190, 33, 55, 255).into()
    }

    pub fn green() -> Color {
        (0, 228, 48, 255).into()
    }

    pub fn lime() -> Color {
        (0, 158, 47, 255).into()
    }

    pub fn dark_green() -> Color {
        (0, 117, 44, 255).into()
    }

    pub fn sky_blue() -> Color {
        (102, 191, 255, 255).into()
    }

    pub fn blue() -> Color {
        (0, 121, 241, 255).into()
    }

    pub fn purple() -> Color {
        (200, 122, 255, 255).into()
    }

    pub fn violet() -> Color {
        (135, 60, 190, 255).into()
    }

    pub fn dark_purple() -> Color {
        (0, 82, 172, 255).into()
    }

    pub fn beige() -> Color {
        (211, 176, 131, 255).into()
    }

    pub fn brown() -> Color {
        (127, 106, 79, 255).into()
    }

    pub fn dark_brown() -> Color {
        (76, 63, 47, 255).into()
    }

    pub fn white() -> Color {
        (255, 255, 255, 255).into()
    }

    pub fn black() -> Color {
        (0, 0, 0, 255).into()
    }

    pub fn magenta() -> Color {
        (255, 0, 255, 255).into()
    }

    pub fn ray_white() -> Color {
        (245, 245, 245, 255).into()
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(v: (u8, u8, u8, u8)) -> Self {
        Self(Vec4::new(
            v.0 as f32 / 255.0,
            v.1 as f32 / 255.0,
            v.2 as f32 / 255.0,
            v.3 as f32 / 255.0,
        ))
    }
}

impl Into<(u8, u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8, u8) {
        (
            (self.0.x * 255.0).max(0.0).min(255.0) as u8,
            (self.0.y * 255.0).max(0.0).min(255.0) as u8,
            (self.0.z * 255.0).max(0.0).min(255.0) as u8,
            (self.0.w * 255.0).max(0.0).min(255.0) as u8,
        )
    }
}

impl Into<(f32, f32, f32, f32)> for Color {
    fn into(self) -> (f32, f32, f32, f32) {
        (self.0.x, self.0.y, self.0.z, self.0.w)
    }
}

pub fn build_square_texture<T: Into<Color>>(ctx: &mut Context, width: u16, color: T) -> Texture {
    let color: Color = color.into();
    let mut out: Vec<u8> = Vec::with_capacity((width * width) as usize);
    for _i in 0..(width * width) {
        let c = color.as_u8();
        for byte in color.as_u8().iter() {
            out.push(*byte);
        }
    }
    Texture::from_rgba8(ctx, width, width, out.as_slice())
}
