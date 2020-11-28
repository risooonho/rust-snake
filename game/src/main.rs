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

struct GameState {
    direction: components::Direction,
    game_world: GameWorld,
    input: components::Input,
    move_timer: components::Timer,
    food_timer: components::Timer,
    renderer: graphics::MainRenderer,
}

impl GameState {
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

impl EventHandler for GameState {
    fn resize_event(&mut self, ctx: &mut Context, _width: f32, _height: f32) {
        self.game_world.camera.resize(ctx);
    }

    fn update(&mut self, _ctx: &mut Context) {
        self.direction.update(&self.input);
        systems::update_input(&mut self.game_world, &self.input);
        if self.move_timer.finished() {
            systems::update_velocity_direction(&mut self.game_world);
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


pub struct GameWorld {
    pub world: hecs::World,
    pub events: SmallVec<[events::Event; 32]>,
    pub camera: components::Camera2D,
}

struct Stage {
    stages: Vec<Box<dyn EventHandler>>,
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Self {
        let mut stages = Vec::with_capacity(8);
        let game_stage = Box::new(GameState::new(ctx));
        stages.push(game_stage as Box<dyn EventHandler>);
        Self {
            stages
        }
    }
}


impl EventHandler for Stage {
    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let stage = match self.stages.get_mut(0) {
            Some(s) => s,
            None => return,
        };
        stage.resize_event(ctx, width, height)
    }

    fn update(&mut self, ctx: &mut Context) {
        let stage = match self.stages.get_mut(0) {
            Some(s) => s,
            _ => return
        };
        stage.update(ctx);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        repeat: bool,
    ) {
        let stage = match self.stages.get_mut(0) {
            Some(s) => s,
            _ => return,
        };
        stage.key_down_event(ctx, keycode, keymods, repeat);
    }

    fn draw(&mut self, ctx: &mut Context) {
        let stage = match self.stages.get_mut(0) {
            Some(s) => s,
            _ => return,
        };
        stage.draw(ctx);
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        let stage = match self.stages.get_mut(0) {
            Some(s) => s,
            _ => return,
        };
        stage.mouse_motion_event(ctx, x, y);
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        let stage = match self.stages.get_mut(0) {
            Some(s) => s,
            _ => return,
        };
        stage.mouse_wheel_event(ctx, x, y);
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        let stage = match self.stages.get_mut(0) {
            Some(s) => s,
            _ => return,
        };
        stage.mouse_button_down_event(ctx, button, x, y);
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        let stage = match self.stages.get_mut(0) {
            Some(s) => s,
            _ => return,
        };
        stage.mouse_button_up_event(ctx, button, x, y);
    }

    fn char_event(
        &mut self,
        ctx: &mut Context,
        character: char,
        keymods: KeyMods,
        repeat: bool,
    ) {
        let stage = match self.stages.get_mut(0) {
            Some(s) => s,
            _ => return,
        };
        stage.char_event(ctx, character, keymods, repeat);
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        let stage = match self.stages.get_mut(0) {
            Some(s) => s,
            _ => return,
        };
        stage.key_up_event(ctx, keycode, keymods);

    }

    fn raw_mouse_motion(&mut self, ctx: &mut Context, dx: f32, dy: f32) {
        let stage = match self.stages.get_mut(0) {
            Some(s) => s,
            _ => return,
        };
        stage.raw_mouse_motion(ctx, dx, dy);
    }

    fn quit_requested_event(&mut self, ctx: &mut Context) {
        let stage = match self.stages.get_mut(0) {
            Some(s) => s,
            _ => return,
        };
        stage.quit_requested_event(ctx);
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        UserData::owning(Stage::new(&mut ctx), ctx)
    });
}
