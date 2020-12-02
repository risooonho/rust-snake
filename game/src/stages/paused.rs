use crate::{components::Input, stages::{Stage, NextStage}};

#[derive(Debug, Copy, Clone, Default)]
pub struct Paused {
    go_back: bool,
}

impl Paused {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Stage for Paused {
    fn update(&mut self, input: &Input, _: &mut miniquad::Context) -> NextStage {
        if self.go_back {
            return NextStage::Pop;
        }
        NextStage::Noop
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut miniquad::Context,
        keycode: miniquad::KeyCode,
        _keymods: miniquad::KeyMods,
        repeat: bool,
    ) {
        if repeat {
            return;
        }
        if keycode == miniquad::KeyCode::Escape {
            self.go_back = true;
        }
    }
}