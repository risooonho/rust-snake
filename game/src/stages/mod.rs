use miniquad::{Context, EventHandler, KeyCode, KeyMods, MouseButton, TouchPhase};
use smallvec::SmallVec;

use crate::assets;
use crate::components;
use crate::graphics;
use crate::systems;
use crate::Vec2;

use crate::GameWorld;

pub enum NextStage {
    Noop,
    Pop,
}

pub trait Stage {
    fn update(&mut self, _ctx: &mut Context) -> NextStage;
    fn draw(&mut self, _ctx: &mut Context) {}
    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) {}
    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) {}
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) {}
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }

    fn char_event(
        &mut self,
        _ctx: &mut Context,
        _character: char,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {}

    fn touch_event(&mut self, ctx: &mut Context, phase: TouchPhase, _id: u64, x: f32, y: f32) {
        if phase == TouchPhase::Started {
            self.mouse_button_down_event(ctx, MouseButton::Left, x, y);
        }

        if phase == TouchPhase::Ended {
            self.mouse_button_up_event(ctx, MouseButton::Left, x, y);
        }

        if phase == TouchPhase::Moved {
            self.mouse_motion_event(ctx, x, y);
        }
    }

    fn raw_mouse_motion(&mut self, _ctx: &mut Context, _dx: f32, _dy: f32) {}
    fn quit_requested_event(&mut self, _ctx: &mut Context) {}
}

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
        let ahead = game_world.world.spawn((
            components::Snake,
            assets::AssetType::Snake,
            components::Position(Vec2::new(0., 0.)),
            components::Velocity(Vec2::new(0., 1.)),
            components::HeadDirection::default(),
        ));
        let tail = components::Tail { segment: 1, ahead };

        game_world.world.spawn((
            tail,
            assets::AssetType::Tail,
            components::Collision::snake(),
            components::Position(Vec2::new(0., -1.)),
        ));

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
    fn resize_event(&mut self, ctx: &mut Context, _width: f32, _height: f32) {
        self.game_world.camera.resize(ctx);
    }

    fn update(&mut self, _ctx: &mut Context) -> NextStage {
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
        systems::game_over_system(&mut self.game_world);
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
            _ => {}
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        systems::gather_render_cmds(&mut self.game_world, &mut self.renderer.render_commands);
        systems::debug_render_cmds(&mut self.game_world, &mut self.renderer.render_commands);
        self.renderer.draw(ctx);
    }
}
