use miniquad::{Buffer, BufferType, Context, KeyCode, KeyMods};
use smallvec::SmallVec;

use crate::components;
use crate::graphics;
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
            events: SmallVec::new(),
            camera: components::Camera2D::new(ctx, 20.),
            world: hecs::World::new(),
        };
        systems::create_snake_system(&mut game_world);
        game_world.world.spawn((
            components::Position(glam::Vec2::new(-10., -4.)),
            components::Text::new("Test"),
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

    fn update(&mut self, _ctx: &mut Context) -> NextStage {
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

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        repeat: bool,
    ) {
        if repeat {
            return;
        }

        match keycode {
            KeyCode::Left | KeyCode::A => {
                self.input.go_left = true;
            }
            KeyCode::Right | KeyCode::D => {
                self.input.go_right = true;
            }
            KeyCode::Up | KeyCode::W => {
                self.input.go_up = true;
            }
            KeyCode::Down | KeyCode::S => {
                self.input.go_down = true;
            }
            KeyCode::Escape => {
                self.input.pause = true;
            }
            _ => {}
        }
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
    if renderer.font_mesh.is_some() {
        return;
    }
    use crate::shaders::Vertex;
    use glam::Vec2;

    let GameWorld { world, .. } = game_world;
    for (_, (text, _pos)) in &mut world.query::<(&components::Text, &components::Position)>() {
        let mut vertices: Vec<Vertex> = Vec::with_capacity(text.string.chars().count() * 4);
        let mut indices: Vec<u16> = Vec::with_capacity(text.string.chars().count() * 6);
        // let width = renderer.example_font.font_image.width;
        // let height = renderer.example_font.font_image.height;
        let mut offset = 0.0f32;
        let scale = 0.1f32;
        for (index, character) in text.string.chars().enumerate() {
            let index = index as u16;
            if let Some(glyph) = renderer.example_font.glyphs.get(&character) {
                println!("{:?}", glyph);
                let w = (glyph.glyph_w as f32 / 2.) * scale;
                let h = (glyph.glyph_h as f32 / 2.) * scale;
                vertices.push(Vertex {
                    pos: Vec2::new(offset - w, -h),
                    uv: Vec2::new(0., 1.),
                });
                vertices.push(Vertex {
                    pos: Vec2::new(offset + w, -h),
                    uv: Vec2::new(1., 1.),
                });
                vertices.push(Vertex {
                    pos: Vec2::new(offset + w, h),
                    uv: Vec2::new(1., 0.),
                });
                vertices.push(Vertex {
                    pos: Vec2::new(offset - w, h),
                    uv: Vec2::new(0., 0.),
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
        renderer.font_mesh = Some((vec![vertex_buffer], index_buffer));
    }
}
