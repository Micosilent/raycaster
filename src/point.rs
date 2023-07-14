use crate::vec3::Vector3;
#[derive(Copy, Clone)]
pub struct Point {
    pub vector: Vector3,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            vector: Vector3 { x, y, z },
        }
    }
    pub fn get_translated(self, vector: Vector3) -> Point {
        return Point::new(
            self.vector.x + vector.x,
            self.vector.y + vector.y,
            self.vector.z + vector.z,
        );
    }
}
