use miniquad::*;
use stages::GameState;


mod assets;
mod components;
mod events;
mod graphics;
mod shaders;
mod stages;
mod systems;
mod utils;

struct Stage {
    stages: stages::StageStack,
    renderer: graphics::MainRenderer,
    input: components::Input,
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Self {
        let mut renderer = graphics::MainRenderer::new(ctx);

        let (width, height) = ctx.screen_size();
        let mut input = components::Input::default();
        input.width = width;
        input.height = height;
        let mut stages = stages::new_stage_stack(16);
        let game_stage = Box::new(GameState::new(&input, &mut renderer.asset_commands));

        stages.push(game_stage as Box<dyn stages::Stage>);


        Self { stages, renderer, input }
    }
}

impl EventHandler for Stage {
    fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32) {
        self.input.width = width;
        self.input.height = height;
        self.input.resized = true;
    }

    fn update(&mut self, ctx: &mut Context) {
        let stage = match self.stages.last_mut() {
            Some(s) => s,
            _ => return,
        };
        let next_stage = stage.update(&self.input, &mut self.renderer);
        match next_stage {
            stages::NextStage::Push(mut new_stage) => {
                stage.exit();
                new_stage.enter();
                self.stages.push(new_stage);
            }
            stages::NextStage::Pop => {
                stage.exit();
                self.stages.pop().expect("Popped an Empty StageStack");
                match self.stages.last_mut() {
                    Some(s) => s.enter(),
                    _ => {}
                };
            }
            _ => {}
        };
        self.input.reset();
        self.renderer.load_assets(ctx);
    }

    fn draw(&mut self, ctx: &mut Context) {
        for stage in self.stages.iter_mut() {
            stage.draw(ctx, &mut self.renderer);
        }
        self.renderer.draw(ctx);
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
                self.input.go_back = true;
            }
            _ => {}
        }
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        UserData::owning(Stage::new(&mut ctx), ctx)
    });
}
