use crate::data::shader::*;
use crate::shd_interface;
use crate::data::LumTextureBinding;
use crate::types::{Shader, UniformParameter};
use std::collections::HashMap;

pub const LIGHT_CHUNKS: u32 = 250;

shd_interface![
    LightPassShader,

    camera_pos, [f32; 3],
    normal_texture, LumTextureBinding,
    position_texture, LumTextureBinding,
    rms_texture, LumTextureBinding,
    light_count, i32
];

impl Shader<LightPassShader> for LightPassShader {
    fn vertex_source() -> String {
        r#"
        in vec3 position;
        in vec3 normal;
        in vec3 color;

        out vec2 uv;

        void main() {
            vec2 l_uv = (position.xy + 1.0) / 2.0;

            uv = l_uv;

            gl_Position = vec4(position, 1.);
        }
        "#.to_string()
    }

    fn fragment_source() -> String {
        r#"
        in vec2 uv;

        uniform vec3 camera_pos;

        uniform sampler2D normal_texture;
        uniform sampler2D position_texture;
        uniform sampler2D rms_texture;

        uniform vec3 light_pos[LIGHT_CHUNKS];
        uniform vec3 light_color[LIGHT_CHUNKS];
        uniform float light_intensity[LIGHT_CHUNKS];
        uniform float light_linear[LIGHT_CHUNKS];
        uniform float light_quadratic[LIGHT_CHUNKS];
        uniform float light_radius[LIGHT_CHUNKS];
        uniform int light_count;

        out vec4 frag_color;

        void main() {
            vec3 frag_pos = texture(position_texture, uv).rgb;
            vec3 normal = texture(normal_texture, uv).rgb;
            vec3 rms = texture(rms_texture, uv).rgb;

            float specular = rms.z;

            vec3 lighting = vec3(0.0, 0.0, 0.0);
            vec3 view_dir = normalize(camera_pos - frag_pos);

            for (int i = 0; i < light_count; i++) {
                float distance = length(light_pos[i] - frag_pos);

                if(distance < light_radius[i]) {
                    vec3 light_dir = normalize(light_pos[i] - frag_pos);
                    vec3 diffused = dot(normal, light_dir) * (light_color[i] * light_intensity[i]);

                    vec3 halfway_dir = normalize(light_dir + view_dir);
                    float spec = pow(max(dot(normal, halfway_dir), 0.0), 16.0);
                    vec3 speculared = light_color[i] * spec * specular;

                    float attenuation = 1.0 / (1.0 + light_linear[i] * distance + light_quadratic[i] * distance * distance);

                    diffused *= attenuation;
                    speculared *= attenuation;
                    lighting += diffused + speculared;
                }
            }

            frag_color = vec4(lighting, 1.0);
        }
        "#.to_string().replace("LIGHT_CHUNKS", &LIGHT_CHUNKS.to_string())
    }

    fn parameters(&self) -> HashMap<String, UniformParameter> {
        let mut map = HashMap::new();

        map.insert("camera_pos".to_string(), UniformParameter::Vector3(&self.camera_pos));

        map.insert("normal_texture".to_string(), UniformParameter::Texture(&self.normal_texture));
        map.insert("position_texture".to_string(), UniformParameter::Texture(&self.position_texture));
        map.insert("rms_texture".to_string(), UniformParameter::Texture(&self.rms_texture));

        map.insert("light_pos".to_string(), UniformParameter::Vector3Array(LIGHT_CHUNKS));
        map.insert("light_color".to_string(), UniformParameter::Vector3Array(LIGHT_CHUNKS));
        map.insert("light_intensity".to_string(), UniformParameter::FloatArray(LIGHT_CHUNKS));

        map.insert("light_linear".to_string(), UniformParameter::FloatArray(LIGHT_CHUNKS));
        map.insert("light_quadratic".to_string(), UniformParameter::FloatArray(LIGHT_CHUNKS));
        map.insert("light_radius".to_string(), UniformParameter::FloatArray(LIGHT_CHUNKS));


        map.insert("light_count".to_string(), UniformParameter::Integer(&self.light_count));

        map
    }
}