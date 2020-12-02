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
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Self {
        let renderer = graphics::MainRenderer::new(ctx);
        let mut stages = stages::new_stage_stack(16);
        let game_stage = Box::new(GameState::new(ctx));
        stages.push(game_stage as Box<dyn stages::Stage>);



        Self { stages, renderer }
    }
}

impl EventHandler for Stage {
    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let stage = match self.stages.last_mut() {
            Some(s) => s,
            None => return,
        };
        stage.resize_event(ctx, width, height)
    }

    fn update(&mut self, ctx: &mut Context) {
        let stage = match self.stages.last_mut() {
            Some(s) => s,
            _ => return,
        };
        let next_stage = stage.update(ctx);
        match next_stage {
            stages::NextStage::Push(mut new_stage) => {
                stage.exit(ctx);
                new_stage.enter(ctx);
                self.stages.push(new_stage);
            }
            stages::NextStage::Pop => {
                stage.exit(ctx);
                self.stages.pop().expect("Popped an Empty StageStack");
                match self.stages.last_mut() {
                    Some(s) => s.enter(ctx),
                    _ => {}
                };
            }
            _ => {}
        };
    }

    fn draw(&mut self, ctx: &mut Context) {
        let stage = match self.stages.last_mut() {
            Some(s) => s,
            _ => return,
        };
        stage.draw(ctx, &mut self.renderer);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        repeat: bool,
    ) {
        let stage = match self.stages.last_mut() {
            Some(s) => s,
            _ => return,
        };
        stage.key_down_event(ctx, keycode, keymods, repeat);
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        let stage = match self.stages.last_mut() {
            Some(s) => s,
            _ => return,
        };
        stage.mouse_motion_event(ctx, x, y);
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        let stage = match self.stages.last_mut() {
            Some(s) => s,
            _ => return,
        };
        stage.mouse_wheel_event(ctx, x, y);
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let stage = match self.stages.last_mut() {
            Some(s) => s,
            _ => return,
        };
        stage.mouse_button_down_event(ctx, button, x, y);
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let stage = match self.stages.last_mut() {
            Some(s) => s,
            _ => return,
        };
        stage.mouse_button_up_event(ctx, button, x, y);
    }

    fn char_event(&mut self, ctx: &mut Context, character: char, keymods: KeyMods, repeat: bool) {
        let stage = match self.stages.last_mut() {
            Some(s) => s,
            _ => return,
        };
        stage.char_event(ctx, character, keymods, repeat);
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        let stage = match self.stages.last_mut() {
            Some(s) => s,
            _ => return,
        };
        stage.key_up_event(ctx, keycode, keymods);
    }

    fn raw_mouse_motion(&mut self, ctx: &mut Context, dx: f32, dy: f32) {
        let stage = match self.stages.last_mut() {
            Some(s) => s,
            _ => return,
        };
        stage.raw_mouse_motion(ctx, dx, dy);
    }

    fn quit_requested_event(&mut self, ctx: &mut Context) {
        let stage = match self.stages.last_mut() {
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
