use image::{ImageBuffer, RgbImage};
use rand::rng;
use rand::seq::SliceRandom;
use std::f32::consts::SQRT_2;
use structopt::StructOpt;

#[derive(Debug)]
struct Mathematical2DVector {
    direction_x: f32,
    direction_y: f32,
}
impl Mathematical2DVector {
    fn new(direction_x: f32, direction_y: f32) -> Mathematical2DVector {
        Mathematical2DVector {
            direction_x,
            direction_y,
        }
    }
    fn compute_dot_product(&self, other_vector: &Mathematical2DVector) -> f32 {
        self.direction_x * other_vector.direction_x + self.direction_y * other_vector.direction_y
    }
}


#[derive(Debug, StructOpt)]
#[structopt(name = "perlin-noise", about = "I wanted to understand the Perlin Noise Algorithm")]
struct Opt {
    #[structopt(short, long, help="Put this flag to generate a black and white image instead of numeric values")]
    generate_image: bool,

    #[structopt(long, default_value="1000", help="The amount of values you want to generate on the X axis")]
    width: u32,

    #[structopt(long, default_value="1000", help="The amount of values you want to generate on the Y axis")]
    height: u32,

    #[structopt(short, long, default_value="500", help="The size of the chunks you want to generate. ")]
    chunk_size: u32,

    #[structopt(short, long, default_value="1", help="Number of images to generate.")]
    number_of_images_to_generate: u32,
}

fn generate_permutation_table() -> [usize; 512] {
    let mut rng = rng();
    let mut random_values = Vec::new();
    for i in 0..256 {
        random_values.push(i);
    }
    random_values.shuffle(&mut rng);

    let mut final_permutation_table = [1; 512];

    for i in 0..512 {
        let index: usize = i & 255;
        final_permutation_table[i] = random_values[index];
    }
    final_permutation_table
}

fn get_corner_vector(value_from_perm_table: usize) -> Mathematical2DVector {
    let normalized_value = value_from_perm_table & 7;
    let direction_for_length_1_vector: f32 = 1.0 / SQRT_2;
    match normalized_value {
        0 => {
            Mathematical2DVector::new(direction_for_length_1_vector, direction_for_length_1_vector)
        }
        1 => Mathematical2DVector::new(
            -direction_for_length_1_vector,
            direction_for_length_1_vector,
        ),
        2 => Mathematical2DVector::new(
            direction_for_length_1_vector,
            -direction_for_length_1_vector,
        ),
        3 => Mathematical2DVector::new(
            -direction_for_length_1_vector,
            -direction_for_length_1_vector,
        ),
        4 => Mathematical2DVector::new(1.0, 1.0),
        5 => Mathematical2DVector::new(-1.0, 1.0),
        6 => Mathematical2DVector::new(1.0, -1.0),
        _ => Mathematical2DVector::new(-1.0, -1.0),
    }
}

fn ease_interpolation_value(interpolation_value: f32) -> f32 {
    // 3.0 * interpolation_value * interpolation_value - 2.0 * interpolation_value * interpolation_value * interpolation_value
    interpolation_value
        * interpolation_value
        * interpolation_value
        * (interpolation_value * (interpolation_value * 6.0 - 15.0) + 10.0)
}

fn compute_linear_interpolation(interpolation_value: f32, value_1: f32, value_2: f32) -> f32 {
    value_1 + interpolation_value * (value_2 - value_1)
}

fn calculate_value_at_coordinates(
    x: u32,
    y: u32,
    permutation_table: [usize; 512],
    chunk_size: u32,
) -> f32 {
    let x = x as f32 / chunk_size as f32;
    let y = y as f32 / chunk_size as f32;

    let x_floor = x.floor();
    let y_floor = y.floor();
    let x_decimal_part = x - x_floor;
    let y_decimal_part = y - y_floor;


    let x_left = x_floor as usize & 255;
    let y_top = y_floor as usize & 255;
    let x_right = (x_left + 1) & 255;
    let y_bottom = (y_top + 1) & 255;

    // Get corners values from permutation table
    let value_for_top_left_corner = permutation_table[permutation_table[x_left] + y_top];
    let value_for_top_right_corner = permutation_table[permutation_table[x_right] + y_top];
    let value_for_bottom_right_corner = permutation_table[permutation_table[x_right] + y_bottom];
    let value_for_bottom_left_corner = permutation_table[permutation_table[x_left] + y_bottom];


    // Get each corner's vectors
    let bottom_left_corner_vector = get_corner_vector(value_for_bottom_left_corner);
    let top_left_corner_vector = get_corner_vector(value_for_top_left_corner);
    let bottom_right_corner_vector = get_corner_vector(value_for_bottom_right_corner);
    let top_right_corner_vector = get_corner_vector(value_for_top_right_corner);

    // Get vectors from coordinates to each corner
    let vector_to_top_left = Mathematical2DVector::new(-x_decimal_part, -y_decimal_part);
    let vector_to_top_right = Mathematical2DVector::new(1.0 - x_decimal_part, -y_decimal_part);
    let vector_to_bottom_right = Mathematical2DVector::new(1.0 - x_decimal_part, 1.0 - y_decimal_part);
    let vector_to_bottom_left = Mathematical2DVector::new(-x_decimal_part, 1.0 - y_decimal_part);


    // Compute dot product of vectors
    let dot_product_top_left = vector_to_top_left.compute_dot_product(&top_left_corner_vector);
    let dot_product_top_right = vector_to_top_right.compute_dot_product(&top_right_corner_vector);
    let dot_product_bottom_right =
        vector_to_bottom_right.compute_dot_product(&bottom_right_corner_vector);
    let dot_product_bottom_left =
        vector_to_bottom_left.compute_dot_product(&bottom_left_corner_vector);
    
    let faded_x = ease_interpolation_value(x_decimal_part);
    let faded_y = ease_interpolation_value(y_decimal_part);


    let interpolated_left_dot_products =
        compute_linear_interpolation(faded_y, dot_product_top_left, dot_product_bottom_left);
    let interpolated_right_dot_products =
        compute_linear_interpolation(faded_y, dot_product_top_right, dot_product_bottom_right);

    let interpolated_left_right = compute_linear_interpolation(
        faded_x,
        interpolated_left_dot_products,
        interpolated_right_dot_products,
    );
    interpolated_left_right
}

fn main() {
    let opt = Opt::from_args();
    let number_of_iteration: u32 = {
        match opt.generate_image {
            true => {
                opt.number_of_images_to_generate
            },
            false => { 1 }
        }
    };

    for i in 0..number_of_iteration {
        let permutation_table = generate_permutation_table();

        if opt.generate_image {
            let mut image: RgbImage = ImageBuffer::new(opt.width, opt.height);

            for x in 0..opt.width {
                for y in 0..opt.height {
                    let generated_values_for_coordinates =
                        ((calculate_value_at_coordinates(x, y, permutation_table, opt.chunk_size) + 1.0) / 2.0)
                            * 255.0;
                    let generated_values_for_coordinates = generated_values_for_coordinates as u8;
                    *image.get_pixel_mut(x, y) = image::Rgb([
                        generated_values_for_coordinates,
                        generated_values_for_coordinates,
                        generated_values_for_coordinates,
                    ]);
                }
            }
            let image_name = format!("generated_image_{}.png", i);
            image.save(image_name).unwrap();
        } else {
            for y in 0..opt.height {
                for x in 0..opt.width {
                    let generated_values_for_coordinates = calculate_value_at_coordinates(x, y, permutation_table, opt.chunk_size);
                    print!("{} ", generated_values_for_coordinates);
                }
                print!("\n");
            }
        }
    }
}
