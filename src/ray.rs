use crate::vec3::Vector3;

pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}
impl Ray {
    pub fn at(self, t: f64) -> Vector3 {
        self.origin + t * self.direction
    }
}
