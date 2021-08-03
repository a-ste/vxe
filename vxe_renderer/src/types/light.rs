use cgmath::Vector3;

pub struct Light {
    pub position: Vector3<f32>,
    pub color: [f32; 3],
    pub intensity: f32,
    pub linear_attenuation: f32,
    pub quadratic_attenuation: f32
}

impl Light {
    pub fn new(position: [f32; 3]) -> Light {
        Light {
            position: Vector3::new(position[0], position[1], position[2]),
            color: [1.0, 1.0, 1.0],
            intensity: 1.5,
            linear_attenuation: 0.7,
            quadratic_attenuation: 1.8
        }
    }
}