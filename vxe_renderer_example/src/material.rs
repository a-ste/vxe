use vxe_renderer::types::{Material, DeferredFrameBuffer, Parameter, MeshShader, default_pipeline, UniformParameter, default_render_state};
use vxe_renderer::context::{Context};
use std::rc::Rc;
use std::sync::RwLock;
use vxe_renderer::data::{LumTess, Vertex, LumProgram};
use std::collections::HashMap;
use crate::shader::TestShader;

pub struct TestMaterial {
    shr: LumProgram<TestShader>
}

impl Material for TestMaterial {
    fn new(ctx: &mut Context) -> Rc<RwLock<dyn Material>> where Self: Sized {
        let mat = TestMaterial {
            shr: TestShader::new(ctx)
        };

        Rc::new(RwLock::new(mat))
    }

    fn render(&mut self, ctx: &mut Context, frame: &DeferredFrameBuffer, tess: &LumTess<Vertex, u32>, trs: [[f32; 4]; 4], projection: [[f32; 4]; 4], view: [[f32; 4]; 4]) {
        ctx.pipeline(frame, default_pipeline(), |_, mut sc| {
            sc.use_shader(&mut self.shr, |mut rc, uni| {
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

                rc.render(default_render_state(), |mut tc| {
                    tc.draw(tess)
                })
            })
        });
    }

    fn name(&self) -> String {
        "Test Material".to_string()
    }

    fn set_parameter(&mut self, _name: String, _parameter: Parameter) {
        todo!()
    }

    fn parameters(&self) -> HashMap<String, Parameter> {
        todo!()
    }
}
