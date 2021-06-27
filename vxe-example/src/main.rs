use vxe_renderer::{
    data::*,
    RendererBuilder,
    vertex
};
use vxe_renderer::handler::Handler;
use vxe_renderer::context::{Context, LumProgram, PipelineState, RenderState};
use std::time::Instant;

const VERTICES: [Vertex; 3] = [
    vertex![-0.5, -0.5, 0.0, 255, 0, 0],
    vertex![0.5, -0.5, 0.0, 0, 255, 0],
    vertex![0., 0.5, 0.0, 0, 0, 255],
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
    tess: Vec<Vertex>,
    start: Instant,
    last_sec: u64,
    lean: f32,
}

impl Handler for ExampleHandler {
    fn init(&mut self, ctx: &mut Context) {
        self.shd = Some(ctx.new_shader_program(VS, FS));
    }

    fn draw(&mut self, ctx: &mut Context) {
        let back_buffer = &ctx.back_buffer();
        let mut shader = self.shd.as_mut().unwrap();

        // let time = self.start.elapsed().as_secs_f32();

        let vert = &mut self.tess;

        vert[0] = vertex![-0.5 - self.lean, -0.5, 0.0, 255, 0, 0];

        let tess = ctx.new_tess(vert, &[0, 1, 2].to_vec());

        // let r = (time).sin() / 2.0 + 0.5;
        // let g = (time + (2.0 * PI) / 3.0).sin() / 2.0 + 0.5;
        // let b = (time + (2.0 * PI) / 3.0 * 2.0).sin() / 2.0 + 0.5;

        self.lean += ctx.delta() / 10.0;

        ctx.pipeline(back_buffer, PipelineState::default().set_clear_color([0.0, 0.0, 0.0, 1.0]), |mut pc| {
            pc.use_shader(&mut shader, |mut rc| {
                rc.render(RenderState::default(),|mut tc| {
                    tc.draw(&tess)
                })
            })
        });

        if self.last_sec != self.start.elapsed().as_secs() {
            println!("{} fps", ctx.fps());
        }

        self.last_sec = self.start.elapsed().as_secs();
    }
}

fn main() {
    let mut renderer = RendererBuilder::new()
        .title("hi")
        .vsync(false)
        .fps_limit(200)
        .build();

    let handler = ExampleHandler {
        shd: None,
        tess: VERTICES.to_vec(),
        start: Instant::now(),
        last_sec: 0,
        lean: 0.0,
    };

    renderer.run_loop(handler);
}
