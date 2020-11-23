mod snake_head;

use glam::Mat4;
use glam::Quat;
use glam::Vec2;
use glam::Vec3;
use miniquad::date;
use miniquad::{Bindings, Context};
use quad_rand as qrand;

use crate::utils::{build_square_texture, make_square, Color};

pub use self::snake_head::SnakeHead;

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

#[derive(PartialEq, Clone, Copy)]
pub enum Food {
    None,
    Food { position: Vec2 },
}

pub struct WorldFood {
    pub bindings: Bindings,
    pub world_food: [Food; 10],
}

impl WorldFood {
    pub fn new_bindings(ctx: &mut Context) -> Bindings {
        let texture = build_square_texture(ctx, 4, Color::purple());
        let (vertex_buffer, index_buffer) = make_square(ctx, 0.8);

        Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        }
    }

    pub fn new(ctx: &mut Context) -> Self {
        let bindings = WorldFood::new_bindings(ctx);

        WorldFood {
            bindings,
            world_food: [
                Food::None,
                Food::None,
                Food::None,
                Food::None,
                Food::None,
                Food::None,
                Food::None,
                Food::None,
                Food::None,
                Food::None,
            ],
        }
    }

    pub fn spawn(&mut self) -> Option<Vec2> {
        if self.count() == self.world_food.len() {
            return None;
        }

        let x = qrand::gen_range(-24, 24);
        let y = qrand::gen_range(-15, 15);
        if let Some(index) = self.world_food.iter().position(|food| food == &Food::None) {
            let position = Vec2::new(x as f32, y as f32);
            self.world_food[index] = Food::Food { position };
            return Some(position)
        }
        return None
    }

    fn count(&self) -> usize {
        self.world_food.iter().fold(0, |mut acc, food| {
            if &Food::None != food {
                acc += 1;
            }
            acc
        });
        0
    }
}


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
