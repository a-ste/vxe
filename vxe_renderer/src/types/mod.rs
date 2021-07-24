mod mesh;
mod mesh_shader;
mod parameter;
mod material;
mod transform;
mod camera;

pub use mesh::Mesh;
pub use mesh_shader::MeshShader;
pub use parameter::Parameter;
pub use parameter::UniformParameter;
pub use material::Material;
pub use transform::Transform;
pub use camera::Camera;

use crate::data::{LumFrameBuffer, LumRGB, LumDepth};

/// Framebuffer that will be used by deferred rendering
pub type DeferredFrameBuffer = LumFrameBuffer<(LumRGB), LumDepth>;

pub use material::default_pipeline;
pub use material::default_render_state;