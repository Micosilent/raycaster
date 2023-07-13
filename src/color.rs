use std::string;

use crate::vec3::Vector3;
#[derive(Copy, Clone)]
pub struct Color {
    vector: Vector3,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            vector: Vector3::new(r, g, b),
        }
    }

    pub fn scale(&mut self) {
        let max: f64 = 255.0;
        let min: f64 = 0.0;

        self.vector = self.vector * (max - min) + min;
    }

    pub fn to_string(self) -> String {
        format!(
            "{red} {green} {blue} \n",
            red = self.vector.x,
            green = self.vector.y,
            blue = self.vector.z
        )
    }
}
