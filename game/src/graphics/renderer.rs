use miniquad::*;
use crate::shaders;

pub struct MainRenderer {
    pub shader_pipeline: miniquad::Pipeline,
}

impl MainRenderer {
    pub fn new(ctx: &mut Context) -> Self {
        let shader = shaders::sprite::new(ctx).unwrap();
        let shader_pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
        );
        Self {
            shader_pipeline,
        }
    }

    pub fn apply_sprite_pipeline(&mut self, ctx: &mut Context) {
        ctx.apply_pipeline(&self.shader_pipeline);
    }
}
