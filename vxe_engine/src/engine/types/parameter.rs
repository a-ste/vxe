use vxe_renderer::data::shader::Uniform;

pub enum Parameter {
    Float(f32),
    Matrix4([[f32; 4]; 4]),
}

pub enum UniformParameter<'a> {
    Float(&'a Uniform<f32>),
    Matrix4(&'a Uniform<[[f32; 4]; 4]>),
}