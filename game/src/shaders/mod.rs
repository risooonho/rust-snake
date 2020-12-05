use glam::Vec2;

pub mod screen;
pub mod sprite;

#[repr(C)]
#[derive(Debug)]
pub struct Vertex {
    pub pos: Vec2,
    pub uv: Vec2,
}

impl Vertex {
    pub fn buffer_formats() -> [miniquad::VertexAttribute; 2] {
        [
            miniquad::VertexAttribute::new("pos", miniquad::VertexFormat::Float2),
            miniquad::VertexAttribute::new("uv", miniquad::VertexFormat::Float2),
        ]
    }
}
