use std::collections::HashMap;

use glam::Vec2;
use miniquad::*;
use smallvec::SmallVec;

mod components;
mod events;
mod shaders;
mod utils;
mod systems;
mod assets;

pub struct GameWorld {
    pub world: hecs::World,
    pub events: SmallVec<[events::Event; 32]>,
    pub bindings: assets::BindingAssets,
    pub camera: components::Camera2D,
}

struct Stage {
    direction: components::Direction,
    game_world: GameWorld,
    input: components::Input,
    pipeline: Pipeline,
    move_timer: components::Timer,
    food_timer: components::Timer,
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Self {
        let shader = shaders::sprite::new(ctx).unwrap();

        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
        );

        let mut bindings = HashMap::new();
        let snake_food_binding = components::Food::new_bindings(ctx);
        let snake_bindings = components::Snake::new_bindings(ctx);
        let tail_bindings = components::Tail::new_bindings(ctx);
        bindings.insert(assets::AssetType::Food, snake_food_binding);
        bindings.insert(assets::AssetType::Snake, snake_bindings);
        bindings.insert(assets::AssetType::Tail, tail_bindings);

        let mut game_world = GameWorld {
            events: SmallVec::new(),
            camera: components::Camera2D::new(ctx, 20.),
            bindings,
            world: hecs::World::new(),
        };
        let ahead = game_world.world.spawn((
            components::Snake,
            components::Position(Vec2::new(0., 0.)),
            components::Velocity(Vec2::new(0., 1.)),
        ));
        let tail = components::Tail{
            segment: 1,
            ahead,
        };

        game_world.world.spawn((
            tail,
            components::Collision::snake(),
           components::Position(Vec2::new(0., -1.)),
        ));

        Stage {
            direction: components::Direction::Up,
            game_world,
            pipeline,
            move_timer: components::Timer::new(0.25),
            input: components::Input::default(),
            food_timer: components::Timer::new(0.5),
        }
    }
}

impl EventHandler for Stage {
    fn resize_event(&mut self, ctx: &mut Context, _width: f32, _height: f32) {
        self.game_world.camera.resize(ctx);
    }

    fn update(&mut self, _ctx: &mut Context) {
        self.direction.update(&self.input);
        if self.move_timer.finished() {
            systems::tail_movement_system(&mut self.game_world);
            systems::head_collision_system(&mut self.game_world);
            systems::handle_collision_system(&mut self.game_world);
            systems::trigger_tail_spawn(&mut self.game_world);
            systems::movement_system(&mut self.game_world);
            systems::spawn_tail_system(&mut self.game_world);
            self.move_timer.reset();
        } else {
            let direction = self.direction.velocity();
            systems::update_head_direction(&mut self.game_world, direction);
        }
        if self.food_timer.finished() {
            systems::add_food_system(&mut self.game_world);
            self.food_timer.reset();
        }

        systems::despawn_food_system(&mut self.game_world);
        systems::game_over_system(&mut self.game_world);
        self.input = Default::default();
        self.game_world.events.clear();
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

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {}

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(PassAction::Clear {
            color: Some(utils::Color::dark_gray().into()),
            depth: Some(1.),
            stencil: None,
        });
        ctx.apply_pipeline(&self.pipeline);

        systems::render_food_system(&mut self.game_world, ctx);
        systems::render_snake_system(&mut self.game_world, ctx);
        systems::render_tail_system(&mut self.game_world, ctx);

        ctx.end_render_pass();
        ctx.commit_frame();
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        UserData::owning(Stage::new(&mut ctx), ctx)
    });
}
