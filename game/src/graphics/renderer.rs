use font::MappedCharInfo;
use miniquad::*;
// TODO(jhurstwright): Replace with no_std hashmap
use std::collections::HashMap;

use crate::graphics::font;
use crate::shaders;
use crate::utils;
use crate::{components, graphics, types, AssetIdentity};

pub type Materials = HashMap<AssetIdentity, MaterialAsset>;
pub type Meshes = HashMap<AssetIdentity, MeshAsset>;

#[derive(Debug, Clone)]
pub struct SpriteRenderCommand {
    pub binding: AssetIdentity,
    pub position: glam::Vec2,
    pub angle: f32,
    pub num_of_elements: i32,
}

#[derive(Debug, Clone)]
pub struct RenderFontCommand {
    pub font: String,
    pub text: String,
    pub position: glam::Vec2,
}

#[derive(Debug, Clone)]
pub enum RenderAssetCommands {
    LoadText {
        text: String,
        font: String,
    },
    // TOOD(jhurstwright): I really want to blindly create, and GC old texts later
    UpdateText {
        new_text: String,
        text: String,
        font: String,
    },
}

#[derive(Clone, Debug)]
pub struct DrawMesh2D {
    pub material: AssetIdentity,
    pub mesh: AssetIdentity,
    pub position: glam::Vec2,
    pub rotation: f32,
}

impl DrawMesh2D {
    pub fn model(&self) -> glam::Mat4 {
        glam::Mat4::from_rotation_translation(
            glam::Quat::from_axis_angle(glam::Vec3::new(0., 0., 1.), self.rotation),
            glam::Vec3::new(self.position.x, self.position.y, 0.),
        )
    }
}

#[derive(Clone, Debug)]
pub struct DrawFont {
    pub text: String,
    pub font: AssetIdentity,
    pub position: glam::Vec2,
}

impl DrawFont {
    pub fn model(&self) -> glam::Mat4 {
        glam::Mat4::from_rotation_translation(
            glam::Quat::from_axis_angle(glam::Vec3::new(0., 0., 1.), 0.),
            glam::Vec3::new(self.position.x, self.position.y, 0.),
        )
    }
}

#[derive(Clone, Debug)]
pub enum RenderCommand {
    DrawMesh2D(DrawMesh2D),
    DrawFont(DrawFont),
}

impl RenderCommand {
    pub fn into_draw_2d(&self) -> Option<&'_ DrawMesh2D> {
        match self {
            RenderCommand::DrawMesh2D(mesh) => Some(mesh),
            _ => None,
        }
    }

    pub fn into_draw_font(&self) -> Option<&'_ DrawFont> {
        match self {
            RenderCommand::DrawFont(font) => Some(font),
            _ => None,
        }
    }
}

pub struct RenderTarget {
    pub render_target: miniquad::Texture,
    pub depth_target: Option<miniquad::Texture>,
    pub commands: Vec<RenderCommand>,
    pass: miniquad::RenderPass,
}

impl RenderTarget {
    pub fn new(ctx: &mut miniquad::Context, width: u32, height: u32) -> Self {
        let render_target = miniquad::Texture::new_render_texture(
            ctx,
            miniquad::TextureParams {
                width,
                height,
                ..Default::default()
            },
        );
        let pass = miniquad::RenderPass::new(ctx, render_target, None);

        Self {
            render_target,
            depth_target: None,
            commands: Vec::with_capacity(128),
            pass,
        }
    }

    pub fn begin(&mut self, ctx: &mut miniquad::Context, action: miniquad::PassAction) {
        ctx.begin_pass(self.pass, action);
    }
}

#[derive(Debug, Clone)]
pub struct MeshAsset {
    pub identity: AssetIdentity,
    pub vertices: Vec<miniquad::Buffer>,
    pub indices: miniquad::Buffer,
    pub num_of_indices: u16,
}

impl MeshAsset {
    pub fn new<T: Into<AssetIdentity>>(
        identity: T,
        vertices: Vec<miniquad::Buffer>,
        indices: miniquad::Buffer,
        num_of_indices: u16,
    ) -> Self {
        Self {
            identity: identity.into(),
            vertices,
            indices,
            num_of_indices,
        }
    }

