use vxe_renderer::{
    data::{*, shader::*},
    RendererBuilder,
    vertex,
    shd_interface
};
use vxe_renderer::handler::Handler;
use vxe_renderer::context::{Context, LumProgram, PipelineState, RenderState};
use std::time::Instant;
use std::f32::consts::PI;

const VERTICES: [Vertex; 4] = [
    vertex![-1.0, -1.0, 0.0, 1.0, 0.0, 0.0],
    vertex![1.0, -1.0, 0.0, 0.0, 1.0, 0.0],
    vertex![1.0, 1.0, 0.0, 0.0, 0.0, 1.0],
    vertex![-1.0, 1.0, 0.0, 1.0, 1.0, 1.0],
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

uniform float intensity;

void main() {
  frag_color = vec4(v_color, 1.0) * intensity;
}
"#;

shd_interface!(
    BasicShader,
    intensity, f32
);


pub struct ExampleHandler {
    shd: Option<LumProgram<BasicShader>>,
    tess: Vec<Vertex>,
    start: Instant,
    last_sec: u64,
    lean: f32,
    span: f32,
}

impl Handler for ExampleHandler {
    fn init(&mut self, ctx: &mut Context) {
        self.shd = Some(ctx.new_shader_program(VS, FS));
    }

    fn draw(&mut self, ctx: &mut Context) {
        let back_buffer = &ctx.back_buffer();
        let mut shader = self.shd.as_mut().unwrap();

        let vert = &mut self.tess;

        self.span += ctx.delta() * 15.0;
        let time = self.span;

        let phase1 = time;
        let phase2 = time + (2.0 * PI) / 4.0;
        let phase3 = time + (2.0 * PI) / 4.0 * 2.0;
        let phase4 = time + (2.0 * PI) / 4.0 * 3.0;

        vert[0] = vertex![-1.0, -1.0, 0.0, phase1.sin() / 2.0 + 0.5, phase2.sin() / 2.0 + 0.5, phase3.sin() / 2.0 + 0.5];
        vert[1] = vertex![1.0, -1.0, 0.0,  phase2.sin() / 2.0 + 0.5, phase3.sin() / 2.0 + 0.5, phase4.sin() / 2.0 + 0.5];
        vert[2] = vertex![1.0, 1.0, 0.0,   phase3.sin() / 2.0 + 0.5, phase4.sin() / 2.0 + 0.5, phase1.sin() / 2.0 + 0.5];
        vert[3] = vertex![-1.0, 1.0, 0.0,  phase4.sin() / 2.0 + 0.5, phase1.sin() / 2.0 + 0.5, phase2.sin() / 2.0 + 0.5];

        let tess = ctx.new_tess(vert, &[0, 2, 1, 0, 3, 2].to_vec());

        self.lean += ctx.delta() / 10.0;

        ctx.pipeline(back_buffer, PipelineState::default().set_clear_color([0.0, 0.0, 0.0, 1.0]), |mut pc| {
            pc.use_shader(&mut shader, |mut rc, uni| {
                rc.set_uniform(&uni.intensity, phase1.sin() / 2.0 + 0.5);

                rc.render(RenderState::default()
                              .set_face_culling(FaceCulling::new(FaceCullingOrder::CW, FaceCullingMode::Back)),
                          |mut tc| {
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
        .fps_limit(200.0)
        .build();

    let handler = ExampleHandler {
        shd: None,
        tess: VERTICES.to_vec(),
        start: Instant::now(),
        last_sec: 0,
        lean: 0.0,
        span: 0.0,
    };

    renderer.run_loop(handler);
}
