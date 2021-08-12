use cgmath::Vector3;

const CON: f32 = 1.0;

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

    pub fn radius(&self) -> f32 {
        let lin = self.linear_attenuation;
        let quad = self.quadratic_attenuation;

        let max_bright =  f32::max(f32::max(self.color[0] * self.intensity, self.color[1] * self.intensity), self.color[2] * self.intensity);

        (-lin + f32::sqrt(lin * lin - 4.0 * quad * (CON - (256.0 / 5.0) * max_bright))) / (2.0 * quad)
    }
}