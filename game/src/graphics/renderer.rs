use miniquad::*;
use smallvec::SmallVec;
use std::collections::HashMap;

use crate::assets;
use crate::components;
use crate::graphics;
use crate::shaders;

pub type RenderCommands = SmallVec<[SpriteRenderCommand; 64]>;
pub type Materials = HashMap<assets::AssetType, Vec<Texture>>;
pub type Meshes = HashMap<assets::AssetType, (Vec<miniquad::Buffer>, miniquad::Buffer)>;

#[derive(Debug, Clone, Copy)]
pub struct SpriteRenderCommand {
    pub binding: assets::AssetType,
    pub position: glam::Vec2,
}

pub struct MainRenderer {
    pub shader_pipeline: miniquad::Pipeline,
    pub render_commands: SmallVec<[SpriteRenderCommand; 64]>,
    pub meshes: Meshes,
    pub materials: Materials,
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

        let mut materials = HashMap::new();
        let mut meshes = HashMap::new();

        let snake_texture = new_snake_texture(ctx);
        let tail_texture = new_tail_texture(ctx);
        let food_texture = new_food_texture(ctx);
        materials.insert(assets::AssetType::Food, vec![food_texture]);
        materials.insert(assets::AssetType::Tail, vec![tail_texture]);
        materials.insert(assets::AssetType::Snake, vec![snake_texture]);

        let snake_mesh = crate::utils::make_square(ctx, 1.);
        let food_mesh = crate::utils::make_square(ctx, 0.8);
        let tail_mesh = crate::utils::make_square(ctx, 0.8);

        meshes.insert(assets::AssetType::Food, (vec![food_mesh.0], food_mesh.1));
        meshes.insert(assets::AssetType::Tail, (vec![tail_mesh.0], tail_mesh.1));
        meshes.insert(assets::AssetType::Snake, (vec![snake_mesh.0], snake_mesh.1));

        bindings.insert(assets::AssetType::Food, snake_food_binding);
        bindings.insert(assets::AssetType::Snake, snake_bindings);
        bindings.insert(assets::AssetType::Tail, tail_bindings);

        Self {
            shader_pipeline,
            render_commands: SmallVec::new(),
            materials,
            meshes,
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
                let (vertex_buffers, index_buffer) = match self.meshes.get(binding) {
                    Some(m) => m,
                    _ => continue,
                };
                let material = match self.materials.get(binding) {
                    Some(m) => m,
                    _ => continue,
                };
                let model = glam::Mat4::from_rotation_translation(
                    glam::Quat::from_axis_angle(glam::Vec3::new(0., 0., 1.), 0.),
                    glam::Vec3::new(position.x, position.y, 0.),
                );
                uniform.model = model;
                let bindings = miniquad::Bindings {
                    vertex_buffers: vertex_buffers.clone(),
                    index_buffer: index_buffer.clone(),
                    images: material.clone(),
                };
                ctx.apply_bindings(&bindings);
                ctx.apply_uniforms(&uniform);
                ctx.draw(0, 6, 1);
            }
        }
        ctx.end_render_pass();
        ctx.commit_frame();
        self.render_commands.clear();
    }
}

pub fn new_food_texture(ctx: &mut Context) -> miniquad::Texture {
    crate::utils::build_square_texture(ctx, 4, crate::graphics::colors::PURPLE)
}

pub fn new_food_bindings(ctx: &mut Context) -> miniquad::Bindings {
    let texture = new_food_texture(ctx);
    let (vertex_buffer, index_buffer) = crate::utils::make_square(ctx, 0.8);

    miniquad::Bindings {
        vertex_buffers: vec![vertex_buffer],
        index_buffer,
        images: vec![texture],
    }
}

pub fn new_snake_texture(ctx: &mut Context) -> miniquad::Texture {
    crate::utils::build_square_texture(ctx, 4, crate::graphics::colors::RAYWHITE)
}

pub fn new_snake_bindings(ctx: &mut Context) -> miniquad::Bindings {
    let texture = new_snake_texture(ctx);
    let (vertex_buffer, index_buffer) = crate::utils::make_square(ctx, 1.);

    miniquad::Bindings {
        vertex_buffers: vec![vertex_buffer],
        index_buffer,
        images: vec![texture],
    }
}

pub fn new_tail_texture(ctx: &mut Context) -> miniquad::Texture {
    crate::utils::build_square_texture(ctx, 4, crate::graphics::colors::RAYWHITE)
}

pub fn new_tail_bindings(ctx: &mut Context) -> miniquad::Bindings {
    let texture = new_tail_texture(ctx);
    let (vertex_buffer, index_buffer) = crate::utils::make_square(ctx, 0.8);

    miniquad::Bindings {
        vertex_buffers: vec![vertex_buffer],
        index_buffer,
        images: vec![texture],
    }
}