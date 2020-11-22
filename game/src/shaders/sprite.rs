use glam::Mat4;
use miniquad::*;

pub const VERTEX: &str = include_str!("./sprite.vert");
pub const FRAGMENT: &str = include_str!("./sprite.frag");

#[repr(C)]
pub struct VertexUniforms {
    pub model: Mat4,
    pub view: Mat4,
    pub projection: Mat4,
}

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("model", UniformType::Mat4),
                UniformDesc::new("view", UniformType::Mat4),
                UniformDesc::new("projection", UniformType::Mat4),
            ],
        },
    }
}

pub fn new(ctx: &mut Context) -> Result<Shader, ShaderError> {
    Shader::new(ctx, VERTEX, FRAGMENT, meta())
}
