use std::fs;
use vec3::Vector3;

mod vec3;

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
            let red_value: f64 = column as f64 / ((image_width - 1) as f64);
            let green_value: f64 = row as f64 / ((image_height - 1) as f64);
            let blue_value: f64 = 0.25;

            let normalized_red_value: f64 = scale(red_value);
            let normalized_green_value: f64 = scale(green_value);
            let normalized_blue_value: f64 = scale(blue_value);

            // Write the values to the file
            let pixel: String = format!(
                "{red} {green} {blue} \n",
                red = normalized_red_value,
                green = normalized_green_value,
                blue = normalized_blue_value
            );

            data.push_str(&pixel)
        }
    }

    fs::write("./firstImage.ppm", data).expect("Unable to write file");
}

fn scale(value: f64) -> f64 {
    let max: f64 = 255.0;
    let min: f64 = 0.0;

    return value * (max - min) + min;
}
