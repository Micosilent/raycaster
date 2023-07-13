use std::fs;

mod color;
mod point;
mod ray;
mod vec3;

use point::Point;
use ray::Ray;
use vec3::Vector3;

use crate::color::Color;

fn main() {
    //Image settings
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    print!(
        "Aspect Ratio: {}, imageWidth {} imageHeight {}",
        aspect_ratio, image_width, image_height
    );
    let mut data = format!(
        "P3\n {width} {height} \n255\n",
        width = image_width,
        height = image_height
    );

    //Camera Settings
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin.vector
        - (horizontal / 2.0).unwrap()
        - (vertical / 2.0).unwrap()
        - Vector3::new(0.0, 0.0, focal_length);

    for row in (0..image_height).rev() {
        // println!("Printing row: {}", row);
        for column in 0..image_width {
            // Generate RGB values for the pixel at (row, column)
            let u = column as f64 / (image_width - 1) as f64;
            let v = row as f64 / (image_height - 1) as f64;
            println!("Column: {}, Row: {}", column, row);
            println!("U: {}, V: {}", u, v);

            let direction = lower_left_corner + horizontal * u + vertical * v - origin.vector;
            let ray = Ray::new(origin.vector, direction);

            let mut pixel_color = ray_color(ray);

            pixel_color.scale();

            data.push_str(&pixel_color.to_string())
        }
    }

    fs::write("./firstImage.ppm", data).expect("Unable to write file");
}

fn ray_color(ray: Ray) -> Color {
    if hit_sphere(Point::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction: Vector3 = ray.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);

    let vector: Vector3 = Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t;

    return Color::new(vector.x, vector.y, vector.z);
}

fn hit_sphere(center: Point, sphere_radius: f64, ray: Ray) -> bool {
    let distance = ray.distance_to_point(center);

    return distance <= sphere_radius;
}
