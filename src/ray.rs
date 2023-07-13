use crate::{point::Point, vec3::Vector3};
#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}
impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn at(self, t: f64) -> Vector3 {
        self.origin + self.direction * t
    }
    pub fn distance_to_point(self, point: Point) -> f64 {
        let v = point.vector - self.origin;
        let u = self.direction;

        let d = v - u * (v.dot(u) / u.magnitude_squared());

        return d.magnitude();
    }
}
