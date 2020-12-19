pub const VERTEX: &str = include_str!("./ui.vert");
pub const FRAGMENT: &str = include_str!("./ui.frag");

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

const UI_VERTEX_BUFFERS: [miniquad::VertexAttribute; 3] = [
    miniquad::VertexAttribute::new("a)ois", miniquad::VertexFormat::Float2),
    miniquad::VertexAttribute::new("a_tc", miniquad::VertexFormat::Float2),
    miniquad::VertexAttribute::new("aa_color", miniquad::VertexFormat::Byte4),
];

#[repr(C)]
pub struct UiUniforms {
    pub screen_size: glam::Vec2,
    pub tex_size: glam::Vec2,
}

pub fn meta() -> miniquad::ShaderMeta {
    miniquad::ShaderMeta {
        images: vec!["u_sampler".to_string()],
        uniforms: miniquad::UniformBlockLayout {
            uniforms: vec![
                miniquad::UniformDesc::new("u_screen_size", miniquad::UniformType::Float2),
                miniquad::UniformDesc::new("u_text_size", miniquad::UniformType::Float2),
            ],
        },
    }
}

pub fn new(ctx: &mut miniquad::Context) -> Result<miniquad::Shader, miniquad::ShaderError> {
    miniquad::Shader::new(ctx, VERTEX, FRAGMENT, meta())
}
