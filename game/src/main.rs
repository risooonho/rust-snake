use glam::{Mat4, Quat, Vec2, Vec3};
use miniquad::*;

mod components;

#[derive(PartialEq)]
enum Direction {
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

struct SnakeHead {
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

    pub fn update_direction(&mut self, input: &Input) {
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

    pub fn draw(&self, ctx: &mut Context, uniform: &mut shader::VertexUniforms) {
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

#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

struct Timer {
    start: f64,
    duration: f64,
}

impl Timer {
    fn new(duration: f64) -> Self {
        Self {
            start: date::now(),
            duration,
        }
    }

    fn reset(&mut self) {
        self.start = date::now();
    }

    fn finished(&self) -> bool {
        let now = date::now();
        return (now - self.start) > self.duration;
    }
}

#[derive(Default)]
struct Input {
    go_left: bool,
    go_right: bool,
    go_up: bool,
    go_down: bool,
}

struct Stage {
    input: Input,
    snake_head: SnakeHead,
    scale: f32,
    pipeline: Pipeline,
    move_timer: components::Timer,
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Self {
        let shader = shader::new(ctx).unwrap();

        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
        );

        let snake_head = SnakeHead::new(ctx);

        Stage {
            snake_head,
            pipeline,
            scale: 20.,
            move_timer: components::Timer::new(0.4),
            input: Input::default(),
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {
        if self.move_timer.finished() {
            self.snake_head.step();
            self.move_timer.reset();
        } else {
            self.snake_head.update_direction(&self.input);
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
        let (width, height) = ctx.screen_size();
        let aspect = width / height;
        let projection = Mat4::orthographic_rh_gl(
            -1. * aspect * self.scale,
            1. * aspect * self.scale,
            -1. * self.scale,
            1. * self.scale,
            -1.,
            1.0,
        );
        let view = Mat4::from_rotation_translation(Quat::identity(), Vec3::new(0.0, 0., 0.));
        let model = Mat4::identity();
        let mut uniform = shader::VertexUniforms {
            model,
            view,
            projection,
        };

        ctx.begin_default_pass(PassAction::clear_color(0.9, 0.9, 0.95, 1.));
        ctx.apply_pipeline(&self.pipeline);

        self.snake_head.draw(ctx, &mut uniform);

        ctx.end_render_pass();
        ctx.commit_frame();
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        UserData::owning(Stage::new(&mut ctx), ctx)
    });
}

mod shader {
    use glam::Mat4;
    use miniquad::*;

    pub const VERTEX: &str = include_str!("./sprite.vert");
    pub const FRAGMENT: &str = include_str!("./sprite.frag");

    #[repr(C)]
    pub struct VertexUniforms {
        pub model: Mat4,
        pub view: Mat4,
        pub projection: Mat4,
    }

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("model", UniformType::Mat4),
                    UniformDesc::new("view", UniformType::Mat4),
                    UniformDesc::new("projection", UniformType::Mat4),
                ],
            },
        }
    }

    pub fn new(ctx: &mut Context) -> Result<Shader, ShaderError> {
        Shader::new(ctx, VERTEX, FRAGMENT, meta())
    }
}
