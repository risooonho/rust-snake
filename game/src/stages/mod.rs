use miniquad::{Context, KeyCode, KeyMods, MouseButton, TouchPhase};
pub mod game;
pub mod paused;

use crate::{components::Input, graphics};
pub use game::GameState;
pub use paused::Paused;

pub type StageStack = Vec<Box<dyn Stage>>;

pub fn new_stage_stack(capacity: usize) -> StageStack {
    Vec::with_capacity(capacity)
}

pub enum NextStage {
    Noop,
    Pop,
    Push(Box<dyn Stage>),
}

pub trait Stage {
    fn enter(&mut self, _ctx: &mut Context) {}
    fn exit(&mut self, _ctx: &mut Context) {}
    fn update(&mut self, input: &Input, _ctx: &mut Context) -> NextStage;
    fn draw(&mut self, _ctx: &mut Context, _renderer: &mut graphics::MainRenderer) {}
    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) {}
    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) {}
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) {}
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }

    fn char_event(
        &mut self,
        _ctx: &mut Context,
        _character: char,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {}

    fn touch_event(&mut self, ctx: &mut Context, phase: TouchPhase, _id: u64, x: f32, y: f32) {
        if phase == TouchPhase::Started {
            self.mouse_button_down_event(ctx, MouseButton::Left, x, y);
        }

        if phase == TouchPhase::Ended {
            self.mouse_button_up_event(ctx, MouseButton::Left, x, y);
        }

        if phase == TouchPhase::Moved {
            self.mouse_motion_event(ctx, x, y);
        }
    }

    fn raw_mouse_motion(&mut self, _ctx: &mut Context, _dx: f32, _dy: f32) {}
    fn quit_requested_event(&mut self, _ctx: &mut Context) {}
}
