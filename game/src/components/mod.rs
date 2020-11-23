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
    pub fn new(ctx: &mut Context) -> Self {
        let texture = build_square_texture(ctx, 4, Color::purple());
        let (vertex_buffer, index_buffer) = make_square(ctx, 0.8);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        };

        WorldFood {
            bindings,
            world_food: [
                Food::Food {
                    position: Vec2::new(2., 2.),
                },
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

    pub fn draw(
        &self,
        ctx: &mut Context,
        uniform: &mut crate::shaders::sprite::VertexUniforms,
    ) {
        self.world_food.iter().for_each(move |food| {
            if let Food::Food { position } = food {
                let model = Mat4::from_rotation_translation(
                    Quat::from_axis_angle(Vec3::new(0., 0., 1.), 0.),
                    Vec3::new(position.x, position.y, 0.),
                );
                uniform.model = model;
                ctx.apply_bindings(&self.bindings);
                ctx.apply_uniforms(uniform);
                ctx.draw(0, 6, 1);
            }
        });
    }

    // pub fn spawn(&mut self) {
    //     if self.count() == self.world_food.len() {
    //         return;
    //     }

    //     let x = qrand::gen_range(-24, 24);
    //     let y = qrand::gen_range(-15, 15);
    //     if let Some(index) = self.world_food.iter().position(|food| food != &Food::None) {
    //         self.world_food[index] = Food::Food { position: Vec2::new(x as f32, y as f32) };
    //     }
    // }

    // pub fn count(&self) -> usize {
    //     self.world_food.iter().fold(0, |mut acc, food| {
    //         if &Food::None != food {
    //             acc += 1;
    //         }
    //         acc
    //     });
    //     0
    // }
}
