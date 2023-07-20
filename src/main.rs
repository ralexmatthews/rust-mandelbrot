#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};

mod complex_numbers;

// Returns the result of the Mandelbrot iteration
// the next answer comes from squaring the previous answer and adding the base
// operand^2 + base
fn iterate(
    base: complex_numbers::ComplexNumber,
    operand: complex_numbers::ComplexNumber,
) -> complex_numbers::ComplexNumber {
    complex_numbers::add_complex_numbers(complex_numbers::square_complex_number(operand), base)
}

// Returns the number of iterations it took to either exceed two or repeat a number
//
// base is the complex number being tested
// operand is the result of the current iteration and starts at 0
// iteration is the number of iterations that have been performed so far
// set_of_used_numbers is a list of numbers that have already been tested
//
// If the number converges, None is returned
// If the number diverges, Some(iteration) is returned
fn find_converges(
    base: complex_numbers::ComplexNumber,
    operand: complex_numbers::ComplexNumber,
    iteration: u16,
) -> Option<u16> {
    if iteration >= 1024 {
        return None;
    }

    let result = iterate(base, operand);

    let magnitude = complex_numbers::get_magnitude(result);
    if magnitude >= 2.0 {
        return Some(iteration);
    }

    find_converges(base, result, iteration + 1)
}

// maps a number between 0 and 1024 to a pixel color
fn map_iterations_to_pixel(value: u16) -> Pixel {
    px!(255, 255 - (value / 4), 255 - (value / 4))
}

fn main() {
    let out_of_bounds: f32 = 2.0;
    let resolution: u16 = 16000;

    let mut image = Image::new(u32::from(resolution), u32::from(resolution));

    let half_of_resolution = i16::try_from(resolution).unwrap() / 2;

    let float_half_of_resolution = f32::from(half_of_resolution);

    for y in -half_of_resolution..half_of_resolution {
        for x in -half_of_resolution..half_of_resolution {
            let complex_number = complex_numbers::ComplexNumber {
                real: (f32::from(x) / float_half_of_resolution) * out_of_bounds,
                imaginary: (f32::from(y) / float_half_of_resolution) * out_of_bounds,
            };

            let result = find_converges(complex_number, complex_numbers::zero(), 0);

            let ux = u32::try_from(x + half_of_resolution).unwrap();
            let uy = u32::try_from(y + half_of_resolution).unwrap();

            match result {
                Some(x) => image.set_pixel(ux, uy, map_iterations_to_pixel(x)),
                None => image.set_pixel(ux, uy, px!(0, 0, 0)),
            }
        }
    }

    let u_half = u32::from(resolution / 2);
    for x in 0..u32::from(resolution) {
        image.set_pixel(u_half, x, px!(0, 0, 0));
        image.set_pixel(u_half + 1, x, px!(0, 0, 0));
        image.set_pixel(u_half - 1, x, px!(0, 0, 0));
        image.set_pixel(u_half + 2, x, px!(0, 0, 0));
        image.set_pixel(u_half - 2, x, px!(0, 0, 0));

        image.set_pixel(x, u_half, px!(0, 0, 0));
        image.set_pixel(x, u_half + 1, px!(0, 0, 0));
        image.set_pixel(x, u_half - 1, px!(0, 0, 0));
        image.set_pixel(x, u_half + 2, px!(0, 0, 0));
        image.set_pixel(x, u_half - 2, px!(0, 0, 0));
    }

    let _ = image.save("fractal.bmp");
}
