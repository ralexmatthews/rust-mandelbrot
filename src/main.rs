#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};
use std::thread;

mod complex_numbers;
mod mandelbrot;

// maps a number between 0 and 1024 to a pixel color
fn map_iterations_to_pixel(value: u16) -> Pixel {
    let u32_value = u32::from(value);
    let color_points: [(u32, (u32, u32, u32)); 6] = [
        (1024, (0, 0, 0)),    // black
        (256, (0, 0, 64)),    // blue
        (192, (102, 0, 0)),   // maroon
        (128, (255, 0, 0)),   // red
        (64, (255, 165, 0)),  // orange
        (0, (255, 255, 255)), // white
    ];

    let index = color_points
        .iter()
        .position(|x| x.0 <= u32_value)
        .unwrap_or(5);

    if index == 0 {
        return px!(0, 0, 0);
    }

    let (r1, g1, b1) = color_points[index].1;
    let (r2, g2, b2) = color_points[index - 1].1;

    let diff_from_prev = color_points[index - 1].0 - u32_value;
    let diff_from_next = u32_value - color_points[index].0;
    let total_diff = diff_from_prev + diff_from_next;

    let get_average = |x1, x2| (((diff_from_prev) * x1) + ((diff_from_next) * x2)) / total_diff;

    let r_average = get_average(r1, r2);
    let g_average = get_average(g1, g2);
    let b_average = get_average(b1, b2);

    px!(r_average, g_average, b_average)
}

fn do_work(begin: i32, end: i32, half_of_resolution: i32, out_of_bounds: f64) -> Vec<Vec<Pixel>> {
    let float_half_of_resolution = f64::from(half_of_resolution);
    let mut pixels = vec![];
    for y in begin..end {
        let mut row = vec![];
        for x in -half_of_resolution..half_of_resolution {
            let complex_number = complex_numbers::ComplexNumber {
                real: (f64::from(x) / float_half_of_resolution) * out_of_bounds,
                imaginary: (f64::from(y) / float_half_of_resolution) * out_of_bounds,
            };

            let result = mandelbrot::find_converges(complex_number, complex_numbers::zero(), 0);

            let pixel = match result {
                Some(x) => map_iterations_to_pixel(x),
                None => px!(0, 0, 0),
            };

            row.push(pixel);
        }
        pixels.push(row);
    }
    pixels
}

fn main() {
    let out_of_bounds: f64 = 2.0;
    let resolution: u16 = 24000;

    let mut image = Image::new(u32::from(resolution), u32::from(resolution));

    let half_of_resolution = i32::from(resolution) / 2;

    let threads = [
        thread::spawn(move || {
            do_work(
                -half_of_resolution,
                -(half_of_resolution / 2),
                half_of_resolution,
                out_of_bounds,
            )
        }),
        thread::spawn(move || {
            do_work(
                -(half_of_resolution / 2),
                0,
                half_of_resolution,
                out_of_bounds,
            )
        }),
        thread::spawn(move || {
            do_work(0, half_of_resolution / 2, half_of_resolution, out_of_bounds)
        }),
        thread::spawn(move || {
            do_work(
                half_of_resolution / 2,
                half_of_resolution,
                half_of_resolution,
                out_of_bounds,
            )
        }),
    ];

    let thread_results = threads.map(|t| t.join().unwrap());

    let all_pixels = thread_results
        .iter()
        .flat_map(|x| x.iter())
        .cloned()
        .collect::<Vec<_>>();

    for y in 0..(resolution - 1) {
        for x in 0..(resolution - 1) {
            let pixel = all_pixels[y as usize][x as usize];
            image.set_pixel(u32::try_from(x).unwrap(), u32::try_from(y).unwrap(), pixel);
        }
    }

    let u_half = u32::from(resolution / 2);
    for x in 0..u32::from(resolution) {
        image.set_pixel(u_half, x, px!(0, 0, 0));

        image.set_pixel(x, u_half, px!(0, 0, 0));
    }

    let _ = image.save("fractal.bmp");
}
