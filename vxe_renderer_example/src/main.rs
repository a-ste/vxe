mod material;
mod shader;
mod pass;

use vxe_renderer::handler::Handler;
use vxe_renderer::context::{Context, PipelineState, RenderState};
use vxe_renderer::data::{Vertex, Sampler, LumTess, LumProgram};
use vxe_renderer::RendererBuilder;
use vxe_renderer::types::{Mesh, DeferredFrameBuffer, Material, Parameter, Transform, Camera, MeshShader, UniformParameter};

use obj::{load_obj, Obj};
use obj::Vertex as OBJVertex;
use vxe_renderer::data::{VertexPosition, VertexNormal, VertexRGB};
use std::rc::Rc;
use std::sync::RwLock;
use std::collections::HashMap;
use crate::material::TestMaterial;
use cgmath::{Vector3, Quaternion, Rotation, Euler, Deg};
use std::borrow::Borrow;
use std::f32::consts::PI;
use crate::pass::FinalPass;
use std::time::Instant;

pub struct ExampleHandler {
    start_t: Instant,
    last: f32,

    mesh: Option<Mesh>,
    frame: Option<DeferredFrameBuffer>,

    final_pass: Option<LumProgram<FinalPass>>,
    camera: Camera,
}

impl Handler for ExampleHandler {
    fn init(&mut self, ctx: &mut Context) {
        let object_data: &[u8] = include_bytes!("cactus.obj");

        let obj: Obj<OBJVertex, u32> = load_obj(object_data).unwrap();

        let vertices = obj.vertices.into_iter().map(|v|
        Vertex::new(
            VertexPosition::new(v.position),
            VertexNormal::new(v.normal),
            VertexRGB::new([1.0, 1.0, 1.0]),
        )).collect::<Vec<Vertex>>();

        let mut mesh = Mesh::new(vertices, obj.indices);
        mesh.build(ctx);

        let mat = TestMaterial::new(ctx);
        mesh.set_material(mat);

        self.mesh = Some(mesh);
        self.frame = Some(ctx.new_frame_buffer(ctx.resolution(), 1, Sampler::default()));
        self.final_pass = Some(FinalPass::new(ctx));


        self.camera.transform.position -= Vector3::new(0.7, 0.0, 0.0);
    }

    fn draw(&mut self, ctx: &mut Context) {
        let back = ctx.back_buffer();

        let mesh = self.mesh.as_ref().unwrap();
        let frame = self.frame.as_mut().unwrap();
        let pass = self.final_pass.as_mut().unwrap();

        if self.last.floor() < self.start_t.elapsed().as_secs_f32().floor() {
            println!("fps {}", ctx.fps());
        }

        self.last = self.start_t.elapsed().as_secs_f32();


        // Clearing
        ctx.pipeline(frame, PipelineState::default(), |pc, sc| Ok(()));


        let (persp, view) = self.camera.matrices(ctx);

        let mut mesh_trs = Transform::default();

        let rot = self.start_t.elapsed().as_secs_f32() * 40.0;
        mesh_trs.rotation = Euler::new(Deg(45.0 + rot), Deg(45.0 + rot), Deg(45.0)).into();

        mesh.draw(ctx, frame, mesh_trs, persp, view);


        // Rendering whatever is in the frame
        let quad = ctx.new_quad();

        ctx.pipeline(&back, PipelineState::default(), |pc, mut sc| {
            let frame_tex = frame.color_slot();
            let bound_tex = pc.bind_texture(frame_tex);

            sc.use_shader(pass, |mut rc, uni| {
                let params = uni.parameters();

                if let Some(frame_enum) = params.get("frame") {
                    if let UniformParameter::Texture(frame_uniform) = frame_enum {
                        rc.set_uniform(frame_uniform, bound_tex.binding());
                    }
                }

                rc.render(RenderState::default(), |mut tc| {
                    tc.draw(&quad)
                })
            })
        });
    }
}

fn main() {
    let mut renderer = RendererBuilder::new()
        .title("3D example")
        .resolution([1366, 768])
        .vsync(false)
        .fps_limit(0.0)
        .build();

    let handler = ExampleHandler {
        start_t: Instant::now(),
        last: 0.0,
        mesh: None,
        frame: None,
        final_pass: None,
        camera: Camera::new(Transform::default(), 90.0, 0.01, 100.0),
    };

    renderer.run_loop(handler);
}
