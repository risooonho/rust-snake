use glam::Vec2;
use miniquad::*;
use smallvec::SmallVec;
use stages::GameState;

mod assets;
mod components;
mod events;
mod graphics;
mod shaders;
mod systems;
mod utils;
mod stages;


pub struct GameWorld {
    pub world: hecs::World,
    pub events: SmallVec<[events::Event; 32]>,
    pub camera: components::Camera2D,
}

struct Stage {
    stages: Vec<Box<dyn stages::Stage>>,
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Self {
        let mut stages = Vec::with_capacity(8);
        let game_stage = Box::new(GameState::new(ctx));
        stages.push(game_stage as Box<dyn stages::Stage>);
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
        let stage = match self.stages.last_mut() {
            Some(s) => s,
            _ => return
        };
        match stage.update(ctx) {
            _ => {}
        }
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
