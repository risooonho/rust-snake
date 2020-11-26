mod snake_head;

use glam::Vec2;
use glam::Mat4;
use glam::Quat;
use glam::Vec3;
use miniquad::date;
use miniquad::Context;


pub use self::snake_head::Direction;

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

#[derive(Copy, Clone)]
pub struct Camera2D {
    scale: f32,
    view: Mat4,
    projection: Mat4,
}

impl Camera2D {
    pub fn new(ctx: &mut Context, scale: f32) -> Camera2D {
        let (width, height) = ctx.screen_size();
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

    pub fn resize(&mut self, ctx: &mut Context) {
        let (width, height) = ctx.screen_size();
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

    pub fn uniform(&self) -> crate::shaders::sprite::VertexUniforms {
        crate::shaders::sprite::VertexUniforms {
            projection: self.projection,
            view: self.view,
            model: Mat4::identity(),
        }
    }
}
pub struct Snake;

impl Snake {
    pub fn new_bindings(ctx: &mut Context) -> miniquad::Bindings {
        let texture = crate::utils::build_square_texture(ctx, 4, crate::graphics::colors::RAYWHITE);
        let (vertex_buffer, index_buffer) = crate::utils::make_square(ctx, 1.);

        miniquad::Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        }
    }
}

pub struct Food;

impl Food {
    pub fn new_bindings(ctx: &mut Context) -> miniquad::Bindings {
        let texture = crate::utils::build_square_texture(ctx, 4, crate::graphics::colors::PURPLE);
        let (vertex_buffer, index_buffer) = crate::utils::make_square(ctx, 0.8);

        miniquad::Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        }
    }
}

pub struct Tail {
    pub segment: usize,
    pub ahead: hecs::Entity,
}


impl Tail {
    pub fn new_bindings(ctx: &mut Context) -> miniquad::Bindings {
        let texture = crate::utils::build_square_texture(ctx, 4, crate::graphics::colors::RAYWHITE);
        let (vertex_buffer, index_buffer) = crate::utils::make_square(ctx, 0.8);

        miniquad::Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        }
    }
}
pub struct Position( pub Vec2);
pub struct Velocity( pub Vec2);

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