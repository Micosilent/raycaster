use crate::point::Point;
use crate::ray::Ray;
use crate::vec3::Vector3;

pub struct HitRecord {
    pub hit_point: Point,
    pub normal_vector: Vector3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
