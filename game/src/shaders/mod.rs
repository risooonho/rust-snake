use glam::Vec2;

pub mod sprite;


#[repr(C)]
pub struct Vertex {
    pub pos: Vec2,
    pub uv: Vec2,
}