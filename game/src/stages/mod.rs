use miniquad::Context;
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
    fn enter(&mut self, _renderer: &mut graphics::MainRenderer) {}
    fn exit(&mut self, _renderer: &mut graphics::MainRenderer) {}
    fn update(&mut self, input: &Input, _renderer: &mut graphics::MainRenderer) -> NextStage;
    fn draw(&mut self, _ctx: &mut Context, _renderer: &mut graphics::MainRenderer) {}
}
