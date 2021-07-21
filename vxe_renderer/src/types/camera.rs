use crate::types::Transform;
use cgmath::{perspective, Deg, Matrix4, Vector3, Point3};
use crate::context::Context;

pub struct Camera {
    pub transform: Transform,
    pub fov: f32,
    pub near: f32,
    pub far: f32
}

impl Camera {
    pub fn new(trs: Transform, fov: f32, near: f32, far: f32) -> Camera {
        Camera {
            fov,
            transform: trs,
            near, far
        }
    }

    /// Generates projection and view matrices
    pub fn matrices(&self, ctx: &mut Context) -> ([[f32; 4]; 4], [[f32; 4]; 4]) {
        let res = ctx.resolution();
        let aspect = res[0] as f32 / res[1] as f32;

        let persp = perspective(Deg(self.fov), aspect, self.near, self.far);

        let forward = self.transform.rotation * Vector3::<f32>::new(1.0, 0.0, 0.0);
        let up = self.transform.rotation * Vector3::<f32>::new(0.0, 0.0, 1.0);

        let look_at_position = self.transform.position + forward;

        let eye: (f32, f32, f32) = self.transform.position.into();
        let center: (f32, f32, f32) = look_at_position.into();

        let view = Matrix4::look_at_rh(Point3::from(eye), Point3::from(center), up);

        (persp.into(), view.into())
    }
}