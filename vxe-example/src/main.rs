use vxe_renderer::{
    data::*,
    RendererBuilder
};
use vxe_renderer::handler::Handler;
use vxe_renderer::context::{Context, LumProgram, LumTess};
use std::time::Instant;

const VERTICES: [Vertex; 3] = [
    Vertex::new(
        VertexPosition::new([-0.5, -0.5]),
        VertexRGB::new([255, 0, 0]),
    ),
    Vertex::new(
        VertexPosition::new([0.5, -0.5]),
        VertexRGB::new([0, 255, 0]),
    ),
    Vertex::new(
        VertexPosition::new([0., 0.5]),
        VertexRGB::new([0, 0, 255])
    ),
];

const VS: &'static str = r#"
in vec2 position;
in vec3 color;

out vec3 v_color;

void main() {
  v_color = color;

  gl_Position = vec4(position, 0., 1.);
}
"#;

const FS: &'static str = r#"
in vec3 v_color;

out vec4 frag_color;

void main() {
  frag_color = vec4(v_color, 1.0);
}
"#;

pub struct ExampleHandler {
    shd: Option<LumProgram>,
    tess: Option<LumTess>,
    start: Instant,
    last_sec: u64,
}

impl Handler for ExampleHandler {
    fn init(&mut self, ctx: &mut Context) {
        self.shd = Some(ctx.new_shader_program(VS, FS));
        self.tess = Some(ctx.new_tess(VERTICES.to_vec()));
    }

    fn draw(&mut self, ctx: &mut Context) {
        let back_buffer = &ctx.back_buffer();
        let mut shader = self.shd.as_mut().unwrap();
        let tess = self.tess.as_ref().unwrap();

        ctx.pipeline(back_buffer, |mut pc| {
            pc.use_shader(&mut shader, |mut rc| {
                rc.render(|mut tc| {
                    tc.draw(&tess)
                })
            })
        });

        if self.last_sec != self.start.elapsed().as_secs() {
            println!("{} fps", ctx.get_fps());
        }

        self.last_sec = self.start.elapsed().as_secs();
    }
}

fn main() {
    let mut renderer = RendererBuilder::new()
        .title("hi")
        .vsync(true)
        .build();

    let handler = ExampleHandler {
        shd: None,
        tess: None,
        start: Instant::now(),
        last_sec: 0,
    };

    renderer.run_loop(handler);
}
