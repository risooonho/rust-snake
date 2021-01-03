use graphics::ui;
use miniquad::*;
use stages::GameState;
use std::time::Instant;

mod components;
mod events;
mod graphics;
mod shaders;
mod stages;
mod systems;
mod types;
mod utils;

pub use types::{AssetIdentity, Color};

struct SnakeGame {
    input: components::Input,
    painter: ui::MegaUI,
    renderer: graphics::MainRenderer,
    stages: stages::StageStack,
    start_time: Instant,
}

impl SnakeGame {
    pub fn new(mut ctx: Context) -> Self {

        let painter = ui::MegaUI::new(&mut ctx);
        let mut renderer = graphics::MainRenderer::new(ctx);

        let mut input = components::Input::default();
        let mut stages = stages::new_stage_stack(16);
        let init_state = GameState::new(&input, &mut renderer);
        let game_stage = Box::new(init_state);

        stages.push(game_stage as Box<dyn stages::Stage>);

        SnakeGame {
            painter,
            start_time: Instant::now(),
            stages,
            renderer,
            input,
        }
    }
}

impl EventHandlerFree for SnakeGame {
    fn resize_event(&mut self, width: f32, height: f32) {

        self.input.width = width;
        self.input.height = height;
        self.input.resized = true;
    }

    fn mouse_button_up_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {
    }

    fn mouse_button_down_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
    }

    fn update(&mut self) {
        let stage = match self.stages.last_mut() {
            Some(s) => s,
            _ => return,
        };
        let next_stage = stage.update(&self.input, &mut self.renderer);
        match next_stage {
            stages::NextStage::Push(mut new_stage) => {
                stage.exit(&mut self.renderer);
                new_stage.enter(&mut self.renderer);
                self.stages.push(new_stage);
            }
            stages::NextStage::Pop => {
                stage.exit(&mut self.renderer);
                self.stages.pop().expect("Popped an Empty StageStack");
                match self.stages.last_mut() {
                    Some(s) => s.enter(&mut self.renderer),
                    _ => {}
                };
            }
            _ => {}
        };

        self.renderer.load_assets();
        self.input.reset();
    }

    fn draw(&mut self) {
        for stage in self.stages.iter_mut() {
            stage.draw(&mut self.renderer);
        }
        let ctx = &mut self.renderer.ctx;
        ctx.clear(Some((1., 1., 1., 1.)), None, None);
        ctx.begin_default_pass(miniquad::PassAction::clear_color(0., 0., 0., 1.));
        ctx.end_render_pass();

        // self.renderer.draw();
    }

    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, repeat: bool) {
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
                self.input.go_back = true;
            }
            _ => {}
        }
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |ctx| {
        UserData::free(SnakeGame::new(ctx))
    });
}
