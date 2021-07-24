use crate::context::Context;
use luminance::pipeline::PipelineState;
use luminance::render_state::RenderState;
use crate::data::{LumFrameBuffer, LumProgram, GL33, LumRGB, LumRGBA, LumGray, LumDepth};
use crate::types::{UniformParameter, MeshShader};
use luminance::shader::UniformInterface;
use std::collections::HashMap;
use luminance::texture::{Dim2, Texture};
use luminance::backend::color_slot::ColorSlot;
use luminance::backend::depth_slot::DepthSlot;

/// Util struct to help with rendering things
#[allow(dead_code)]
pub struct RenderUtils {}

#[allow(dead_code)]
impl RenderUtils {
    /// Renders a quad using provided shader and binds provided RGB color slots
    pub fn render_quad_pass_rgb<C1, D1, S>(
        ctx: &mut Context, target_frame: &LumFrameBuffer<C1, D1>,
        shader: &mut LumProgram<S>, color_slots: HashMap<String, &mut Texture<GL33, Dim2, LumRGB>>
    ) where
        C1: ColorSlot<GL33, Dim2>,
        D1: DepthSlot<GL33, Dim2>,
        S: UniformInterface<GL33> + MeshShader<S>
    {
        // Creating quad to draw with
        let quad = ctx.new_quad();

        // Creating pipeline
        ctx.pipeline(target_frame, PipelineState::default(), |pc, mut sc| {
            // Binding frame's color slots
            let mut bound_textures = HashMap::new();

            for (name, texture) in color_slots {
                bound_textures.insert(name, pc.bind_texture(texture));
            }

            // Using the final pass shader
            sc.use_shader(shader, |mut rc, uni| {
                let params = uni.parameters();

                // Passing frame texture to the shader
                for (name, bound) in bound_textures {
                    if let Some(uniform_param) = params.get(&name) {
                        if let UniformParameter::Texture(uniform_ref) = uniform_param {
                            rc.set_uniform(uniform_ref, bound.binding());
                        }
                    }
                }

                // Rendering quad
                rc.render(RenderState::default(), |mut tc| {
                    tc.draw(&quad)
                })
            })
        });
    }

    /// Renders a quad using provided shader and binds provided RGBA color slots
    pub fn render_quad_pass_rgba<C1, D1, S>(
        ctx: &mut Context, target_frame: &LumFrameBuffer<C1, D1>,
        shader: &mut LumProgram<S>, color_slots: HashMap<String, &mut Texture<GL33, Dim2, LumRGBA>>
    ) where
        C1: ColorSlot<GL33, Dim2>,
        D1: DepthSlot<GL33, Dim2>,
        S: UniformInterface<GL33> + MeshShader<S>
    {
        // Creating quad to draw with
        let quad = ctx.new_quad();

        // Creating pipeline
        ctx.pipeline(target_frame, PipelineState::default(), |pc, mut sc| {
            // Binding frame's color slots
            let mut bound_textures = HashMap::new();

            for (name, texture) in color_slots {
                bound_textures.insert(name, pc.bind_texture(texture));
            }

            // Using the final pass shader
            sc.use_shader(shader, |mut rc, uni| {
                let params = uni.parameters();

                // Passing frame texture to the shader
                for (name, bound) in bound_textures {
                    if let Some(uniform_param) = params.get(&name) {
                        if let UniformParameter::Texture(uniform_ref) = uniform_param {
                            rc.set_uniform(uniform_ref, bound.binding());
                        }
                    }
                }

                // Rendering quad
                rc.render(RenderState::default(), |mut tc| {
                    tc.draw(&quad)
                })
            })
        });
    }

