use glam::Vec2;
use miniquad::{Buffer, BufferType, Context, Texture};

use crate::graphics::Color;
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

pub fn make_arrow(ctx: &mut Context) -> (Buffer, Buffer) {
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
        }
    ];
    let indices: [u16; 9] = [0, 1, 2, 0, 2, 3, 1, 2, 4];

    let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);
    let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);
    (vertex_buffer, index_buffer)

}

pub fn make_square(ctx: &mut Context, size: f32) -> (Buffer, Buffer) {
    let vertices = [
        Vertex {
            pos: Vec2::new(-size / 2., -size / 2.),
            uv: Vec2::new(0., 1.),
        },
        Vertex {
            pos: Vec2::new(size / 2., -size / 2.),
            uv: Vec2::new(1., 1.),
        },
        Vertex {
            pos: Vec2::new(size / 2., size / 2.),
            uv: Vec2::new(1., 0.),
        },
        Vertex {
            pos: Vec2::new(-size / 2., size / 2.),
            uv: Vec2::new(0., 0.),
        },
    ];
    let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

    let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);
    let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);
    (vertex_buffer, index_buffer)
}
