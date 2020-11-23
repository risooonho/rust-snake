use glam::{Mat4, Quat, Vec2, Vec3};
use miniquad::*;

use crate::utils::Color;

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Up
    }
}

pub struct SnakeHead {
    pub direction: Direction,
    pub position: Vec2,
    pub bindings: Bindings,
}
impl SnakeHead {
    pub fn new(ctx: &mut Context) -> SnakeHead {
        let texture = crate::utils::build_square_texture(ctx, 4, Color::ray_white());
        let (vertex_buffer, index_buffer) = crate::utils::make_square(ctx, 1.);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        };
        SnakeHead {
            position: Vec2::new(0., 0.),
            bindings,
            direction: Default::default(),
        }
    }

    pub fn update_direction(&mut self, input: &crate::components::Input) {
        if input.go_left {
            if self.direction == Direction::Right {
                return;
            }
            self.direction = Direction::Left;
        }
        if input.go_right {
            if self.direction == Direction::Left {
                return;
            }
            self.direction = Direction::Right;
        }
        if input.go_down {
            if self.direction == Direction::Up {
                return;
            }
            self.direction = Direction::Down;
        }
        if input.go_up {
            if self.direction == Direction::Down {
                return;
            }
            self.direction = Direction::Up;
        }
    }

    pub fn step(&mut self) {
        let velocicty = self.velocity();
        self.position += velocicty;
    }

    pub fn velocity(&self) -> Vec2 {
        match self.direction {
            Direction::Up => Vec2::new(0., 1.),
            Direction::Right => Vec2::new(1., 0.),
            Direction::Down => Vec2::new(0., -1.),
            Direction::Left => Vec2::new(-1., 0.),
        }
    }

    pub fn draw(&self, ctx: &mut Context, uniform: &mut crate::shaders::sprite::VertexUniforms) {
        uniform.model = self.model();
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(uniform);
        ctx.draw(0, 6, 1);
    }

    pub fn model(&self) -> Mat4 {
        Mat4::from_rotation_translation(
            Quat::from_axis_angle(Vec3::new(0., 0., 1.), 0.),
            Vec3::new(self.position.x, self.position.y, 0.),
        )
    }
}
