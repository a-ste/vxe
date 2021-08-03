mod shader;
mod final_pass;

use crate::context::{Context, RenderState};
use crate::data::{LumProgram, LumFrameBuffer, LumRGB, LumDepth, Sampler, DepthComparison};
use crate::deferred::light_pass::shader::LightPassShader;
use crate::deferred::light_pass::final_pass::FinalLightPassShader;
use crate::types::{Shader, DeferredFrameBuffer, Light, default_pipeline, UniformParameter};
use luminance::backend::color_slot::ColorSlot;
use luminance_gl::GL33;
use luminance::texture::Dim2;
use luminance::backend::depth_slot::DepthSlot;
use crate::context::utils::{RenderUtils, FrameUtils};
use std::collections::HashMap;
use luminance::blending::{Blending, Equation, Factor};
use std::convert::TryFrom;
use cgmath::Vector3;

#[allow(dead_code)]
pub struct LightPass {
    frame: LumFrameBuffer<LumRGB, LumDepth>,
    shader: LumProgram<LightPassShader>,
    final_pass: LumProgram<FinalLightPassShader>
}

#[allow(dead_code)]
impl LightPass {
    pub fn new(ctx: &mut Context) -> LightPass {
        LightPass {
            frame: ctx.new_frame_buffer(ctx.resolution(), 0, Sampler::default()),
            shader: LightPassShader::new(ctx),
            final_pass: FinalLightPassShader::new(ctx)
        }
    }

    pub fn render<C, D>(&mut self, ctx: &mut Context, frame: &mut DeferredFrameBuffer, target: &LumFrameBuffer<C, D>, camera_pos: Vector3<f32>, lights: &Vec<Light>)
        where
            C: ColorSlot<GL33, Dim2>,
            D: DepthSlot<GL33, Dim2>
    {
        let shd = &mut self.shader;
        let final_pass = &mut self.final_pass;
        let frm = &mut self.frame;
        let quad = ctx.new_quad();
        let (_, normal_slot, position_slot, rms_slot) = frame.color_slot();

        FrameUtils::clear_black(ctx, frm);

        // Rendering each light into the frame buffer
        for light in lights {
            ctx.pipeline(frm, default_pipeline(), |pc, mut sc| {
                let normal = pc.bind_texture(normal_slot);
                let position = pc.bind_texture(position_slot);
                let rms = pc.bind_texture(rms_slot);

                sc.use_shader(shd,|mut rc, uni| {
                    let params = uni.parameters();

                    UniformParameter::vector3_uniform(&mut rc, &params, "camera_pos", <[f32; 3]>::try_from(camera_pos).unwrap());

                    UniformParameter::texture_uniform(&mut rc, &params, "normal_texture", normal.binding());
                    UniformParameter::texture_uniform(&mut rc, &params, "position_texture", position.binding());
                    UniformParameter::texture_uniform(&mut rc, &params, "rms_texture", rms.binding());

                    UniformParameter::vector3_uniform(&mut rc, &params, "light_pos", <[f32; 3]>::try_from(light.position).unwrap());
                    UniformParameter::vector3_uniform(&mut rc, &params, "light_color", light.color);
                    UniformParameter::float_uniform(&mut rc, &params, "light_intensity", light.intensity);

                    UniformParameter::float_uniform(&mut rc, &params, "light_linear", light.linear_attenuation);
                    UniformParameter::float_uniform(&mut rc, &params, "light_quadratic", light.quadratic_attenuation);

                    rc.render(
                        RenderState::default().set_blending(
                                Blending {
                                equation: Equation::Additive,
                                src: Factor::One,
                                dst: Factor::One
                            })
                            .set_depth_test(DepthComparison::Always), |mut tc| {
                        tc.draw(&quad)
                    })
                })
            });
        }

        // Rendering onto target frame
        let (diffuse_slot, ..) = frame.color_slot();
        let light_slot = frm.color_slot();

        let mut map = HashMap::new();

        map.insert("diffuse_texture".to_string(), diffuse_slot);
        map.insert("light_texture".to_string(), light_slot);

        RenderUtils::render_quad_pass_rgb(
            ctx,
            target,
            final_pass,
            map
        )
    }
}