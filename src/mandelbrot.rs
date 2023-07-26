use crate::complex_numbers;

// Returns the result of the Mandelbrot iteration
// the next answer comes from squaring the previous answer and adding the base
// current_iterant^2 + number_being_tested
fn iterate(
    number_being_tested: complex_numbers::ComplexNumber,
    current_iterant: complex_numbers::ComplexNumber,
) -> complex_numbers::ComplexNumber {
    let operand_squared = complex_numbers::square_complex_number(current_iterant);
    complex_numbers::add_complex_numbers(operand_squared, number_being_tested)
}

// Returns the number of iterations it took to either exceed two or repeat a number
//
// number_being_tested is the complex number being tested
// current_iterant is the result of the current iteration, and starts at 0
// iteration is the number of iterations that have been performed so far
//
// If the number converges, None is returned
// If the number diverges, Some(iteration) is returned
pub fn find_converges(
    number_being_tested: complex_numbers::ComplexNumber,
    current_iterant: complex_numbers::ComplexNumber,
    iteration: u16,
) -> Option<u16> {
    if iteration >= 1024 {
        return None;
    }

    let result = iterate(number_being_tested, current_iterant);

    // if the next term is the same as the previous, we have converged
    if complex_numbers::are_equal(current_iterant, result) {
        return None;
    }

    // if the magnitude of the result is greater than 2, we have diverged
    let magnitude_squared = complex_numbers::get_magnitude_squared(result);
    if magnitude_squared >= 4.0 {
        return Some(iteration);
    }

    // inconclusive, so keep going
    find_converges(number_being_tested, result, iteration + 1)
}