    /// Renders a quad using provided shader and binds provided Gray color slots
    pub fn render_quad_pass_gray<C1, D1, S>(
        ctx: &mut Context, target_frame: &LumFrameBuffer<C1, D1>,
        shader: &mut LumProgram<S>, color_slots: HashMap<String, &mut Texture<GL33, Dim2, LumGray>>
    ) where
        C1: ColorSlot<GL33, Dim2>,
        D1: DepthSlot<GL33, Dim2>,
        S: UniformInterface<GL33> + MeshShader<S>
    {
        // Creating quad to draw with
        let quad = ctx.new_quad();

        // Creating pipeline
        ctx.pipeline(target_frame, PipelineState::default(), |pc, mut sc| {
            // Binding frame's color slots
            let mut bound_textures = HashMap::new();

            for (name, texture) in color_slots {
                bound_textures.insert(name, pc.bind_texture(texture));
            }

            // Using the final pass shader
            sc.use_shader(shader, |mut rc, uni| {
                let params = uni.parameters();

                // Passing frame texture to the shader
                for (name, bound) in bound_textures {
                    if let Some(uniform_param) = params.get(&name) {
                        if let UniformParameter::Texture(uniform_ref) = uniform_param {
                            rc.set_uniform(uniform_ref, bound.binding());
                        }
                    }
                }

                // Rendering quad
                rc.render(RenderState::default(), |mut tc| {
                    tc.draw(&quad)
                })
            })
        });
    }

    /// Renders a quad using provided shader, binds provided RGB color slots and depth slot
    pub fn render_quad_pass_rgb_depth<C1, D1, S>(
        ctx: &mut Context, target_frame: &LumFrameBuffer<C1, D1>,
        shader: &mut LumProgram<S>, color_slots: HashMap<String, &mut Texture<GL33, Dim2, LumRGB>>, depth_slot: (String, &mut Texture<GL33, Dim2, LumDepth>)
    ) where
        C1: ColorSlot<GL33, Dim2>,
        D1: DepthSlot<GL33, Dim2>,
        S: UniformInterface<GL33> + MeshShader<S>
    {
        // Creating quad to draw with
        let quad = ctx.new_quad();

        // Creating pipeline
        ctx.pipeline(target_frame, PipelineState::default(), |pc, mut sc| {
            // Binding frame's color slots
            let mut bound_textures = HashMap::new();

            for (name, texture) in color_slots {
                bound_textures.insert(name, pc.bind_texture(texture));
            }

            let (depth_name, depth_texture) = depth_slot;
            let depth_bound = pc.bind_texture(depth_texture);

            // Using the final pass shader
            sc.use_shader(shader, |mut rc, uni| {
                let params = uni.parameters();

                // Passing frame texture to the shader
                for (name, bound) in bound_textures {
                    if let Some(uniform_param) = params.get(&name) {
                        if let UniformParameter::Texture(uniform_ref) = uniform_param {
                            rc.set_uniform(uniform_ref, bound.binding());
                        }
                    }
                }

                // Passing depth texture
                {
                    let (name, bound) = (depth_name, depth_bound);

                    if let Some(uniform_param) = params.get(&name) {
                        if let UniformParameter::DepthTexture(uniform_ref) = uniform_param {
                            rc.set_uniform(uniform_ref, bound.binding());
                        }
                    }
                }

                // Rendering quad
                rc.render(RenderState::default(), |mut tc| {
                    tc.draw(&quad)
                })
            })
        });
    }

    /// Renders a quad using provided shader, binds provided RGBA color slots and depth slot
    pub fn render_quad_pass_rgba_depth<C1, D1, S>(
        ctx: &mut Context, target_frame: &LumFrameBuffer<C1, D1>,
        shader: &mut LumProgram<S>, color_slots: HashMap<String, &mut Texture<GL33, Dim2, LumRGBA>>, depth_slot: (String, &mut Texture<GL33, Dim2, LumDepth>)
    ) where
        C1: ColorSlot<GL33, Dim2>,
        D1: DepthSlot<GL33, Dim2>,
        S: UniformInterface<GL33> + MeshShader<S>
    {
        // Creating quad to draw with
        let quad = ctx.new_quad();

        // Creating pipeline
        ctx.pipeline(target_frame, PipelineState::default(), |pc, mut sc| {
            // Binding frame's color slots
            let mut bound_textures = HashMap::new();

            for (name, texture) in color_slots {
                bound_textures.insert(name, pc.bind_texture(texture));
            }

            let (depth_name, depth_texture) = depth_slot;
            let depth_bound = pc.bind_texture(depth_texture);

            // Using the final pass shader
            sc.use_shader(shader, |mut rc, uni| {
                let params = uni.parameters();

                // Passing frame texture to the shader
                for (name, bound) in bound_textures {
                    if let Some(uniform_param) = params.get(&name) {
                        if let UniformParameter::Texture(uniform_ref) = uniform_param {
                            rc.set_uniform(uniform_ref, bound.binding());
                        }
                    }
                }

                // Passing depth texture
                {
                    let (name, bound) = (depth_name, depth_bound);

                    if let Some(uniform_param) = params.get(&name) {
                        if let UniformParameter::DepthTexture(uniform_ref) = uniform_param {
                            rc.set_uniform(uniform_ref, bound.binding());
                        }
                    }
                }

                // Rendering quad
                rc.render(RenderState::default(), |mut tc| {
                    tc.draw(&quad)
                })
            })
        });
    }

