use crate::complex_numbers;

// Returns the result of the Mandelbrot iteration
// the next answer comes from squaring the previous answer and adding the base
// operand^2 + base
fn iterate(
    base: complex_numbers::ComplexNumber,
    operand: complex_numbers::ComplexNumber,
) -> complex_numbers::ComplexNumber {
    let operand_squared = complex_numbers::square_complex_number(operand);
    complex_numbers::add_complex_numbers(operand_squared, base)
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
pub fn find_converges(
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
