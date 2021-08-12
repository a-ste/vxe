use std::time::Instant;

use cgmath::{Deg, Euler, Vector3, Rad};
use obj::{load_obj, Obj};
use obj::Vertex as OBJVertex;

use vxe_renderer::context::{Context};
use vxe_renderer::context::utils::{FrameUtils};
use vxe_renderer::data::{Sampler, Vertex};
use vxe_renderer::data::{VertexNormal, VertexPosition, VertexRGB};
use vxe_renderer::handler::Handler;
use vxe_renderer::RendererBuilder;
use vxe_renderer::types::{Camera, DeferredFrameBuffer, Material, Mesh, Transform, Light};
use vxe_renderer::vertex;

use vxe_renderer::deferred::{PBRMaterial, LightPass};
use cgmath::num_traits::FloatConst;
use rand::Rng;

mod material;
mod shader;
mod pass;

pub struct ExampleHandler {
    start_t: Instant,
    last: f32,

    mesh: Option<Mesh>,
    frame: Option<DeferredFrameBuffer>,
    light_pass: Option<LightPass>,
    camera: Camera,
    lights: Vec<Light>
}

impl Handler for ExampleHandler {
    fn init(&mut self, ctx: &mut Context) {
        // Loading mesh obj file
        let object_data: &[u8] = include_bytes!("ico.obj");

        let obj: Obj<OBJVertex, u32> = load_obj(object_data).unwrap();

        // Converting mesh data to vxe-renderer's vertex format
        let vertices = obj.vertices.into_iter().map(|v|
            vertex!(v.position[0], v.position[1], v.position[2],
                    v.normal[0], v.normal[1], v.normal[2],
                    1.0, 1.0, 1.0)
        ).collect::<Vec<Vertex>>();

        // Creating mesh struct and building its internal data
        let mut mesh = Mesh::new(vertices, obj.indices);
        mesh.build(ctx);

        // Creating material and assigning it
        let mat = PBRMaterial::new(ctx);
        mesh.set_material(mat);

        // Saving mesh struct, creating frame for deferred rendering and final pass shader
        self.mesh = Some(mesh);
        self.frame = Some(ctx.new_frame_buffer(ctx.resolution(), 1, Sampler::default()));
        self.light_pass = Some(LightPass::new(ctx));

        // Offsetting camera location
        self.camera.transform.position += Vector3::new(5.7, 0.0, 0.0);
        self.camera.transform.rotation = Euler::new(Deg(0.0), Deg(0.0), Deg(180.0)).into();


    }

    fn draw(&mut self, ctx: &mut Context) {
        // Retrieving back buffer for further use
        let back = ctx.back_buffer();

        // Bringing out references to objects beforehand, to not get in trouble with borrow checker
        let mesh = self.mesh.as_ref().unwrap();
        let frame = self.frame.as_mut().unwrap();
        let pass = self.light_pass.as_mut().unwrap();

        // Printing fps every second
        if self.last.floor() < self.start_t.elapsed().as_secs_f32().floor() {
            println!("fps {}", ctx.fps());
        }
        self.last = self.start_t.elapsed().as_secs_f32();

        // Rotating camera
        let time = f32::sin(self.start_t.elapsed().as_secs_f32() * 0.1);
        let rot = time.sin() * f32::PI();
        self.camera.transform.position = Vector3::new(rot.sin() * 7.7,rot.cos() * 7.7, 0.0);
        self.camera.transform.rotation = Euler::new(Rad(0.0), Rad(0.0), Rad(-(time * 2.5) - f32::PI() / 2.0)).into();

        // Clearing frame
        FrameUtils::clear_black(ctx, frame);

        // Getting matrices from camera
        let (persp, view) = self.camera.matrices(ctx);

        for x in -8..=8 {
            for y in -5..=5 {
                // Mesh transform
                let mut mesh_trs = Transform::default();
                mesh_trs.position += Vector3::new(0.0, x as f32, y as f32);
                mesh_trs.scale = Vector3::new(0.4, 0.4, 0.4);
                let rot = self.start_t.elapsed().as_secs_f32() * 40.0;
                mesh_trs.rotation = Euler::new(Deg(0.0 + rot), Deg(-90.0 + rot), Deg(0.0 + rot)).into();

                // Drawing the mesh onto the frame
                mesh.draw(ctx, frame, mesh_trs, persp, view);
            }
        }

        // Rendering light pass onto back buffer
        pass.render::<(), ()>(ctx, frame, &back, self.camera.transform.position, &self.lights)
    }
}

fn main() {
    // Initializing renderer
    let mut renderer = RendererBuilder::new()
        .title("3D example")
        .resolution([1366, 768])
        .vsync(false)
        .fps_limit(0.0)
        .build();

    let mut lights = vec![];
    let mut rang = rand::thread_rng();

    for i in 0..50 {
        let mut light = Light::new([1.5, rang.gen_range(-7.0..7.0), rang.gen_range(-4.0..4.0)]);

        light.color = [rang.gen_range(0.1..1.0), rang.gen_range(0.1..1.0), rang.gen_range(0.1..1.0)];
        light.linear_attenuation = 0.7;
        light.quadratic_attenuation = 1.6;

        println!("light #{} radius: {}", i, light.radius());

        lights.push(light)
    }

    // Initializing event handler
    let handler = ExampleHandler {
        start_t: Instant::now(),
        last: 0.0,
        mesh: None,
        frame: None,
        light_pass: None,
        camera: Camera::new(Transform::default(), 90.0, 0.01, 100.0),
        lights
    };

    // Running render loop
    renderer.run_loop(handler);
}

