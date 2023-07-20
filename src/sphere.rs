use crate::{
    hit::{HitRecord, Hittable},
    point::Point,
    ray::Ray,
    vec3::Vector3,
};

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let points = ray.intersection_to_sphere(self.center, self.radius);
        if points.len() == 0 {
            return None;
        } else if points.len() == 1 {
            let normal_vector = Vector3::new_from_2_points(self.center, points[0]);
            let t = Vector3::new_from_2_points(ray.origin.as_point(), points[0]).magnitude()
                / ray.direction.magnitude();
            let front_face = return Some(HitRecord {
                hit_point: points[0],
                normal_vector,
                t,
            });
        }
        let normal_vector_0 = Vector3::new_from_2_points(self.center, points[0]);
        let normal_vector_1 = Vector3::new_from_2_points(self.center, points[1]);

        let origin_to_intersection_0 = Vector3::new_from_2_points(ray.origin.as_point(), points[0]);

        //TODO: Check for tmin and tmax

        if normal_vector_0.dot(origin_to_intersection_0) < 0.0 {
            let t = Vector3::new_from_2_points(ray.origin.as_point(), points[0]).magnitude()
                / ray.direction.magnitude();
            return Some(HitRecord {
                hit_point: points[0],
                normal_vector: normal_vector_0,
                t,
            });
        } else {
            let t = Vector3::new_from_2_points(ray.origin.as_point(), points[1]).magnitude()
                / ray.direction.magnitude();
            return Some(HitRecord {
                hit_point: points[1],
                normal_vector: normal_vector_1,
                t,
            });
        }
    }
}
