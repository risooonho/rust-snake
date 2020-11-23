use glam::{Mat4, Quat, Vec3};
use miniquad::*;

mod components;
mod shaders;
mod utils;

struct Camera2D {
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

    pub fn uniform(&self) -> shaders::sprite::VertexUniforms {
        shaders::sprite::VertexUniforms {
            projection: self.projection,
            view: self.view,
            model: Mat4::identity(),
        }
    }
}

struct Stage {
    camera: Camera2D,
    input: components::Input,
    snake_head: components::SnakeHead,
    pipeline: Pipeline,
    move_timer: components::Timer,
    food: components::WorldFood,
    food_timer: components::Timer,
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Self {
        let shader = shaders::sprite::new(ctx).unwrap();

        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
        );

        let snake_head = components::SnakeHead::new(ctx);

        Stage {
            camera: Camera2D::new(ctx, 20.),
            snake_head,
            pipeline,
            move_timer: components::Timer::new(0.4),
            input: components::Input::default(),
            food: components::WorldFood::new(ctx),
            food_timer: components::Timer::new(1.),
        }
    }
}

impl EventHandler for Stage {
    fn resize_event(&mut self, ctx: &mut Context, _width: f32, _height: f32) {
        self.camera.resize(ctx);

    }

    fn update(&mut self, _ctx: &mut Context) {
        if self.move_timer.finished() {
            self.snake_head.step();
            self.move_timer.reset();
        } else {
            self.snake_head.update_direction(&self.input);
        }
        if self.food_timer.finished() {
            self.food.spawn();
            self.food_timer.reset();
        } else {
        }
        self.input = Default::default()
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        repeat: bool,
    ) {
        if repeat {
            return;
        }

        match keycode {
            KeyCode::Left | KeyCode::A => {
                self.input.go_left = true;
            }
            KeyCode::Right | KeyCode::D => {
                self.input.go_right = true;
            }
            KeyCode::Up | KeyCode::W => {
                self.input.go_up = true;
            }
            KeyCode::Down | KeyCode::S => {
                self.input.go_down = true;
            }
            _ => {}
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {}

    fn draw(&mut self, ctx: &mut Context) {
        let mut uniform = self.camera.uniform();

        ctx.begin_default_pass(PassAction::Clear {
            color: Some(utils::Color::dark_gray().into()),
            depth: Some(1.),
            stencil: None,
        });
        ctx.apply_pipeline(&self.pipeline);

        self.snake_head.draw(ctx, &mut uniform);
        self.food.draw(ctx, &mut uniform);

        ctx.end_render_pass();
        ctx.commit_frame();
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        UserData::owning(Stage::new(&mut ctx), ctx)
    });
}
