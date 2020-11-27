use glam::Vec2;
use miniquad::*;
use smallvec::SmallVec;

mod assets;
mod components;
mod events;
mod graphics;
mod shaders;
mod systems;
mod utils;

pub struct GameWorld {
    pub world: hecs::World,
    pub events: SmallVec<[events::Event; 32]>,
    pub camera: components::Camera2D,
}

struct Stage {
    direction: components::Direction,
    game_world: GameWorld,
    input: components::Input,
    move_timer: components::Timer,
    food_timer: components::Timer,
    renderer: graphics::MainRenderer,
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Self {
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
        ));
        let tail = components::Tail { segment: 1, ahead };

        game_world.world.spawn((
            tail,
            assets::AssetType::Tail,
            components::Collision::snake(),
            components::Position(Vec2::new(0., -1.)),
        ));

        Stage {
            direction: components::Direction::Up,
            game_world,
            renderer,
            move_timer: components::Timer::new(0.75),
            input: components::Input::default(),
            food_timer: components::Timer::new(1.5),
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
        self.renderer.update_view(&self.game_world.camera);

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

    fn draw(&mut self, ctx: &mut Context) {
        systems::gather_render_cmds(&mut self.game_world, &mut self.renderer.render_commands);
        systems::debug_render_cmds(&mut self.game_world, &mut self.renderer.render_commands);
        self.renderer.draw(ctx);
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        UserData::owning(Stage::new(&mut ctx), ctx)
    });
}
