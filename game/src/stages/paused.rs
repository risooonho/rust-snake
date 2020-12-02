use crate::{
    components::Input,
    graphics,
    stages::{NextStage, Stage},
};

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
    fn update(&mut self, input: &Input, _renderer: &mut graphics::MainRenderer) -> NextStage {
        println!("{:?}", input);
        if input.go_back {
            return NextStage::Pop;
        }
        NextStage::Noop
    }
}
