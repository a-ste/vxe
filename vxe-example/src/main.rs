use vxe_renderer::{
    data::*,
    RendererBuilder
};
use vxe_renderer::handler::Handler;
use vxe_renderer::context::{Context, LumProgram, PipelineState, RenderState};
use std::time::Instant;
use std::f32::consts::PI;

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
    tess: Vec<Vertex>,
    start: Instant,
    last_sec: u64,
}

impl Handler for ExampleHandler {
    fn init(&mut self, ctx: &mut Context) {
        self.shd = Some(ctx.new_shader_program(VS, FS));
    }

    fn draw(&mut self, ctx: &mut Context) {
        let back_buffer = &ctx.back_buffer();
        let mut shader = self.shd.as_mut().unwrap();

        let time = self.start.elapsed().as_secs_f32();

        let vert = &mut self.tess;

        // vert[0] = Vertex::new(
        //     VertexPosition::new([-0.5 + (time + (2.0 * PI) / 3.0).sin() / 3.0, -0.5 + (time + (2.0 * PI) / 3.0 + PI / 3.0).sin() / 3.0]),
        //     VertexRGB::new([255, 0, 0]),
        // );
        //
        // vert[1] = Vertex::new(
        //     VertexPosition::new([0.5 + (time + (2.0 * PI) / 3.0 * 2.0).sin() / 3.0, -0.5 + (time + (2.0 * PI) / 3.0 * 2.0 + PI / 3.0).sin() / 3.0]),
        //     VertexRGB::new([0, 255, 0]),
        // );
        //
        // vert[2] = Vertex::new(
        //     VertexPosition::new([0. + (time).sin() / 3.0, 0.5 + (time + PI / 3.0).sin() / 3.0]),
        //     VertexRGB::new([0, 0, 255]),
        // );

        let tess = ctx.new_tess(vert);

        let r = (time).sin() / 2.0 + 0.5;
        let g = (time + (2.0 * PI) / 3.0).sin() / 2.0 + 0.5;
        let b = (time + (2.0 * PI) / 3.0 * 2.0).sin() / 2.0 + 0.5;


        ctx.pipeline(back_buffer, PipelineState::default().set_clear_color([r, g, b, 1.0]), |mut pc| {
            pc.use_shader(&mut shader, |mut rc| {
                rc.render(RenderState::default(),|mut tc| {
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
        .vsync(false)
        .build();

    let handler = ExampleHandler {
        shd: None,
        tess: VERTICES.to_vec(),
        start: Instant::now(),
        last_sec: 0,
    };

    renderer.run_loop(handler);
}
