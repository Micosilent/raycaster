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
    pub fn intersection_to_sphere(self, sphere_center: Point, sphere_radius: f64) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();

        let v: Vector3 = sphere_center.vector - self.origin;
        let u: Vector3 = self.direction;

        let d: Vector3 = u * (v.dot(u) / u.magnitude_squared()) - v;

        if d.magnitude() == sphere_radius {
            points.push(sphere_center.get_translated(d));
        } else if d.magnitude() < sphere_radius {
            let x = (sphere_radius.powf(2.0) - d.magnitude_squared()).sqrt();

            points.push(sphere_center.get_translated(d + ((u * x) / u.magnitude()).unwrap()));
            points.push(sphere_center.get_translated(d - ((u * x) / u.magnitude()).unwrap()));
        }

        points
    }
}
