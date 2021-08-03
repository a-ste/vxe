use crate::data::shader::*;
use crate::shd_interface;
use crate::data::LumTextureBinding;
use crate::types::{Shader, UniformParameter};
use std::collections::HashMap;

shd_interface![
    FinalLightPassShader,
    diffuse_texture, LumTextureBinding,
    light_texture, LumTextureBinding
];

impl Shader<FinalLightPassShader> for FinalLightPassShader {
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

        uniform sampler2D diffuse_texture;
        uniform sampler2D light_texture;

        out vec4 frag_color;

        void main() {
            frag_color = texture2D(diffuse_texture, v_position) * (texture2D(light_texture, v_position) + 0.02);
        }
        "#.to_string()
    }

    fn parameters(&self) -> HashMap<String, UniformParameter> {
        let mut map = HashMap::new();

        map.insert("diffuse_texture".to_string(), UniformParameter::Texture(&self.diffuse_texture));
        map.insert("light_texture".to_string(), UniformParameter::Texture(&self.light_texture));

        map
    }
}