#[repr(C)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn length(&self) -> f64 {
        let sum = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        return sum.sqrt();
    }
}
