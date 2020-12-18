use glam::Vec2;
use miniquad::{Buffer, BufferType, Context, Texture};

use crate::types::Color;
use crate::shaders::Vertex;

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

pub fn make_arrow_raw() -> ([Vertex; 5], [u16; 9]) {
    let vertices = [
        Vertex {
            pos: Vec2::new(-0.5 / 2., -0.5 / 2.),
            uv: Vec2::new(0., 0.),
        },
        Vertex {
            pos: Vec2::new(0.5 / 2., -0.5 / 2.),
            uv: Vec2::new(1., 0.),
        },
        Vertex {
            pos: Vec2::new(0.5 / 2., 0.5 / 2.),
            uv: Vec2::new(1., 1.),
        },
        Vertex {
            pos: Vec2::new(-0.5 / 2., 0.5 / 2.),
            uv: Vec2::new(0., 1.),
        },
        Vertex {
            pos: Vec2::new(0.5, 0.),
            uv: Vec2::new(0., 1.),
        },
    ];
    let indices: [u16; 9] = [0, 1, 2, 0, 2, 3, 1, 2, 4];
    (vertices, indices)
}

pub fn make_square_raw(size: f32) -> ([Vertex; 4], [u16; 6]) {
    make_rectangle_raw(size / 2., size / 2.)
}

pub fn make_square(ctx: &mut Context, size: f32) -> (Buffer, Buffer, u16) {
    make_rectangle(ctx, size / 2., size / 2.)
}

pub fn make_rectangle_raw(width: f32, height: f32) -> ([Vertex; 4], [u16; 6]) {
    let vertices = [
        Vertex {
            pos: Vec2::new(-width, -height),
            uv: Vec2::new(0., 0.),
        },
        Vertex {
            pos: Vec2::new(width, -height),
            uv: Vec2::new(1., 0.),
        },
        Vertex {
            pos: Vec2::new(width, height),
            uv: Vec2::new(1., 1.),
        },
        Vertex {
            pos: Vec2::new(-width, height),
            uv: Vec2::new(0., 1.),
        },
    ];
    let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
    (vertices, indices)
}
pub fn make_rectangle(ctx: &mut Context, width: f32, height: f32) -> (Buffer, Buffer, u16) {
    let (vertices, indices) = make_rectangle_raw(width, height);
    let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);
    let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);
    (vertex_buffer, index_buffer, 6)
}
