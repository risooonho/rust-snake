use glam::Mat4;

pub const VERTEX: &str = include_str!("./ui.vert");
pub const FRAGMENT: &str = include_str!("./ui.frag");

#[repr(C)]
pub struct UiUniforms {
    pub model: Mat4,
    pub proj_view: Mat4,
}

pub fn meta() -> miniquad::ShaderMeta {
    miniquad::ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: miniquad::UniformBlockLayout {
            uniforms: vec![
                miniquad::UniformDesc::new("model", miniquad::UniformType::Mat4),
                miniquad::UniformDesc::new("proj_view", miniquad::UniformType::Mat4),
            ],
        },
    }
}

pub fn new(ctx: &mut miniquad::Context) -> Result<miniquad::Shader, miniquad::ShaderError> {
    miniquad::Shader::new(ctx, VERTEX, FRAGMENT, meta())
}
