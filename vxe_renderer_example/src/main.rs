use std::time::Instant;

use cgmath::{Deg, Euler, Vector3};
use obj::{load_obj, Obj};
use obj::Vertex as OBJVertex;

use vxe_renderer::context::{Context};
use vxe_renderer::context::utils::{FrameUtils, RenderUtils};
use vxe_renderer::data::{LumProgram, Sampler, Vertex};
use vxe_renderer::data::{VertexNormal, VertexPosition, VertexRGB};
use vxe_renderer::handler::Handler;
use vxe_renderer::RendererBuilder;
use vxe_renderer::types::{Camera, DeferredFrameBuffer, Material, Mesh, MeshShader, Transform};

use crate::material::TestMaterial;
use crate::pass::FinalPass;
use std::collections::HashMap;

mod material;
mod shader;
mod pass;

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
        // Loading mesh obj file
        let object_data: &[u8] = include_bytes!("cactus.obj");

        let obj: Obj<OBJVertex, u32> = load_obj(object_data).unwrap();

        // Converting mesh data to vxe-renderer's vertex format
        let vertices = obj.vertices.into_iter().map(|v|
        Vertex::new(
            VertexPosition::new(v.position),
            VertexNormal::new(v.normal),
            VertexRGB::new([1.0, 1.0, 1.0]),
        )).collect::<Vec<Vertex>>();

        // Creating mesh struct and building its internal data
        let mut mesh = Mesh::new(vertices, obj.indices);
        mesh.build(ctx);

        // Creating material and assigning it
        let mat = TestMaterial::new(ctx);
        mesh.set_material(mat);

        // Saving mesh struct, creating frame for deferred rendering and final pass shader
        self.mesh = Some(mesh);
        self.frame = Some(ctx.new_frame_buffer(ctx.resolution(), 1, Sampler::default()));
        self.final_pass = Some(FinalPass::new(ctx));

        // Offsetting camera location
        self.camera.transform.position -= Vector3::new(0.7, 0.0, 0.0);
    }

    fn draw(&mut self, ctx: &mut Context) {
        // Retrieving back buffer for further use
        let back = ctx.back_buffer();

        // Bringing out references to objects beforehand, to not get in trouble with borrow checker
        let mesh = self.mesh.as_ref().unwrap();
        let frame = self.frame.as_mut().unwrap();
        let pass = self.final_pass.as_mut().unwrap();

        // Printing fps every second
        if self.last.floor() < self.start_t.elapsed().as_secs_f32().floor() {
            println!("fps {}", ctx.fps());
        }

        self.last = self.start_t.elapsed().as_secs_f32();

        // Clearing frame
        FrameUtils::clear_black(ctx, frame);

        // Getting matrices from camera
        let (persp, view) = self.camera.matrices(ctx);

        // Mesh transform
        let mut mesh_trs = Transform::default();

        let rot = self.start_t.elapsed().as_secs_f32() * 40.0;
        mesh_trs.rotation = Euler::new(Deg(0.0 + rot), Deg(-90.0 + rot), Deg(0.0 + rot)).into();

        // Drawing the mesh onto the frame
        mesh.draw(ctx, frame, mesh_trs, persp, view);

        // Rendering whatever is in the frame to back buffer
        RenderUtils::render_quad_pass_rgb_depth(
            ctx,
            &back,
            pass,
            HashMap::new(),
            ("frame".to_string(), frame.depth_slot())
        )
    }
}

fn main() {
    // Initializing renderer
    let mut renderer = RendererBuilder::new()
        .title("3D example")
        .resolution([1366, 768])
        .vsync(false)
        .fps_limit(69.0)
        .build();

    // Initializing event handler
    let handler = ExampleHandler {
        start_t: Instant::now(),
        last: 0.0,
        mesh: None,
        frame: None,
        final_pass: None,
        camera: Camera::new(Transform::default(), 90.0, 0.01, 100.0),
    };

    // Running render loop
    renderer.run_loop(handler);
}
