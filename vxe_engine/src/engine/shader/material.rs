use vxe_renderer::data::{UniformInterface, GL33, LumProgram};
use vxe_renderer::context::Context;

pub trait Material {
    fn use_diffuse_shader(&self);
}