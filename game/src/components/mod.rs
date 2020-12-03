use core::str::FromStr;
use glam::{Mat4, Quat, Vec2, Vec3};
use miniquad::date;

use crate::{components, graphics::renderer};

#[derive(Default, Debug, Clone, Copy)]
pub struct Input {
    pub go_left: bool,
    pub go_right: bool,
    pub go_up: bool,
    pub go_down: bool,
    pub go_back: bool,
    pub pause: bool,
    pub width: f32,
    pub height: f32,
    pub resized: bool,
}

impl Input {
    pub fn reset(&mut self) {
        *self = Self {
            width: self.width,
            height: self.height,
            ..Default::default()
        }
    }
}

impl Input {
    pub fn direction(&self) -> Option<Direction> {
        if self.go_left {
            Some(Direction::Left)
        } else if self.go_right {
            Some(Direction::Right)
        } else if self.go_down {
            Some(Direction::Down)
        } else if self.go_up {
            Some(Direction::Up)
        } else {
            None
        }
    }
}

// fn lerp(low: f64, high: f64, value: f64) -> f64 {
//     ((1. - value) * low + high * value).min(high).max(low)
// }

fn inv_lerp(low: f64, high: f64, alpha: f64) -> f64 {
    ( alpha - low ) / ( high - low )
}

// fn remap( original_min: f32, original_max: f32, new_min: f32, new_max: f32, value: f32) -> f32 {
//     let alpha = inv_lerp(original_min, original_max, value);
//     lerp(new_min, new_max, alpha)
// }

#[derive(Debug)]
pub struct Timer {
    start: f64,
    duration: f64,
    paused_time: Option<f64>,
}

impl Timer {
    pub fn new(duration: f64) -> Self {
        Self {
            start: date::now(),
            duration,
            paused_time: None,
        }
    }

    pub fn reset(&mut self) {
        self.start = date::now();
        self.paused_time = None;
    }

    pub fn finished(&self) -> bool {
        let now = date::now();
        return (now - self.start) > self.duration;
    }

    pub fn paused(&mut self) {
        let now = date::now();
        self.paused_time = Some(now);
    }

    pub fn resume(&mut self) {
        let paused_time = match self.paused_time {
            Some(v) => v,
            _ => return,
        };

        let original_alpha = inv_lerp(self.start, self.start + self.duration, paused_time);

        let elasped = paused_time - self.start;
        let now = date::now();
        self.start = now - elasped;

        let new_alpha = inv_lerp(self.start, self.start + self.duration, now);
        assert_eq!(original_alpha, new_alpha);

        self.paused_time = None;
    }

}

#[derive(Copy, Clone)]
pub struct Camera2D {
    pub scale: f32,
    pub view: Mat4,
    pub projection: Mat4,
}

impl Camera2D {
    pub fn new(input: &components::Input, scale: f32) -> Camera2D {
        let components::Input{ width, height, .. } = input;
        let aspect = width / height;
        let projection =
            Mat4::orthographic_rh_gl(-aspect * scale, aspect * scale, -scale, scale, -1., 1.0);
        let view = Mat4::from_rotation_translation(Quat::identity(), Vec3::new(0.0, 0., 0.));

        Camera2D {
            scale,
            view,
            projection,
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        let aspect = width / height;
        #[rustfmt::skip]
        let projection = Mat4::orthographic_rh_gl(
                -aspect * self.scale,
                aspect * self.scale,
                -self.scale,
                self.scale,
                -1.,
                1.0
                );
        self.projection = projection;
    }
}
pub struct Snake;

pub struct Food;

pub struct Tail {
    pub segment: usize,
    pub ahead: hecs::Entity,
}

pub struct Position(pub Vec2);
pub struct Velocity(pub Vec2);

#[derive(Debug, Copy, Clone)]
pub enum CollsionKind {
    Snake,
    Food,
}
pub struct Collision {
    pub kind: CollsionKind,
}

impl Collision {
    pub fn snake() -> Self {
        Self {
            kind: CollsionKind::Snake,
        }
    }

    pub fn food() -> Self {
        Self {
            kind: CollsionKind::Food,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn velocity(&self) -> Vec2 {
        match self {
            Direction::Up => Vec2::new(0., 1.),
            Direction::Right => Vec2::new(1., 0.),
            Direction::Down => Vec2::new(0., -1.),
            Direction::Left => Vec2::new(-1., 0.),
        }
    }

    pub fn update(&mut self, input: &crate::components::Input) {
        if input.go_left {
            if self == &Direction::Right {
                return;
            }
            *self = Direction::Left;
        }
        if input.go_right {
            if self == &Direction::Left {
                return;
            }
            *self = Direction::Right;
        }
        if input.go_down {
            if self == &Direction::Up {
                return;
            }
            *self = Direction::Down;
        }
        if input.go_up {
            if self == &Direction::Down {
                return;
            }
            *self = Direction::Up;
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Up
    }
}

#[derive(Debug, Default)]
pub struct HeadDirection(pub Direction);

pub struct Text {
    string: String,
    font: String,
}

impl Text {
    fn new(str: &str) -> Text {
        Text {
            string: String::from_str(str).expect("Failed to Create Text"),
            font: "KenneyFuture".to_string()
        }
    }

    pub fn create_text<'a>(str: &str) -> (renderer::RenderAssetCommands, components::Text) {
        let text = Text::new(str);
        let cmd = text.load_command();
        (cmd, text)
    }

    pub fn text(&self) -> String {
        self.string.clone()
    }

    pub fn load_command(&self) -> renderer::RenderAssetCommands {
        renderer::RenderAssetCommands::LoadText {
            text: self.string.clone(),
            font: self.font.clone(),
        }
    }

}