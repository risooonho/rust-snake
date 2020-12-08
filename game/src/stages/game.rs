use components::Input;
use graphics::renderer;

use crate::graphics::{self};
use crate::stages::{NextStage, Paused, Stage};
use crate::systems::{self, GameWorld};
use crate::{components, graphics::renderer::MaterialAsset};

pub struct GameState {
    direction: components::Direction,
    game_world: GameWorld,
    move_timer: components::Timer,
    food_timer: components::Timer,
    score: i32,
}

impl GameState {
    pub fn new(input: &components::Input, renderer: &mut renderer::MainRenderer) -> Self {
        let mut game_world = GameWorld {
            events: Vec::with_capacity(32),
            camera: components::Camera2D::new(input, 20.),
            world: hecs::World::new(),
        };
        systems::create_snake_system(&mut game_world);
        let (load_cmd, text_component) =
            components::Text::create_text(format!("Score:  {}", 0).as_str());

        game_world.world.spawn((
            components::Score,
            components::Position(glam::Vec2::new(-24., 18.)),
            text_component,
        ));
        renderer.asset_commands.push(load_cmd);

        let snake_texture = crate::utils::build_square_texture(
            &mut renderer.ctx,
            4,
            crate::graphics::colors::RAYWHITE,
        );
        let tail_texture = crate::utils::build_square_texture(
            &mut renderer.ctx,
            4,
            crate::graphics::colors::RAYWHITE,
        );
        let food_texture = crate::utils::build_square_texture(
            &mut renderer.ctx,
            4,
            crate::graphics::colors::PURPLE,
        );
        let arrow_texture =
            crate::utils::build_square_texture(&mut renderer.ctx, 4, crate::graphics::colors::RED);

        renderer.add_material("Snake", vec![snake_texture]);
        renderer.add_material("Food", vec![food_texture]);
        renderer.add_material("Tail", vec![tail_texture]);
        renderer.add_material("Arrow", vec![arrow_texture]);

        let snake_mesh = crate::utils::make_square_raw(1.);
        let food_mesh = crate::utils::make_square_raw(0.8);
        let tail_mesh = crate::utils::make_square_raw(0.8);
        let arrow_mesh = crate::utils::make_arrow_raw();

        renderer.add_mesh("Food", &food_mesh.0, &food_mesh.1);
        renderer.add_mesh("Tail", &tail_mesh.0, &tail_mesh.1);
        renderer.add_mesh("Snake", &snake_mesh.0, &snake_mesh.1);
        renderer.add_mesh("Arrow", &arrow_mesh.0, &arrow_mesh.1);

        GameState {
            direction: components::Direction::Up,
            game_world,
            move_timer: components::Timer::new(0.25),
            food_timer: components::Timer::new(1.5),
            score: 0,
        }
    }
}

impl Stage for GameState {
    fn enter(&mut self, _: &mut graphics::MainRenderer) {
        self.move_timer.resume();
        self.food_timer.resume();
    }

    fn exit(&mut self, _: &mut graphics::MainRenderer) {
        self.move_timer.paused();
        self.food_timer.paused();
    }

    fn update(&mut self, input: &Input, renderer: &mut graphics::MainRenderer) -> NextStage {
        let input = input.clone();
        if input.resized {
            let Input { width, height, .. } = input;
            self.game_world.camera.resize(width, height);
        }
        if input.pause {
            return NextStage::Push(Box::new(Paused::new()));
        }
        self.direction.update(&input);
        systems::update_input(&mut self.game_world, &input);
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
        systems::update_score_system(
            &mut self.game_world,
            &mut self.score,
            &mut renderer.asset_commands,
        );
        if systems::game_over_system(&mut self.game_world) {
            self.move_timer.reset();
            self.food_timer.reset();
        }

        self.game_world.events.clear();
        NextStage::Noop
    }

    fn draw(&mut self, renderer: &mut graphics::MainRenderer) {
        renderer.update_view(&self.game_world.camera);
        systems::gather_render_cmds(&mut self.game_world, renderer);
        systems::debug_render_cmds(&mut self.game_world, renderer);
        systems::draw_text(&mut self.game_world, renderer);
    }
}
