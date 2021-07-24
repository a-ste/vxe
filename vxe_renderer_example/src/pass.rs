use vxe_renderer::shd_interface;
use vxe_renderer::data::shader::*;

use vxe_renderer::types::{MeshShader, UniformParameter};
use std::collections::HashMap;
use vxe_renderer::data::{LumTextureBinding, LumDepthBinding};

shd_interface!(
    FinalPass,
    frame, LumDepthBinding
);

impl MeshShader<FinalPass> for FinalPass {
    fn vertex_source() -> String {
        r#"
        in vec3 position;
        in vec3 normal;
        in vec3 color;

        out vec2 v_position;

        void main() {
            vec2 uv = (position.xy + 1.0) / 2.0;

            v_position = uv;

            gl_Position = vec4(position, 1.);
        }
        "#.to_string()
    }

    fn fragment_source() -> String {
        r#"
        in vec2 v_position;

        uniform sampler2D frame;

        out vec4 frag_color;

        void main() {
            float depth = pow(texture2D(frame, v_position).x, 20.0);

            frag_color = vec4(depth, depth, depth, 1.);
        }
        "#.to_string()
    }

    fn parameters(&self) -> HashMap<String, UniformParameter> {
        let mut map = HashMap::new();

        map.insert("frame".to_string(),  UniformParameter::DepthTexture(&self.frame));

        map
    }
}