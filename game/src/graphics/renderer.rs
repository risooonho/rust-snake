use miniquad::*;
use crate::graphics;
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

    pub fn begin_default_pass(&self, ctx: &mut Context) {
        ctx.begin_default_pass(PassAction::Clear {
            color: Some(graphics::colors::DARKGRAY.into()),
            depth: Some(1.),
            stencil: None,
        });
    }

    pub fn apply_sprite_pipeline(&self, ctx: &mut Context) {
        ctx.apply_pipeline(&self.shader_pipeline);
    }

    pub fn end_render_pass(&self, ctx: &mut Context) {
        ctx.end_render_pass();
    }

    pub fn commit_frame(&self, ctx: &mut Context) {
        ctx.commit_frame();
    }
}
