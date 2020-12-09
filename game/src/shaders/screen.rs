use miniquad::*;

pub const VERTEX: &str = include_str!("./screen.vert");
pub const FRAGMENT: &str = include_str!("./screen.frag");

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout { uniforms: vec![] },
    }
}

pub fn new(ctx: &mut Context) -> Result<Shader, ShaderError> {
    Shader::new(ctx, VERTEX, FRAGMENT, meta())
}
