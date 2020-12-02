use components::Input;
use miniquad::{Buffer, BufferType, Context, KeyCode, KeyMods};

use crate::components;
use crate::graphics::{self, font};
use crate::stages::{NextStage, Paused, Stage};
use crate::systems::{self, GameWorld};

pub struct GameState {
    direction: components::Direction,
    game_world: GameWorld,
    input: components::Input,
    move_timer: components::Timer,
    food_timer: components::Timer,
}

impl GameState {
    pub fn new(ctx: &mut miniquad::Context) -> Self {
        let mut game_world = GameWorld {
            events: Vec::with_capacity(32),
            camera: components::Camera2D::new(ctx, 20.),
            world: hecs::World::new(),
        };
        systems::create_snake_system(&mut game_world);
        game_world.world.spawn((
            components::Position(glam::Vec2::new(-10., -4.)),
            components::Text::new("Test ABC"),
        ));

        GameState {
            direction: components::Direction::Up,
            game_world,
            move_timer: components::Timer::new(0.25),
            input: components::Input::default(),
            food_timer: components::Timer::new(1.5),
        }
    }
}

impl Stage for GameState {
    fn enter(&mut self, _ctx: &mut Context) {
        self.move_timer.resume();
        self.food_timer.resume();
    }

    fn exit(&mut self, _ctx: &mut Context) {
        self.move_timer.paused();
        self.food_timer.paused();
    }

    fn resize_event(&mut self, ctx: &mut Context, _width: f32, _height: f32) {
        self.game_world.camera.resize(ctx);
    }

    fn update(&mut self, input: &Input, _ctx: &mut Context) -> NextStage {
        self.input = input.clone();
        if self.input.pause {
            self.input.pause = false;
            return NextStage::Push(Box::new(Paused::new()));
        }
        self.direction.update(&self.input);
        systems::update_input(&mut self.game_world, &self.input);
        if self.move_timer.finished() {
            crate::systems::update_velocity_direction(&mut self.game_world);
            systems::tail_movement_system(&mut self.game_world);
            systems::head_collision_system(&mut self.game_world);
            systems::handle_collision_system(&mut self.game_world);
            systems::trigger_tail_spawn(&mut self.game_world);
            systems::movement_system(&mut self.game_world);
            systems::spawn_tail_system(&mut self.game_world);
            self.move_timer.reset();
        }
        if self.food_timer.finished() {
            systems::add_food_system(&mut self.game_world);
            self.food_timer.reset();
        }

        systems::despawn_food_system(&mut self.game_world);
        if systems::game_over_system(&mut self.game_world) {
            self.move_timer.reset();
            self.food_timer.reset();
        }

        self.input = Default::default();
        self.game_world.events.clear();
        NextStage::Noop
    }

    fn draw(&mut self, ctx: &mut Context, renderer: &mut graphics::MainRenderer) {
        renderer.update_view(&self.game_world.camera);
        systems::gather_render_cmds(&mut self.game_world, &mut renderer.render_commands);
        systems::debug_render_cmds(&mut self.game_world, &mut renderer.render_commands);
        draw_text(ctx, &mut self.game_world, renderer);
        renderer.draw(ctx);
    }
}

fn draw_text(ctx: &mut Context, game_world: &mut GameWorld, renderer: &mut graphics::MainRenderer) {
    use crate::shaders::Vertex;
    use glam::Vec2;
    let GameWorld { world, .. } = game_world;

    for (_, (text, pos)) in &mut world.query::<(&components::Text, &components::Position)>() {
        let font = match renderer.fonts.get("KenneyFuture") {
            Some(f) => f,
            _ => return,
        };
        if let Some(_) = renderer.texts.get(&text.string) {
            let cmd = graphics::renderer::RenderFontCommand {
                font: "KenneyFuture".to_string(),
                text: text.string.clone(),
                position: pos.0,
            };
            renderer.render_font_commands.push(cmd);
            continue;
        }
        let mut vertices: Vec<Vertex> = Vec::with_capacity(text.string.chars().count() * 4);
        let mut indices: Vec<u16> = Vec::with_capacity(text.string.chars().count() * 6);
        let (width, height) = font.image_dimensions();
        let mut offset = 0.0f32;
        let scale = 0.025f32;
        for (index, character) in text.string.chars().enumerate() {
            let index = index as u16;
            if let Some(glyph) = font.glyphs.get(&character) {
                let font::CharInfo {
                    glyph_x,
                    glyph_y,
                    glyph_h,
                    glyph_w,
                    ..
                } = *glyph;
                let w = (glyph_w as f32 / 2.) * scale;
                let h = (glyph_h as f32 / 2.) * scale;
                let texture_x = glyph_x as f32 / width as f32;
                let texture_y = glyph_y as f32 / height as f32;
                let texture_w = glyph_w as f32 / width as f32;
                let texture_h = glyph_h as f32 / height as f32;

                vertices.push(Vertex {
                    pos: Vec2::new(offset - w, -h),
                    uv: Vec2::new(texture_x, texture_y + texture_h),
                });
                vertices.push(Vertex {
                    pos: Vec2::new(offset + w, -h),
                    uv: Vec2::new(texture_x + texture_w, texture_y + texture_h),
                });
                vertices.push(Vertex {
                    pos: Vec2::new(offset + w, h),
                    uv: Vec2::new(texture_x + texture_w, texture_y),
                });
                vertices.push(Vertex {
                    pos: Vec2::new(offset - w, h),
                    uv: Vec2::new(texture_x, texture_y),
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
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);
        renderer.texts.insert(text.string.clone(), (vec![vertex_buffer.clone()], index_buffer.clone()));
    }
}
