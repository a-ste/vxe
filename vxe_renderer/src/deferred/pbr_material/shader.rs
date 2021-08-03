use crate::shd_interface;
use crate::data::shader::*;
use crate::types::{Shader, UniformParameter};
use std::collections::HashMap;

shd_interface![
    PBRShader,
    trs, [[f32; 4]; 4],
    projection, [[f32; 4]; 4],
    view, [[f32; 4]; 4],
    color, [f32; 3],
    rms, [f32; 3]
];

impl Shader<PBRShader> for PBRShader {
    fn vertex_source() -> String {
        r#"
        in vec3 position;
        in vec3 normal;
        in vec3 color;

        uniform mat4 projection;
        uniform mat4 view;
        uniform mat4 trs;

        out vec3 v_position;
        out vec3 v_normal;
        out vec3 v_color;

        void main() {
            vec4 world_pos = trs * vec4(position, 1.);

            v_position = world_pos.xyz;

            mat3 normalMatrix = inverse(mat3(trs));
            v_normal = normal * normalMatrix;
            v_color = color;

            gl_Position = (projection * view * world_pos);
        }
        "#.to_string()
    }

    fn fragment_source() -> String {
        r#"
        in vec3 v_position;
        in vec3 v_normal;
        in vec3 v_color;

        uniform vec3 color;
        uniform vec3 rms;

        layout (location = 0) out vec3 o_diffuse;
        layout (location = 1) out vec3 o_normal;
        layout (location = 2) out vec3 o_position;
        layout (location = 3) out vec3 o_rms;

        void main() {
            o_diffuse = color;
            o_normal = normalize(v_normal);
            o_position = v_position;
            o_rms = rms;
        }
        "#.to_string()
    }

    fn parameters(&self) -> HashMap<String, UniformParameter> {
        let mut map = HashMap::new();

        map.insert("trs".to_string(), UniformParameter::Matrix4(&self.trs));
        map.insert("projection".to_string(), UniformParameter::Matrix4(&self.projection));
        map.insert("view".to_string(), UniformParameter::Matrix4(&self.view));
        map.insert("color".to_string(), UniformParameter::Vector3(&self.color));
        map.insert("rms".to_string(), UniformParameter::Vector3(&self.rms));

        map
    }
}