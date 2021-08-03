use crate::data::shader::Uniform;
use crate::data::{LumTextureBinding, LumDepthBinding};
use crate::context::RenderContext;
use std::collections::HashMap;

pub enum Parameter {
    Float(f32),
    Matrix4([[f32; 4]; 4]),
}

pub enum UniformParameter<'a> {
    Float(&'a Uniform<f32>),
    Vector3(&'a Uniform<[f32; 3]>),
    Matrix4(&'a Uniform<[[f32; 4]; 4]>),
    Texture(&'a Uniform<LumTextureBinding>),
    DepthTexture(&'a Uniform<LumDepthBinding>),
}

impl UniformParameter<'_> {
    pub fn float_uniform(rc: &mut RenderContext, params: &HashMap<String, UniformParameter>, param: &str, value: f32) {
        if let Some(enm) = params.get(param) {
            if let UniformParameter::Float(uniform) = enm {
                rc.set_uniform(uniform, value);
            }
        }
    }

    pub fn vector3_uniform(rc: &mut RenderContext, params: &HashMap<String, UniformParameter>, param: &str, value: [f32; 3]) {
        if let Some(enm) = params.get(param) {
            if let UniformParameter::Vector3(uniform) = enm {
                rc.set_uniform(uniform, value);
            }
        }
    }

    pub fn matrix4_uniform(rc: &mut RenderContext, params: &HashMap<String, UniformParameter>, param: &str, value: [[f32; 4]; 4]) {
        if let Some(enm) = params.get(param) {
            if let UniformParameter::Matrix4(uniform) = enm {
                rc.set_uniform(uniform, value);
            }
        }
    }

    pub fn texture_uniform(rc: &mut RenderContext, params: &HashMap<String, UniformParameter>, param: &str, value: LumTextureBinding) {
        if let Some(enm) = params.get(param) {
            if let UniformParameter::Texture(uniform) = enm {
                rc.set_uniform(uniform, value);
            }
        }
    }

    pub fn depth_uniform(rc: &mut RenderContext, params: &HashMap<String, UniformParameter>, param: &str, value: LumDepthBinding) {
        if let Some(enm) = params.get(param) {
            if let UniformParameter::DepthTexture(uniform) = enm {
                rc.set_uniform(uniform, value);
            }
        }
    }
}