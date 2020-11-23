use glam::{Vec2, Vec4};
use miniquad::{Buffer, BufferType, Context, Texture};

use crate::shaders::Vertex;

#[derive(Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Color(Vec4);

impl Color {
    pub fn as_u8(&self) -> [u8; 4] {
        [
            (self.0.x * 255.0f32).max(0.0).min(255.0) as u8,
            (self.0.y * 255.0f32).max(0.0).min(255.0) as u8,
            (self.0.z * 255.0f32).max(0.0).min(255.0) as u8,
            (self.0.w * 255.0f32).max(0.0).min(255.0) as u8,
        ]
    }

    #[allow(dead_code)]
    pub fn light_gray() -> Color {
        (200, 200, 200, 255).into()
    }

    #[allow(dead_code)]
    pub fn gray() -> Color {
        (130, 130, 130, 255).into()
    }

    #[allow(dead_code)]
    pub fn dark_gray() -> Color {
        (80, 80, 80, 255).into()
    }

    #[allow(dead_code)]
    pub fn yellow() -> Color {
        (253, 249, 0, 255).into()
    }

    #[allow(dead_code)]
    pub fn gold() -> Color {
        (255, 203, 0, 255).into()
    }

    #[allow(dead_code)]
    pub fn pink() -> Color {
        (255, 109, 194, 255).into()
    }

    #[allow(dead_code)]
    pub fn red() -> Color {
        (250, 41, 55, 255).into()
    }

    #[allow(dead_code)]
    pub fn maroon() -> Color {
        (190, 33, 55, 255).into()
    }

    #[allow(dead_code)]
    pub fn green() -> Color {
        (0, 228, 48, 255).into()
    }

    #[allow(dead_code)]
    pub fn lime() -> Color {
        (0, 158, 47, 255).into()
    }

    #[allow(dead_code)]
    pub fn dark_green() -> Color {
        (0, 117, 44, 255).into()
    }

    #[allow(dead_code)]
    pub fn sky_blue() -> Color {
        (102, 191, 255, 255).into()
    }

    #[allow(dead_code)]
    pub fn blue() -> Color {
        (0, 121, 241, 255).into()
    }

    #[allow(dead_code)]
    pub fn purple() -> Color {
        (200, 122, 255, 255).into()
    }

    #[allow(dead_code)]
    pub fn violet() -> Color {
        (135, 60, 190, 255).into()
    }

    #[allow(dead_code)]
    pub fn dark_purple() -> Color {
        (0, 82, 172, 255).into()
    }

    #[allow(dead_code)]
    pub fn beige() -> Color {
        (211, 176, 131, 255).into()
    }

    #[allow(dead_code)]
    pub fn brown() -> Color {
        (127, 106, 79, 255).into()
    }

    #[allow(dead_code)]
    pub fn dark_brown() -> Color {
        (76, 63, 47, 255).into()
    }

    #[allow(dead_code)]
    pub fn white() -> Color {
        (255, 255, 255, 255).into()
    }

    #[allow(dead_code)]
    pub fn black() -> Color {
        (0, 0, 0, 255).into()
    }

    #[allow(dead_code)]
    pub fn magenta() -> Color {
        (255, 0, 255, 255).into()
    }

    #[allow(dead_code)]
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
        let x = (self.0.x * 255.0f32).max(0.0).min(255.0) as u8;
        let y = (self.0.y * 255.0f32).max(0.0).min(255.0) as u8;
        let z = (self.0.z * 255.0f32).max(0.0).min(255.0) as u8;
        let w = (self.0.w * 255.0f32).max(0.0).min(255.0) as u8;
        (x, y, z, w)
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
        for byte in color.as_u8().iter() {
            out.push(*byte);
        }
    }
    Texture::from_rgba8(ctx, width, width, out.as_slice())
}

pub fn make_square(ctx: &mut Context, size: f32) -> (Buffer, Buffer) {
    let vertices = [
        Vertex {
            pos: Vec2::new(-size / 2., -size / 2.),
            uv: Vec2::new(0., 0.),
        },
        Vertex {
            pos: Vec2::new(size / 2., -size / 2.),
            uv: Vec2::new(1., 0.),
        },
        Vertex {
            pos: Vec2::new(size / 2., size / 2.),
            uv: Vec2::new(1., 1.),
        },
        Vertex {
            pos: Vec2::new(-size / 2., size / 2.),
            uv: Vec2::new(0., 1.),
        },
    ];
    let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

    let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);
    let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);
    (vertex_buffer, index_buffer)
}
