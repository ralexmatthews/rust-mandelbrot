extern crate bmp;
use crate::bmp::{px, Image, Pixel};
use std::thread;

mod complex_numbers;
mod mandelbrot;

// map of iterations -> colors to use when making the plane.
// more iterations mean closer to being in the set
const COLOR_POINTS: [(u32, (u32, u32, u32)); 6] = [
    (1024, (0, 0, 0)),    // black
    (256, (0, 0, 64)),    // blue
    (192, (102, 0, 0)),   // maroon
    (128, (255, 0, 0)),   // red
    (64, (255, 165, 0)),  // orange
    (0, (255, 255, 255)), // white
];
// this color scheme will give an alternative blue-ish background
// const COLOR_POINTS: [(u32, (u32, u32, u32)); 6] = [
//     (1024, (0, 0, 0)),     // black
//     (256, (255, 165, 0)),  // orange
//     (192, (255, 255, 0)),  // yellow
//     (128, (102, 0, 0)),    // maroon
//     (64, (255, 255, 255)), // white
//     (0, (0, 0, 64)),       // blue
// ];

// the edge of the coordinate system. For the Mandelbrot Set, it is proven that
// all complex numbers with a magnitude greater than 2 will diverge, so we'll
// ignore anything outside that boundary
const OUT_OF_BOUNDS: f64 = 2.0;

// the resolution of the image, in pixels. The image will be RESOLUTION x RESOLUTION
const RESOLUTION: u32 = 16000;
// it is also handy to have half the resolution as a constant, but it can't be based on RESOLUTION
const HALF_OF_RESOLUTION: i32 = 8000;

// maps a number between 0 and 1024 to a pixel color
fn map_iterations_to_pixel(value: u16) -> Pixel {
    let u32_value = u32::from(value);

    // the point at which the next color requires fewer iterations than the current iterations
    let index = COLOR_POINTS
        .iter()
        .position(|x| x.0 <= u32_value)
        .unwrap_or(5);

    // this shouldn't happen, but we should check anyway
    if index == 0 {
        return px!(0, 0, 0);
    }

    // get the two boundary colors that are being interpolated
    let (r1, g1, b1) = COLOR_POINTS[index].1;
    let (r2, g2, b2) = COLOR_POINTS[index - 1].1;

    // get the total range between colors and how far it is in either direction of that range
    let diff_from_prev = u32_value.abs_diff(COLOR_POINTS[index - 1].0);
    let diff_from_next = u32_value.abs_diff(COLOR_POINTS[index].0);
    let total_diff = diff_from_prev + diff_from_next;

    // get the average of the two colors based on how far it is in either direction of the range
    let get_average = |x1, x2| (((diff_from_prev) * x1) + ((diff_from_next) * x2)) / total_diff;

    let r_average = get_average(r1, r2);
    let g_average = get_average(g1, g2);
    let b_average = get_average(b1, b2);

    px!(r_average, g_average, b_average)
}

// map over the rows of the image and calculate the color of each pixel
fn do_work(begin: i32, end: i32) -> Vec<Vec<Pixel>> {
    let float_half_of_resolution = f64::from(HALF_OF_RESOLUTION);
    // the pixels that will be returned
    let mut pixels = vec![];
    for y in begin..end {
        // the pixels in the current row
        let mut row = vec![];
        // since the coordinate system we are testing is mirrored over the x and y axis,
        // we need to have the left half be negative, so split the resolution in two
        // and make the left half negative
        for x in -HALF_OF_RESOLUTION..HALF_OF_RESOLUTION {
            // map our x and y values to the coordinate system we are testing
            let complex_number: complex_numbers::ComplexNumber = (
                (f64::from(x) / float_half_of_resolution) * OUT_OF_BOUNDS,
                (f64::from(y) / float_half_of_resolution) * OUT_OF_BOUNDS,
            );

            // find the number of iterations it takes to diverge
            let result = mandelbrot::find_converges(complex_number, (0.0, 0.0), 0);

            let pixel = match result {
                // some means that it diverges and is not in the set
                Some(x) => map_iterations_to_pixel(x),
                // none means that it converges and is in the set
                None => px!(0, 0, 0),
            };

            // add our pixel to the row
            row.push(pixel);
        }
        // add the completed row to the pixels vector
        pixels.push(row);
    }
    pixels
}

fn main() -> std::io::Result<()> {
    // bitmap image from the library "bmp"
    let mut image = Image::new(u32::from(RESOLUTION), u32::from(RESOLUTION));

    // split the work into four threads
    // in my experience, threads greater than 2 have diminishing returns, so I'm only using 4
    let threads = [
        // top quarter
        thread::spawn(move || do_work(-HALF_OF_RESOLUTION, -(HALF_OF_RESOLUTION / 2))),
        // top-middle quarter
        thread::spawn(move || do_work(-(HALF_OF_RESOLUTION / 2), 0)),
        // bottom-middle quarter
        thread::spawn(move || do_work(0, HALF_OF_RESOLUTION / 2)),
        // bottom quarter
        thread::spawn(move || do_work(HALF_OF_RESOLUTION / 2, HALF_OF_RESOLUTION)),
    ];

    // wait for all threads to finish
    let thread_results = threads.map(|t| t.join().unwrap());

    // flatten the results into a single vector with rows of pixels
    let all_pixels = thread_results
        .iter()
        .flat_map(|x| x.iter())
        .cloned()
        .collect::<Vec<_>>();

    // set all the pixels to the image
    for y in 0..RESOLUTION {
        for x in 0..RESOLUTION {
            let pixel = all_pixels[y as usize][x as usize];
            image.set_pixel(x, y, pixel);
        }
    }

    // draw the x and y axis
    let u_half = HALF_OF_RESOLUTION as u32;
    for x in 0..u32::from(RESOLUTION) {
        image.set_pixel(u_half, x, px!(0, 0, 0));
        image.set_pixel(x, u_half, px!(0, 0, 0));
    }

    image.save("fractal.bmp")
}
