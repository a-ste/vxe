use vxe_renderer::data::shader::Uniform;
use vxe_renderer::data::{UniformInterface, GL33, LumProgram};
use vxe_renderer::context::Context;

pub trait MeshShader<I>
    where
        I: UniformInterface<GL33>
{
    fn new(ctx: &mut Context) -> LumProgram<I> {
        ctx.new_shader_program(Self::vertex_source().as_str(), Self::fragment_source().as_str())
    }

    fn vertex_source() -> String;
    fn fragment_source() -> String;

    fn projection_uni(&self) -> Option<&Uniform<[[f32; 4]; 4]>>;
    fn view_uni(&self) -> Option<&Uniform<[[f32; 4]; 4]>>;
}