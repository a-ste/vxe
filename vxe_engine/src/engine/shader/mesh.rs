use vxe_renderer::data::shader::Uniform;
use vxe_renderer::data::{UniformInterface, GL33, LumProgram};
use vxe_renderer::context::Context;
use std::collections::HashMap;
use crate::types::UniformParameter;

pub trait MeshShader<I>
    where
        I: UniformInterface<GL33>
{
    fn new(ctx: &mut Context) -> LumProgram<I> {
        ctx.new_shader_program(Self::vertex_source().as_str(), Self::fragment_source().as_str())
    }

    fn vertex_source() -> String;
    fn fragment_source() -> String;

    fn parameters(&self) -> HashMap<String, UniformParameter>;
}