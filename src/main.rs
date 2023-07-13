use std::fs;

mod color;
mod point;
mod vec3;

use crate::color::Color;

fn main() {
    // Image size
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    let mut data = format!(
        "P3\n {width} {height} \n255\n",
        width = image_width,
        height = image_height
    );

    for row in (0..image_height).rev() {
        println!("Printing row: {}", row);
        for column in 0..image_width {
            // Generate RGB values for the pixel at (row, column)

            let mut pixel_color = Color::new(
                column as f64 / ((image_width - 1) as f64),
                row as f64 / ((image_height - 1) as f64),
                0.25,
            );

            pixel_color.scale();

            data.push_str(&pixel_color.to_string())
        }
    }

    fs::write("./firstImage.ppm", data).expect("Unable to write file");
}
