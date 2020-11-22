use crate::shaders::Vertex;
use glam::{Mat4, Quat, Vec2, Vec3};
use miniquad::*;

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
        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos: Vec2::new(-0.5,  -0.5 ), uv: Vec2::new( 0.,  0. ) },
            Vertex { pos: Vec2::new( 0.5,  -0.5 ), uv: Vec2::new( 1., 0. ) },
            Vertex { pos: Vec2::new( 0.5,   0.5 ), uv: Vec2::new( 1., 1. ) },
            Vertex { pos: Vec2::new(-0.5,   0.5 ), uv: Vec2::new( 0., 1. ) },
        ];
        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let pixels: [u8; 4 * 4 * 4] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
            0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ];

        let texture = Texture::from_rgba8(ctx, 4, 4, &pixels);
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

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
        match self.direction {
            Direction::Up => self.position += Vec2::new(0., 1.),
            Direction::Right => self.position += Vec2::new(1., 0.),
            Direction::Down => self.position += Vec2::new(0., -1.),
            Direction::Left => self.position += Vec2::new(-1., 0.),
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
