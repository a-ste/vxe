use vxe_renderer::data::{UniformInterface, GL33, LumProgram};
use vxe_renderer::context::Context;
use std::collections::HashMap;
use crate::types::Parameter;

pub trait Material {
    fn use_diffuse_shader(&self);

    fn name(&self) -> String;

    fn set_parameter(&mut self, name: String, parameter: Parameter);
    fn parameters(&self) -> HashMap<String, Parameter>;
}