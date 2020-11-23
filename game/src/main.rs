use std::collections::HashMap;

use glam::{Mat4, Quat, Vec2, Vec3};
use miniquad::*;
use quad_rand as qrand;

mod components;
mod shaders;
mod utils;

#[derive(PartialEq, Eq, Hash)]
enum AssetType {
    Food,
    Snake,
}

type BindingAssets = HashMap<AssetType, Bindings>;

struct GameWorld {
    pub world: hecs::World,
    pub bindings: BindingAssets,
    pub camera: components::Camera2D,
}

struct Stage {
    game_world: GameWorld,
    input: components::Input,
    snake_head: components::SnakeHead,
    pipeline: Pipeline,
    move_timer: components::Timer,
    food_timer: components::Timer,
}

fn add_food_system(game_world: &mut GameWorld) {
    let GameWorld { world, .. } = game_world;
    let snake_count = world.query::<&components::Food>().iter().count();
    if snake_count >= 10 {
        return;
    }

    let x = qrand::gen_range(-24, 24);
    let y = qrand::gen_range(-15, 15);
    let pos = components::Position(Vec2::new(x as f32, y as f32));
    world.spawn((pos, components::Food));
}
fn render_food_system(game_world: &mut GameWorld, ctx: &mut Context) {
    let GameWorld {
        camera,
        world,
        bindings,
    } = game_world;
    let mut uniform = camera.uniform();
    if let Some(binding) = bindings.get(&AssetType::Food) {
        for (_, (_food, pos)) in &mut world.query::<(&components::Food, &components::Position)>() {
            let model = Mat4::from_rotation_translation(
                Quat::from_axis_angle(Vec3::new(0., 0., 1.), 0.),
                Vec3::new(pos.0.x, pos.0.y, 0.),
            );
            uniform.model = model;
            ctx.apply_bindings(&binding);
            ctx.apply_uniforms(&uniform);
            ctx.draw(0, 6, 1);
        }
    }
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

        let snake_head = components::SnakeHead::new(ctx);
        let mut bindings = HashMap::new();
        let snake_food_binding = components::Food::new_bindings(ctx);
        let snake_bindings = components::Snake::new_bindings(ctx);
        bindings.insert(AssetType::Food, snake_food_binding);
        bindings.insert(AssetType::Snake, snake_bindings);

        let mut game_world = GameWorld {
            camera: components::Camera2D::new(ctx, 20.),
            bindings,
            world: hecs::World::new(),
        };
        game_world.world.spawn((
            components::Snake,
            components::Position(Vec2::new(0., 0.)),
            components::Velocity(Vec2::new(0., 1.)),
        ));

        Stage {
            game_world,
            snake_head,
            pipeline,
            move_timer: components::Timer::new(0.4),
            input: components::Input::default(),
            food_timer: components::Timer::new(1.),
        }
    }
}

impl EventHandler for Stage {
    fn resize_event(&mut self, ctx: &mut Context, _width: f32, _height: f32) {
        self.game_world.camera.resize(ctx);
    }

    fn update(&mut self, _ctx: &mut Context) {
        if self.move_timer.finished() {
            self.snake_head.step();
            self.move_timer.reset();
        } else {
            self.snake_head.update_direction(&self.input);
        }
        if self.food_timer.finished() {
            add_food_system(&mut self.game_world);
            self.food_timer.reset();
        } else {
        }
        self.input = Default::default()
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
        let mut uniform = self.game_world.camera.uniform();

        ctx.begin_default_pass(PassAction::Clear {
            color: Some(utils::Color::dark_gray().into()),
            depth: Some(1.),
            stencil: None,
        });
        ctx.apply_pipeline(&self.pipeline);

        self.snake_head.draw(ctx, &mut uniform);
        render_food_system(&mut self.game_world, ctx);

        ctx.end_render_pass();
        ctx.commit_frame();
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        UserData::owning(Stage::new(&mut ctx), ctx)
    });
}
