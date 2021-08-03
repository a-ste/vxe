mod mesh;
mod mesh_shader;
mod parameter;
mod material;
mod transform;
mod camera;
mod light;

pub use mesh::Mesh;
pub use mesh_shader::Shader;
pub use parameter::Parameter;
pub use parameter::UniformParameter;
pub use material::Material;
pub use transform::Transform;
pub use camera::Camera;
pub use light::Light;

use crate::data::{LumFrameBuffer, LumRGB, LumDepth};

/// Framebuffer that will be used by deferred rendering, uses following slots: Diffuse, Normal, Position, Roughness-Metallic-Specular
pub type DeferredFrameBuffer = LumFrameBuffer<(LumRGB, LumRGB, LumRGB, LumRGB), LumDepth>;

pub use material::default_pipeline;
pub use material::default_render_state;