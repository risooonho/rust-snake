use components::Input;
use miniquad::Context;

use crate::{graphics::renderer::RenderAssetCommands, components};
use crate::graphics::{self};
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
    fn enter(&mut self) {
        self.move_timer.resume();
        self.food_timer.resume();
    }

    fn exit(&mut self) {
        self.move_timer.paused();
        self.food_timer.paused();
    }

    fn update(&mut self, input: &Input, _ctx: &mut Context) -> NextStage {
        self.input = input.clone();
        if self.input.resized {
            let Input{ width, height, .. } = self.input;
            self.game_world.camera.resize(width, height);
        }
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

        self.input.reset();
        self.game_world.events.clear();
        NextStage::Noop
    }

    fn draw(&mut self, _ctx: &mut Context, renderer: &mut graphics::MainRenderer) {
        renderer.update_view(&self.game_world.camera);
        systems::gather_render_cmds(&mut self.game_world, &mut renderer.render_commands);
        systems::debug_render_cmds(&mut self.game_world, &mut renderer.render_commands);
        draw_text(&mut self.game_world, renderer);
    }
}

fn draw_text(game_world: &mut GameWorld, renderer: &mut graphics::MainRenderer) {
    let GameWorld { world, .. } = game_world;

    for (_, (text, pos)) in &mut world.query::<(&components::Text, &components::Position)>() {
        if let Some(_) = renderer.texts.get(&text.string) {
            let cmd = graphics::renderer::RenderFontCommand {
                font: "KenneyFuture".to_string(),
                text: text.string.clone(),
                position: pos.0,
            };
            renderer.render_font_commands.push(cmd);
            continue;
        }

        renderer.asset_commands.push(RenderAssetCommands::LoadText { text: text.string.clone(), font: "KenneyFuture".to_string()});
    }
}
