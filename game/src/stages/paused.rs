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
        println!("{:?}", input);
        if input.go_back {
            return NextStage::Pop;
        }
        NextStage::Noop
    }
}