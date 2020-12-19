use miniquad::*;
use stages::GameState;

mod components;
mod events;
mod graphics;
mod shaders;
mod stages;
mod systems;
mod utils;
mod types;
mod ui;

use types::Rect;
pub use types::{AssetIdentity, Color};
use ui::DrawCommand;

struct SnakeGame {
    stages: stages::StageStack,
    renderer: graphics::MainRenderer,
    input: components::Input,
    ui: ui::Ui,
}

impl SnakeGame {
    pub fn new(ctx: Context) -> Self {
        let (width, height) = ctx.screen_size();
        let mut renderer = graphics::MainRenderer::new(ctx);

        let mut input = components::Input::default();
        input.width = width;
        input.height = height;
        let mut stages = stages::new_stage_stack(16);
        let init_state = GameState::new(&input, &mut renderer);
        let game_stage = Box::new(init_state);
        let ui = ui::Ui::new();

        stages.push(game_stage as Box<dyn stages::Stage>);

        SnakeGame {
            stages,
            renderer,
            input,
            ui,
        }
    }
}

impl EventHandlerFree for SnakeGame {
    fn resize_event(&mut self, width: f32, height: f32) {
        self.input.width = width;
        self.input.height = height;
        self.input.resized = true;
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

        self.ui.push_cmd(DrawCommand::draw_rect(Rect::new(glam::Vec2::new(30., 30.), glam::Vec2::new(50., 50.)) , types::colors::GOLD));
        self.renderer.load_assets();
        self.input.reset();
    }

    fn draw(&mut self) {
        for stage in self.stages.iter_mut() {
            stage.draw(&mut self.renderer);
        }
        self.renderer.draw();
        self.ui.draw(&mut self.renderer);
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
