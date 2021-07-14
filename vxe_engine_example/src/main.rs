use vxe_engine::shader::{MeshShader};
use vxe_engine::shader::shd_interface;
use vxe_engine::shader::shd_interface_uses::*;
use std::collections::HashMap;
use vxe_engine::types::UniformParameter;

shd_interface!(
    MyMeshShader,
    projection, [[f32; 4]; 4],
    view, [[f32; 4]; 4]
);

impl MeshShader<MyMeshShader> for MyMeshShader {
    fn vertex_source() -> String {
        r#"
        in vec3 position;
        in vec3 color;

        uniform mat4 projection;
        uniform mat4 view;

        out vec3 v_color;
        out vec3 v_position;

        void main() {
            v_color = color;
            v_position = position;

            gl_Position = projection * view * vec4(position, 1.0);
        }
        "#.to_string()
    }

    fn fragment_source() -> String {
        r#"
        in vec3 v_position;
        in vec3 v_color;

        out vec4 frag_color;

        void main() {
            frag_color = vec4(v_color, 1.0);
        }
        "#.to_string()
    }

    fn parameters(&self) -> HashMap<String, UniformParameter> {
        let mut map = HashMap::new();

        map.insert("projection".to_string(), UniformParameter::Matrix4(&self.projection));
        map.insert("view".to_string(), UniformParameter::Matrix4(&self.view));

        map
    }
}

fn main() {


}
