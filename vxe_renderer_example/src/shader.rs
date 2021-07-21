use vxe_renderer::shd_interface;
use vxe_renderer::data::shader::*;

use vxe_renderer::types::{MeshShader, UniformParameter};
use std::collections::HashMap;

shd_interface!(
    TestShader,
    projection, [[f32; 4]; 4],
    view, [[f32; 4]; 4],
    trs, [[f32; 4]; 4]
);

impl MeshShader<TestShader> for TestShader {
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
            vec4 scr_pos = projection * view * trs * vec4(position, 1.);

            v_position = scr_pos.xyz;
            v_normal = normal;
            v_color = color;

            gl_Position = scr_pos;
        }
        "#.to_string()
    }

    fn fragment_source() -> String {
        r#"
        in vec3 v_position;
        in vec3 v_normal;
        in vec3 v_color;

        uniform mat4 trs;

        layout (location = 0) out vec3 frag_color;

        void main() {
            vec4 light = vec4(0.3, -0.3, -0.5, 1.0);

            vec4 transformed_normal = trs * vec4(v_normal, 1.0);

            float kd = dot(transformed_normal.xyz, -light.xyz);

            frag_color = v_color * (kd / 1.3 + 0.5);
        }
        "#.to_string()
    }

    fn parameters(&self) -> HashMap<String, UniformParameter> {
        let mut map = HashMap::new();

        map.insert("projection".to_string(),  UniformParameter::Matrix4(&self.projection));
        map.insert("view".to_string(),  UniformParameter::Matrix4(&self.view));
        map.insert("trs".to_string(),  UniformParameter::Matrix4(&self.trs));

        map
    }
}