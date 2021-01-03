use crate::shaders::ui as shader_ui;
use miniquad::{
    Bindings, BlendFactor, BlendState, BlendValue, Buffer, BufferLayout, BufferType, Equation,
    Pipeline, PipelineParams,
};
use shader_ui::UiVertex;

pub struct MegaUI {
    ui: megaui::Ui,
    ui_draw_list: Vec<megaui::DrawList>,
    pipeline: Pipeline,
    bindings: Bindings,
    vertex_buffer_size: usize,
    index_buffer_size: usize,
    texture_version: u64,
}

impl MegaUI {
    pub fn new(ctx: &mut miniquad::Context) -> MegaUI {
        let ui = megaui::Ui::new();
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

        MegaUI {
            ui,
            ui_draw_list: Vec::with_capacity(256),
            pipeline,
            bindings,
            index_buffer_size,
            vertex_buffer_size,
            texture_version: 0,
        }
    }

    // fn paint_job(&mut self, ctx: &mut miniquad::Context, (clip_rect, mesh): egui::paint::PaintJob) {
    //     let texture = self.bindings.images[0];
    //     if self.vertex_buffer_size < mesh.vertices.len() {
    //         self.vertex_buffer_size = mesh.vertices.len();
    //         self.bindings.vertex_buffers[0].delete();
    //         self.bindings.vertex_buffers[0] = Buffer::stream(
    //             ctx,
    //             BufferType::VertexBuffer,
    //             self.vertex_buffer_size * std::mem::size_of::<UiVertex>(),
    //         );
    //     }

    //     if self.index_buffer_size < mesh.indices.len() {
    //         self.index_buffer_size = mesh.indices.len();
    //         self.bindings.index_buffer.delete();
    //         self.bindings.index_buffer = Buffer::stream(
    //             ctx,
    //             BufferType::IndexBuffer,
    //             self.index_buffer_size * std::mem::size_of::<u16>(),
    //         );
    //     }

    //     let vertices = mesh
    //         .vertices
    //         .iter()
    //         .map(|v| UiVertex {
    //             pos: (v.pos.x, v.pos.y).into(),
    //             ui: (v.uv.x, v.uv.y).into(),
    //             color: v.color.to_array(),
    //         })
    //         .collect::<Vec<UiVertex>>();

    //     self.bindings.vertex_buffers[0].update(ctx, &vertices);

    //     let indices = mesh.indices.iter().map(|x| *x as u16).collect::<Vec<u16>>();
    //     self.bindings.index_buffer.update(ctx, &indices);

    //     let screen_size = ctx.screen_size();
    //     ctx.begin_default_pass(miniquad::PassAction::Nothing);
    //     ctx.apply_pipeline(&self.pipeline);


    //     // ctx.apply_scissor_rect(
    //     //     clip_min_x,
    //     //     height_pixels as i32 - clip_max_y,
    //     //     clip_max_x - clip_min_x,
    //     //     clip_max_y - clip_min_y,
    //     // );
    //     // ctx.apply_bindings(&self.bindings);
    //     // ctx.apply_uniforms(&shader_ui::UiUniforms{
    //     //     screen_size: [screen_size.0, screen_size.1].into(),
    //     // });

    //     // ctx.draw(0, mesh.indices.len() as i32, 1);
    //     // ctx.end_render_pass();
    //     // ctx.commit_frame();
    // }

    // pub fn paint(
    //     &mut self,
    //     context: &mut miniquad::Context,
    //     jobs: egui::paint::PaintJobs,
    //     texture: Arc<egui::paint::Texture>,
    // ) {
    //     if texture.version != self.texture_version {
    //         self.rebuild_texture(context, texture);
    //     }

    //     for job in jobs {
    //         self.paint_job(context, job);
    //     }
    // }
}
