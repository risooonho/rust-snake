pub mod screen;
pub mod sprite;
pub mod ui;

const VERTEX_BUFFERS: [miniquad::VertexAttribute; 2] = [
    miniquad::VertexAttribute::new("pos", miniquad::VertexFormat::Float2),
    miniquad::VertexAttribute::new("uv", miniquad::VertexFormat::Float2),
];

#[repr(C)]
#[derive(Debug)]
pub struct Vertex {
    pub pos: glam::Vec2,
    pub uv: glam::Vec2,
}

impl Vertex {
    pub fn buffer_formats() -> &'static [miniquad::VertexAttribute] {
        &VERTEX_BUFFERS
    }
}

const UI_VERTEX_BUFFERS: [miniquad::VertexAttribute; 3] = [
    miniquad::VertexAttribute::new("pos", miniquad::VertexFormat::Float2),
    miniquad::VertexAttribute::new("uv", miniquad::VertexFormat::Float2),
    miniquad::VertexAttribute::new("color", miniquad::VertexFormat::Float4),
];

#[repr(C)]
#[derive(Debug)]
pub struct UiVertex {
    pub pos: glam::Vec2,
    pub ui: glam::Vec2,
    pub color: glam::Vec4,
}

impl UiVertex {
    pub fn buffer_formats() -> &'static [miniquad::VertexAttribute] {
        &UI_VERTEX_BUFFERS
    }
}
