use glam::Vec2;

#[derive(PartialEq)]
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