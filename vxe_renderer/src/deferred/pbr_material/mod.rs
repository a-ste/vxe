mod shader;

use crate::types::{Material, DeferredFrameBuffer, Parameter, default_pipeline, UniformParameter, default_render_state, Shader};
use crate::context::Context;
use std::rc::Rc;
use std::sync::RwLock;
use crate::data::{LumTess, Vertex, LumProgram};
use std::collections::HashMap;
use crate::deferred::pbr_material::shader::PBRShader;

pub struct PBRMaterial {
    diffuse_color: [f32; 3],
    roughness: f32,
    metallic: f32,
    specular: f32,

    shader: LumProgram<PBRShader>,
}

impl Material for PBRMaterial {
    fn new(ctx: &mut Context) -> Rc<RwLock<dyn Material>> where Self: Sized {
        let mat = PBRMaterial {
            diffuse_color: [1.0, 1.0, 1.0],
            roughness: 0.01,
            metallic: 0.0,
            specular: 0.2,
            shader: PBRShader::new(ctx),
        };

        Rc::new(RwLock::new(mat))
    }

    fn render(&mut self, ctx: &mut Context, frame: &DeferredFrameBuffer, tess: &LumTess<Vertex, u32>, trs: [[f32; 4]; 4], projection: [[f32; 4]; 4], view: [[f32; 4]; 4]) {
        let diffuse = self.diffuse_color;
        let rms = [self.roughness, self.metallic, self.specular];

        ctx.pipeline(frame, default_pipeline(), |_, mut sc| {
            sc.use_shader(&mut self.shader, |mut rc, uni| {
                let params = uni.parameters();

                if let Some(proj_enum) = params.get("projection") {
                    if let UniformParameter::Matrix4(projection_uniform) = proj_enum {
                        rc.set_uniform(projection_uniform, projection);
                    }
                }

                if let Some(view_enum) = params.get("view") {
                    if let UniformParameter::Matrix4(view_uniform) = view_enum {
                        rc.set_uniform(view_uniform, view);
                    }
                }

                if let Some(trs_enum) = params.get("trs") {
                    if let UniformParameter::Matrix4(trs_uniform) = trs_enum {
                        rc.set_uniform(trs_uniform, trs);
                    }
                }

                if let Some(color_enum) = params.get("color") {
                    if let UniformParameter::Vector3(color_uniform) = color_enum {
                        rc.set_uniform(color_uniform, diffuse);
                    }
                }

                if let Some(rms_enum) = params.get("rms") {
                    if let UniformParameter::Vector3(rms_uniform) = rms_enum {
                        rc.set_uniform(rms_uniform, rms);
                    }
                }

                rc.render(default_render_state(), |mut tc| {
                    tc.draw(tess)
                })
            })
        });
    }

    fn name(&self) -> String {
        "PBR Material".to_string()
    }

    fn set_parameter(&mut self, _name: String, _parameter: Parameter) {
        todo!()
    }

    fn parameters(&self) -> HashMap<String, Parameter> {
        todo!()
    }
}