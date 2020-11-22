mod snake_head;

pub use self::snake_head::SnakeHead;
use miniquad::date;

#[derive(Default)]
pub struct Input {
    pub go_left: bool,
    pub go_right: bool,
    pub go_up: bool,
    pub go_down: bool,
}

pub struct Timer {
    start: f64,
    duration: f64,
}

impl Timer {
    pub fn new(duration: f64) -> Self {
        Self {
            start: date::now(),
            duration,
        }
    }

    pub fn reset(&mut self) {
        self.start = date::now();
    }

    pub fn finished(&self) -> bool {
        let now = date::now();
        return (now - self.start) > self.duration;
    }
}
