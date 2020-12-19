use crate::shaders::ui as shader_ui;
use miniquad::{
    Bindings, BlendFactor, BlendState, BlendValue, Buffer, BufferLayout, BufferType, Equation,
    Pipeline, PipelineParams, Texture,
};
use shader_ui::UiVertex;

pub struct Painter {
    pipeline: Pipeline,
    bindings: Bindings,
    vertex_buffer_size: usize,
    index_buffer_size: usize,
    texture: Option<Texture>,
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
            texture: None,
        }
    }
}
