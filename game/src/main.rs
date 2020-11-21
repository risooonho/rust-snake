use glam::{Mat4, Vec2, Vec3};
use miniquad::*;

#[repr(C)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Self {

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

        Stage { pipeline, bindings }
    }
}

impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        
        let (width, height) = ctx.screen_size();

        
        let fovy = 45.;
        let aspect = width / height;
        let top = fovy / 2.;
        let right = top * aspect;
        let projection = Mat4::orthographic_rh_gl(0., 800., 600., 0., -1., 1.0);
        let model = Mat4::identity();
        let t = date::now();

        ctx.begin_default_pass(Default::default());

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

        ctx.apply_uniforms(&shader::VertexUniforms {
            model,
            projection,
        });
        ctx.draw(0, 6, 1);
        
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
    use miniquad::*;
    use glam::Mat4;

    pub const VERTEX: &str = include_str!("./sprite.vert");
    pub const FRAGMENT: &str = include_str!("./sprite.frag");
    // uniform mat4 model;
    // uniform mat4 projection;

    #[repr(C)]
    pub struct VertexUniforms {
        pub model: Mat4,
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
