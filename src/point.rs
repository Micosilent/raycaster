use crate::vec3::Vector3;

pub struct Point {
    pub vector: Vector3,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            vector: Vector3 { x, y, z },
        }
    }
}
