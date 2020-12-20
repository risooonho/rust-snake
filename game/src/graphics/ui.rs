use std::sync::Arc;
use crate::shaders::ui as shader_ui;
use miniquad::{
    Bindings, BlendFactor, BlendState, BlendValue, Buffer, BufferLayout, BufferType, Equation,
    Pipeline, PipelineParams,
};
use shader_ui::UiVertex;

pub struct Painter {
    pipeline: Pipeline,
    bindings: Bindings,
    vertex_buffer_size: usize,
    index_buffer_size: usize,
    texture_version: u64,
}

impl Painter {
    pub fn new(ctx: &mut miniquad::Context) -> Painter {
        let shader = miniquad::Shader::new(
            ctx,
            shader_ui::VERTEX,
            shader_ui::FRAGMENT,
            shader_ui::meta(),
        );

        let pipeline = Pipeline::with_params(
            ctx,
            &[BufferLayout::default()],
            crate::shaders::ui::UiVertex::buffer_formats(),
            shader.expect("could not make UI shader"),
            PipelineParams {
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),
                ..Default::default()
            },
        );

        let vertex_buffer_size = 100;
        let vertex_buffer = Buffer::stream(
            ctx,
            BufferType::VertexBuffer,
            vertex_buffer_size * std::mem::size_of::<UiVertex>(),
        );

        let index_buffer_size = 100;
        let index_buffer = Buffer::stream(
            ctx,
            BufferType::IndexBuffer,
            index_buffer_size * std::mem::size_of::<u16>(),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![miniquad::Texture::empty()],
        };

        Painter {
            pipeline,
            bindings,
            index_buffer_size,
            vertex_buffer_size,
            texture_version: 0,
        }
    }

    fn rebuild_texture(&mut self, ctx: &mut miniquad::Context, texture: Arc<egui::paint::Texture>) {}
    fn paint_job(&mut self, ctx: &mut miniquad::Context, (clip_rect, mesh): egui::paint::PaintJob) {}


    pub fn paint(&mut self, context: &mut miniquad::Context, jobs: egui::paint::PaintJobs, texture: Arc<egui::paint::Texture>) {
        if texture.version != self.texture_version {
            self.rebuild_texture(context, texture);
        }

        for job in jobs {
            self.paint_job(context, job);
        }
    }

}
