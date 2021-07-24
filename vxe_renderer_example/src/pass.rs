use vxe_renderer::shd_interface;
use vxe_renderer::data::shader::*;

use vxe_renderer::types::{MeshShader, UniformParameter};
use std::collections::HashMap;
use vxe_renderer::data::{LumTextureBinding};

shd_interface!(
    FinalPass,
    frame, LumTextureBinding
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
            frag_color = texture2D(frame, v_position);
        }
        "#.to_string()
    }

    fn parameters(&self) -> HashMap<String, UniformParameter> {
        let mut map = HashMap::new();

        map.insert("frame".to_string(),  UniformParameter::Texture(&self.frame));

        map
    }
}