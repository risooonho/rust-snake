use miniquad::*;
use smallvec::SmallVec;
use std::collections::HashMap;

use crate::assets;
use crate::components;
use crate::graphics;
use crate::shaders;

pub type RenderCommands = SmallVec<[SpriteRenderCommand; 64]>;

#[derive(Debug, Clone, Copy)]
pub struct SpriteRenderCommand {
    pub binding: assets::AssetType,
    pub position: glam::Vec2,
}

pub struct MainRenderer {
    pub shader_pipeline: miniquad::Pipeline,
    pub render_commands: SmallVec<[SpriteRenderCommand; 64]>,
    pub bindings: assets::BindingAssets,
    pub projection: glam::Mat4,
    pub view: glam::Mat4,
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

        let mut bindings = HashMap::new();
        let snake_food_binding = new_food_bindings(ctx);
        let snake_bindings = new_snake_bindings(ctx);
        let tail_bindings = new_tail_bindings(ctx);
        bindings.insert(assets::AssetType::Food, snake_food_binding);
        bindings.insert(assets::AssetType::Snake, snake_bindings);
        bindings.insert(assets::AssetType::Tail, tail_bindings);

        Self {
            shader_pipeline,
            render_commands: SmallVec::new(),
            bindings,
            projection: glam::Mat4::identity(),
            view: glam::Mat4::identity(),
        }
    }

    pub fn update_view(&mut self, camera: &components::Camera2D) {
        self.projection = camera.projection;
        self.view = camera.view;
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(PassAction::Clear {
            color: Some(graphics::colors::DARKGRAY.into()),
            depth: Some(1.),
            stencil: None,
        });

        let mut uniform = crate::shaders::sprite::VertexUniforms {
            projection: self.projection,
            view: self.view,
            model: glam::Mat4::identity(),
        };

        ctx.apply_pipeline(&self.shader_pipeline);
        {
            for SpriteRenderCommand { position, binding } in self.render_commands.iter() {
                if let Some(binding) = self.bindings.get(&binding) {
                    let model = glam::Mat4::from_rotation_translation(
                        glam::Quat::from_axis_angle(glam::Vec3::new(0., 0., 1.), 0.),
                        glam::Vec3::new(position.x, position.y, 0.),
                    );
                    uniform.model = model;
                    ctx.apply_bindings(&binding);
                    ctx.apply_uniforms(&uniform);
                    ctx.draw(0, 6, 1);
                }
            }
        }
        ctx.end_render_pass();
        ctx.commit_frame();
        self.render_commands.clear();
    }
}

pub fn new_food_bindings(ctx: &mut Context) -> miniquad::Bindings {
    let texture = crate::utils::build_square_texture(ctx, 4, crate::graphics::colors::PURPLE);
    let (vertex_buffer, index_buffer) = crate::utils::make_square(ctx, 0.8);

    miniquad::Bindings {
        vertex_buffers: vec![vertex_buffer],
        index_buffer,
        images: vec![texture],
    }
}

pub fn new_snake_bindings(ctx: &mut Context) -> miniquad::Bindings {
    let texture = crate::utils::build_square_texture(ctx, 4, crate::graphics::colors::RAYWHITE);
    let (vertex_buffer, index_buffer) = crate::utils::make_square(ctx, 1.);

    miniquad::Bindings {
        vertex_buffers: vec![vertex_buffer],
        index_buffer,
        images: vec![texture],
    }
}

pub fn new_tail_bindings(ctx: &mut Context) -> miniquad::Bindings {
    let texture = crate::utils::build_square_texture(ctx, 4, crate::graphics::colors::RAYWHITE);
    let (vertex_buffer, index_buffer) = crate::utils::make_square(ctx, 0.8);

    miniquad::Bindings {
        vertex_buffers: vec![vertex_buffer],
        index_buffer,
        images: vec![texture],
    }
}