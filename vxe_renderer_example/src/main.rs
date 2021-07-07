use vxe_renderer::{
    data::{*, shader::*},
    RendererBuilder,
    vertex,
    shd_interface
};
use vxe_renderer::handler::Handler;
use vxe_renderer::data::LumProgram;
use vxe_renderer::context::{Context, PipelineState, RenderState};
use std::time::Instant;
use std::f32::consts::PI;

const VERTICES: [Vertex; 4] = [
    vertex![-1.0, -1.0, 0.0, 1.0, 0.0, 0.0],
    vertex![1.0, -1.0, 0.0, 0.0, 1.0, 0.0],
    vertex![1.0, 1.0, 0.0, 0.0, 0.0, 1.0],
    vertex![-1.0, 1.0, 0.0, 1.0, 1.0, 1.0],
];

const VS: &'static str = r#"
in vec3 position;
in vec3 color;

out vec3 v_color;
out vec3 v_position;

void main() {
  v_color = color;
  v_position = position;

  gl_Position = vec4(position, 1.);
}
"#;

const FS: &'static str = r#"
in vec3 v_color;
in vec3 v_position;

layout (location = 0) out vec4 frag_color;
layout (location = 1) out vec4 frag_color2;

uniform float time;

void main() {
    frag_color = vec4(v_color.brg, 1.0);
    frag_color2 = vec4(v_color, 1.0) * clamp(tan(v_position.y * 50 - (time * 3)) / 2 + 0.5, 0.0, 1.0);
}
"#;

const MULTI_FS: &'static str = r#"
in vec3 v_color;
in vec3 v_position;

out vec4 frag_color;

uniform sampler2D first;
uniform sampler2D second;

void main() {
    vec2 uv = (v_position.xy + 1.0) / 2.0;

    vec4 col1 = vec4(texture(first, uv).rgb, 1.);
    vec4 col2 = vec4(texture(second, uv).rgb, 1.);

    frag_color = col1 / 2.0 + col2 / 2.0;
}
"#;

shd_interface!(
    BasicShader,
    intensity, f32,
    time, f32
);

shd_interface!(
    FinalShader,
    first, LumTextureBinding,
    second, LumTextureBinding
);


pub struct ExampleHandler {
    color_shd: Option<LumProgram<BasicShader>>,
    final_shd: Option<LumProgram<FinalShader>>,
    frm: Option<LumFrameBuffer<(LumRGB, LumRGB), ()>>,
    tess: Vec<Vertex>,
    start: Instant,
    last_sec: u64,
    lean: f32,
    span: f32,
}

impl Handler for ExampleHandler {
    fn init(&mut self, ctx: &mut Context) {
        println!("res {:?}", ctx.resolution());
        self.color_shd = Some(ctx.new_shader_program(VS, FS));
        self.final_shd = Some(ctx.new_shader_program(VS, MULTI_FS));
        self.frm = Some(ctx.new_frame_buffer(ctx.resolution(), 0, Sampler::default()));
    }

    fn draw(&mut self, ctx: &mut Context) {
        let back_buffer = &ctx.back_buffer();
        let mut shader = self.color_shd.as_mut().unwrap();
        let mut final_shd = self.final_shd.as_mut().unwrap();
        let frm = self.frm.as_mut().unwrap();

        let vert = &mut self.tess;

        self.span += ctx.delta() * 5.0;
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

        ctx.pipeline(frm, PipelineState::default().set_clear_color([0.0, 0.0, 0.0, 1.0]), |_, mut sc| {
            sc.use_shader(&mut shader, |mut rc, uni| {
                rc.set_uniform(&uni.intensity, phase1.sin() / 2.0 + 1.0);
                rc.set_uniform(&uni.time, time);

                rc.render(RenderState::default()
                              .set_face_culling(FaceCulling::new(FaceCullingOrder::CW, FaceCullingMode::Back)),
                          |mut tc| {
                    tc.draw(&tess)
                })
            })
        });

        let quad = ctx.new_quad();

        ctx.pipeline(back_buffer, PipelineState::default(), |pc, mut sc| {
            let (first, second) = frm.color_slot();

            let fbind = pc.bind_texture(first);
            let sbind = pc.bind_texture(second);

            sc.use_shader(&mut final_shd, |mut rc, uni| {
                rc.set_uniform(&uni.first, fbind.binding());
                rc.set_uniform(&uni.second, sbind.binding());

                rc.render(RenderState::default(), |mut tc| {
                    tc.draw(&quad)
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
        .title("ksu yu hui")
        .vsync(false)
        .fps_limit(69.0)
        .build();

    let handler = ExampleHandler {
        color_shd: None,
        final_shd: None,
        frm: None,
        tess: VERTICES.to_vec(),
        start: Instant::now(),
        last_sec: 0,
        lean: 0.0,
        span: 0.0,
    };

    renderer.run_loop(handler);
}