    /// Renders a quad using provided shader, binds provided Gray color slots and depth slot
    pub fn render_quad_pass_gray_depth<C1, D1, S>(
        ctx: &mut Context, target_frame: &LumFrameBuffer<C1, D1>,
        shader: &mut LumProgram<S>, color_slots: HashMap<String, &mut Texture<GL33, Dim2, LumGray>>, depth_slot: (String, &mut Texture<GL33, Dim2, LumDepth>)
    ) where
        C1: ColorSlot<GL33, Dim2>,
        D1: DepthSlot<GL33, Dim2>,
        S: UniformInterface<GL33> + MeshShader<S>
    {
        // Creating quad to draw with
        let quad = ctx.new_quad();

        // Creating pipeline
        ctx.pipeline(target_frame, PipelineState::default(), |pc, mut sc| {
            // Binding frame's color slots
            let mut bound_textures = HashMap::new();

            for (name, texture) in color_slots {
                bound_textures.insert(name, pc.bind_texture(texture));
            }

            let (depth_name, depth_texture) = depth_slot;
            let depth_bound = pc.bind_texture(depth_texture);

            // Using the final pass shader
            sc.use_shader(shader, |mut rc, uni| {
                let params = uni.parameters();

                // Passing frame texture to the shader
                for (name, bound) in bound_textures {
                    if let Some(uniform_param) = params.get(&name) {
                        if let UniformParameter::Texture(uniform_ref) = uniform_param {
                            rc.set_uniform(uniform_ref, bound.binding());
                        }
                    }
                }

                // Passing depth texture
                {
                    let (name, bound) = (depth_name, depth_bound);

                    if let Some(uniform_param) = params.get(&name) {
                        if let UniformParameter::DepthTexture(uniform_ref) = uniform_param {
                            rc.set_uniform(uniform_ref, bound.binding());
                        }
                    }
                }

                // Rendering quad
                rc.render(RenderState::default(), |mut tc| {
                    tc.draw(&quad)
                })
            })
        });
    }

    /// Renders a quad using provided shader, binds provided depth slot
    pub fn render_quad_pass_depth<C1, D1, S>(
        ctx: &mut Context, target_frame: &LumFrameBuffer<C1, D1>,
        shader: &mut LumProgram<S>, depth_slot: (String, &mut Texture<GL33, Dim2, LumDepth>)
    ) where
        C1: ColorSlot<GL33, Dim2>,
        D1: DepthSlot<GL33, Dim2>,
        S: UniformInterface<GL33> + MeshShader<S>
    {
        // Creating quad to draw with
        let quad = ctx.new_quad();

        // Creating pipeline
        ctx.pipeline(target_frame, PipelineState::default(), |pc, mut sc| {
            let (depth_name, depth_texture) = depth_slot;
            let depth_bound = pc.bind_texture(depth_texture);

            // Using the final pass shader
            sc.use_shader(shader, |mut rc, uni| {
                let params = uni.parameters();

                // Passing depth texture
                {
                    let (name, bound) = (depth_name, depth_bound);

                    if let Some(uniform_param) = params.get(&name) {
                        if let UniformParameter::DepthTexture(uniform_ref) = uniform_param {
                            rc.set_uniform(uniform_ref, bound.binding());
                        }
                    }
                }

                // Rendering quad
                rc.render(RenderState::default(), |mut tc| {
                    tc.draw(&quad)
                })
            })
        });
    }
}