    pub fn bindings(&self, images: Vec<miniquad::Texture>) -> miniquad::Bindings {
        miniquad::Bindings {
            vertex_buffers: self.vertices.clone(),
            index_buffer: self.indices.clone(),
            images,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MaterialAsset {
    pub identity: AssetIdentity,
    pub textures: Vec<miniquad::Texture>,
}

impl MaterialAsset {
    pub fn new<T: Into<AssetIdentity>>(identity: T, textures: Vec<miniquad::Texture>) -> Self {
        Self {
            identity: identity.into(),
            textures,
        }
    }
}

pub struct MainRenderer {
    pub ctx: miniquad::Context,
    pub debug_font_bindings: miniquad::Bindings,
    pub shader_pipeline: miniquad::Pipeline,
    // TODO(jhurstwright): These should be consolidated into a UnionEnum
    pub render_font_commands: Vec<RenderFontCommand>,
    pub asset_commands: Vec<RenderAssetCommands>,
    pub fonts: HashMap<String, font::Font>,
    pub texts: HashMap<String, (Vec<miniquad::Buffer>, miniquad::Buffer)>,
    pub meshes: Meshes,
    pub materials: Materials,
    pub projection: glam::Mat4,
    pub view: glam::Mat4,
    pub main_render_target: RenderTarget,
    pub debug_render_target: RenderTarget,
    pub render_quad_pipeline: miniquad::Pipeline,
    pub render_quad: MeshAsset,
    pub ui_render_target: RenderTarget,
    pub ui_painter: crate::graphics::ui::MegaUI,
}

fn create_text_buffer(
    renderer: &mut graphics::MainRenderer,
    text: String,
    font: &String,
) -> Option<(Vec<miniquad::Buffer>, miniquad::Buffer)> {
    let font = match renderer.fonts.get(font) {
        Some(f) => f,
        _ => return None,
    };
    use crate::shaders::Vertex;
    use glam::Vec2;
    let mut vertices: Vec<Vertex> = Vec::with_capacity(text.chars().count() * 4);
    let mut indices: Vec<u16> = Vec::with_capacity(text.chars().count() * 6);
    let (width, height) = font.image_dimensions();
    let mut offset = 0.0f32;
    let scale = 0.025f32;
    for (index, character) in text.chars().enumerate() {
        let index = index as u16;
        if let Some(glyph) = font.glyphs.get(&character) {
            let w = (glyph.glyph_w as f32 / 2.) * scale;
            let h = (glyph.glyph_h as f32 / 2.) * scale;

            let MappedCharInfo {
                x,
                y,
                width,
                height,
            } = glyph.scaled_position(width as f32, height as f32);

            vertices.push(Vertex {
                pos: Vec2::new(offset - w, -h),
                uv: Vec2::new(x, y + height),
            });
            vertices.push(Vertex {
                pos: Vec2::new(offset + w, -h),
                uv: Vec2::new(x + width, y + height),
            });
            vertices.push(Vertex {
                pos: Vec2::new(offset + w, h),
                uv: Vec2::new(x + width, y),
            });
            vertices.push(Vertex {
                pos: Vec2::new(offset - w, h),
                uv: Vec2::new(x, y),
            });

            indices.push(0 + (index * 4));
            indices.push(1 + (index * 4));
            indices.push(2 + (index * 4));
            indices.push(0 + (index * 4));
            indices.push(2 + (index * 4));
            indices.push(3 + (index * 4));

            offset += glyph.advance * scale;
        }
    }
    let vertex_buffer = Buffer::immutable(&mut renderer.ctx, BufferType::VertexBuffer, &vertices);
    let index_buffer = Buffer::immutable(&mut renderer.ctx, BufferType::IndexBuffer, &indices);
    Some((vec![vertex_buffer], index_buffer))
}

impl MainRenderer {
    pub fn new(mut context: miniquad::Context) -> Self {
        let materials = HashMap::new();
        let meshes = HashMap::new();
        let mut fonts = HashMap::new();
        let (shader_pipeline, render_quad_pipeline, render_quad, debug_font_bindings) = {
            let ctx = &mut context;

            let shader = shaders::sprite::new(ctx).unwrap();
            let shader_pipeline = Pipeline::with_params(
                ctx,
                &[BufferLayout::default()],
                &shaders::Vertex::buffer_formats(),
                shader,
                PipelineParams {
                    color_blend: Some(BlendState::new(
                        miniquad::Equation::Add,
                        miniquad::BlendFactor::Value(BlendValue::SourceAlpha),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    )),
                    ..Default::default()
                },
            );
            let shader = shaders::screen::new(ctx).unwrap();
            let render_quad_pipeline = Pipeline::with_params(
                ctx,
                &[miniquad::BufferLayout::default()],
                &shaders::Vertex::buffer_formats(),
                shader,
                miniquad::PipelineParams {
                    color_blend: Some(BlendState::new(
                        miniquad::Equation::Add,
                        miniquad::BlendFactor::Value(BlendValue::SourceAlpha),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    )),
                    ..Default::default()
                },
            );

            let mut fallback_font =
                font::Font::load("KenneyFuture", include_bytes!("KenneyFuture.ttf"));
            for char in font::ascii_character_list() {
                fallback_font.cache_glyph(char);
            }
            let tex = fallback_font.texture(ctx);
            let (vertices, indices, _) = utils::make_square(ctx, 32.);
            let bindings = miniquad::Bindings {
                vertex_buffers: vec![vertices],
                index_buffer: indices,
                images: vec![tex],
            };
            fonts.insert(fallback_font.name.clone(), fallback_font);

            let render_mesh = crate::utils::make_rectangle(ctx, 1., 1.);
            let render_quad = MeshAsset::new(
                "MainRenderTarget",
                vec![render_mesh.0],
                render_mesh.1,
                render_mesh.2,
            );
            (shader_pipeline, render_quad_pipeline, render_quad, bindings)
        };

        let shader = shaders::ui::new(&mut context).unwrap();
        let ui_pipeline = Pipeline::with_params(
            &mut context,
            &[miniquad::BufferLayout::default()],
            &shaders::UiVertex::buffer_formats(),
            shader,
            miniquad::PipelineParams {
                color_blend: Some(BlendState::new(
                    miniquad::Equation::Add,
                    miniquad::BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),
                ..Default::default()
            },
        );
        let (width, height) = context.screen_size();
        let main_render_target = RenderTarget::new(&mut context, width as u32, height as u32);
        let debug_render_target = RenderTarget::new(&mut context, width as u32, height as u32);
        let ui_render_target = RenderTarget::new(&mut context, width as u32, height as u32);
        let ui_painter = crate::graphics::ui::MegaUI::new(&mut context);

        Self {
            asset_commands: Vec::with_capacity(32),
            debug_font_bindings,
            fonts,
            materials,
            meshes,
            texts: HashMap::new(),
            projection: glam::Mat4::identity(),
            render_font_commands: Vec::with_capacity(64),
            shader_pipeline,
            render_quad_pipeline,
            view: glam::Mat4::identity(),
            main_render_target,
            debug_render_target,
            render_quad,
            ctx: context,
            ui_render_target,
            ui_painter,
        }
    }

    pub fn update_view(&mut self, camera: &components::Camera2D) {
        self.projection = camera.projection;
        self.view = camera.view;
    }

    #[allow(dead_code)]
    pub fn resize(&mut self, _width: f32, _height: f32) {
        // Resize UIRenderTarget, MainRenderTarget, DebugRenderTarget
        todo!()
    }

    pub fn add_material<T: Into<AssetIdentity>>(
        &mut self,
        name: T,
        textures: Vec<miniquad::Texture>,
    ) {
        let asset_name = name.into();
        let material = MaterialAsset::new(asset_name.clone(), textures);
        self.materials.insert(asset_name, material);
    }

    pub fn add_mesh<T: Into<AssetIdentity>>(
        &mut self,
        name: T,
        vertices: &[shaders::Vertex],
        indices: &[u16],
    ) {
        let asset = name.into();
        let vertex_buffer = Buffer::immutable(&mut self.ctx, BufferType::VertexBuffer, &vertices);
        let index_buffer = Buffer::immutable(&mut self.ctx, BufferType::IndexBuffer, &indices);
        let mesh = MeshAsset::new(
            asset.clone(),
            vec![vertex_buffer],
            index_buffer,
            indices.len() as u16,
        );
        self.meshes.insert(asset, mesh);
    }

    pub fn load_assets(&mut self) {
        let commands: Vec<RenderAssetCommands> = self.asset_commands.drain(..).collect();
        commands.iter().for_each(|cmd| match cmd {
            RenderAssetCommands::LoadText { text, font } => {
                if !self.texts.contains_key(text) {
                    let buffer = create_text_buffer(self, text.clone(), font);
                    if let Some(buffers) = buffer {
                        self.texts.insert(text.clone(), buffers);
                    }
                }
            }
            RenderAssetCommands::UpdateText {
                text,
                font,
                new_text,
            } => {
                if let Some((vertices, indices)) = self.texts.remove(text) {
                    vertices.iter().for_each(|b| b.delete());
                    indices.delete();
                }
                let buffer = create_text_buffer(self, new_text.clone(), font);
                if let Some(buffers) = buffer {
                    self.texts.insert(new_text.clone(), buffers);
                }
            }
        });
    }

    fn draw_main_target(&mut self) {
        let mut uniform = crate::shaders::sprite::VertexUniforms {
            projection: self.projection,
            view: self.view,
            model: glam::Mat4::identity(),
        };

        self.main_render_target.begin(
            &mut self.ctx,
            miniquad::PassAction::Clear {
                color: Some(types::colors::DARKGRAY.into()),
                depth: Some(1.),
                stencil: None,
            },
        );

        self.ctx.apply_pipeline(&self.shader_pipeline);

        for render_cmd in self
            .main_render_target
            .commands
            .iter()
            .filter_map(|draw| draw.into_draw_2d())
        {
            let (bindings, elements) = self.prepare_draw(&render_cmd.mesh, &render_cmd.material);
            let model = render_cmd.model();
            uniform.model = model;
            self.ctx.apply_bindings(&bindings);
            self.ctx.apply_uniforms(&uniform);
            self.ctx.draw(0, elements as i32, 1);
        }
        for font_cmd in self
            .main_render_target
            .commands
            .iter()
            .filter_map(|cmd| cmd.into_draw_font())
        {
            let (v, i) = &self
                .texts
                .get(&font_cmd.text)
                .expect("Text should be in GPU memory, but isn't");
            let elements = font_cmd.text.len() as i32 * 6;
            let m = &self.debug_font_bindings.images;
            let bindings = miniquad::Bindings {
                vertex_buffers: v.clone(),
                index_buffer: i.clone(),
                images: m.clone(),
            };
            let model = font_cmd.model();
            uniform.model = model;
            self.ctx.apply_bindings(&bindings);
            self.ctx.apply_uniforms(&uniform);
            self.ctx.draw(0, elements as i32, 1);
        }

        // Render the Font
        for cmd in self.render_font_commands.iter() {
            let RenderFontCommand { text, position, .. } = cmd;
            if let Some((v, i)) = &self.texts.get(text) {
                let model = glam::Mat4::from_rotation_translation(
                    glam::Quat::from_axis_angle(glam::Vec3::new(0., 0., 1.), (0.0f32).to_radians()),
                    glam::Vec3::new(position.x, position.y, 0.),
                );
                let m = &self.debug_font_bindings.images;
                uniform.model = model;
                let bindings = miniquad::Bindings {
                    vertex_buffers: v.clone(),
                    index_buffer: i.clone(),
                    images: m.clone(),
                };
                self.ctx.apply_bindings(&bindings);
                self.ctx.apply_uniforms(&uniform);
                self.ctx.draw(0, 6 * text.len() as i32, 1);
            }
        }
        self.ctx.end_render_pass();
        self.main_render_target.commands.clear();
    }

    fn draw_debug_target(&mut self) {
        let mut uniform = crate::shaders::sprite::VertexUniforms {
            projection: self.projection,
            view: self.view,
            model: glam::Mat4::identity(),
        };

        self.debug_render_target.begin(
            &mut self.ctx,
            miniquad::PassAction::Clear {
                color: Some((0., 0., 0., 0.).into()),
                depth: None,
                stencil: None,
            },
        );
        self.ctx.apply_pipeline(&self.shader_pipeline);
        let draw_sprite = self
            .debug_render_target
            .commands
            .iter()
            .filter_map(|draw| draw.into_draw_2d());

        for render_cmd in draw_sprite {
            let (bindings, elements) = self.prepare_draw(&render_cmd.mesh, &render_cmd.material);
            let model = render_cmd.model();
            uniform.model = model;
            self.ctx.apply_bindings(&bindings);
            self.ctx.apply_uniforms(&uniform);
            self.ctx.draw(0, elements as i32, 1);
        }
        self.ctx.end_render_pass();
        self.debug_render_target.commands.clear();
    }

    fn draw_layers_to_default(&mut self) {
        self.ctx.begin_default_pass(PassAction::Nothing);

        // TODO: Add post processinging pipeline
        self.ctx.apply_pipeline(&self.render_quad_pipeline);
        let main_render_bindings = miniquad::Bindings {
            vertex_buffers: self.render_quad.vertices.clone(),
            index_buffer: self.render_quad.indices,
            images: vec![self.main_render_target.render_target],
        };
        self.ctx.apply_bindings(&main_render_bindings);
        self.ctx.draw(0, self.render_quad.num_of_indices as i32, 1);

        let debug_render_pass = miniquad::Bindings {
            vertex_buffers: self.render_quad.vertices.clone(),
            index_buffer: self.render_quad.indices,
            images: vec![self.debug_render_target.render_target],
        };
        self.ctx.apply_bindings(&debug_render_pass);
        self.ctx.draw(0, self.render_quad.num_of_indices as i32, 1);

        self.ctx.end_render_pass();
    }

    pub fn draw(&mut self) {
        self.draw_main_target();
        self.draw_debug_target();
        self.draw_layers_to_default();

        self.ctx.commit_frame();
    }

    fn prepare_draw(
        &self,
        mesh: &AssetIdentity,
        material: &AssetIdentity,
    ) -> (miniquad::Bindings, i32) {
        let mesh = self
            .meshes
            .get(mesh)
            .expect("Failed to get mesh, and it should have been loaded before drying to draw it");
        let material = self
            .materials
            .get(material)
            .expect("Failed to get material, and developer failed to implement fallback default");
        let bindings = mesh.bindings(material.textures.clone());
        (bindings, mesh.num_of_indices as i32)
    }
}
