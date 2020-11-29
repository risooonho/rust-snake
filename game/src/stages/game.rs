use miniquad::{Context,  KeyCode, KeyMods};
use smallvec::SmallVec;

use crate::assets;
use crate::components;
use crate::graphics;
use crate::systems;
use crate::Vec2;
use crate::stages::{Stage, Paused, NextStage};

use crate::GameWorld;


pub struct GameState {
    direction: components::Direction,
    game_world: crate::GameWorld,
    input: components::Input,
    move_timer: components::Timer,
    food_timer: components::Timer,
    renderer: graphics::MainRenderer,
}

impl GameState {
    pub fn new(ctx: &mut miniquad::Context) -> Self {
        let renderer = graphics::MainRenderer::new(ctx);
        let mut game_world = GameWorld {
            events: SmallVec::new(),
            camera: components::Camera2D::new(ctx, 20.),
            world: hecs::World::new(),
        };
        systems::create_snake_system(&mut game_world);

        GameState {
            direction: components::Direction::Up,
            game_world,
            renderer,
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
            return NextStage::Push(Box::new(Paused::new()))

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
        self.renderer.update_view(&self.game_world.camera);

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

    fn draw(&mut self, ctx: &mut Context) {
        systems::gather_render_cmds(&mut self.game_world, &mut self.renderer.render_commands);
        systems::debug_render_cmds(&mut self.game_world, &mut self.renderer.render_commands);
        self.renderer.draw(ctx);
    }
}