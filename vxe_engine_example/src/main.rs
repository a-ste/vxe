use vxe_engine::shader::{MeshShader};
use vxe_engine::shader::shd_interface;
use vxe_engine::shader::shd_interface_uses::*;

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

    fn projection_uni(&self) -> Option<&Uniform<[[f32; 4]; 4]>> {
        Some(&self.projection)
    }

    fn view_uni(&self) -> Option<&Uniform<[[f32; 4]; 4]>> {
        Some(&self.view)
    }
}

fn main() {


}
