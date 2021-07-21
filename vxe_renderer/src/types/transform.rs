use cgmath::{Vector3, Quaternion, Matrix4};

pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: Vector3<f32>,
}

impl Transform {
    pub fn default() -> Transform {
        Transform {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0)
        }
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        let pos_mat = Matrix4::from_translation(self.position);
        let rot_mat = Matrix4::from(self.rotation);
        let scl_mat = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);

        pos_mat * rot_mat * scl_mat
    }

    pub fn raw_matrix(&self) -> [[f32; 4]; 4] {
        self.matrix().into()
    }
}