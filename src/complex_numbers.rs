pub type ComplexNumber = (f64, f64);

// checks if two complex numbers are equal, within a certain epsilon
const EPSILON: f64 = 0.0000000000000001;
pub fn are_equal(number1: ComplexNumber, number2: ComplexNumber) -> bool {
    (number1.0 - number2.0).abs() < EPSILON && (number1.1 - number2.1).abs() < EPSILON
}

// gets the square of the magnitude. This is faster than getting the actual magnitude
// because it doesn't need to take the square root
pub fn get_magnitude_squared(complex_number: ComplexNumber) -> f64 {
    (complex_number.0 * complex_number.0) + (complex_number.1 * complex_number.1)
}

pub fn square_complex_number(number: ComplexNumber) -> ComplexNumber {
    // (r1 * r1) + (r1 * i2) + (i1 + r2) + (i1 * i2)
    (
        (number.0 * number.0) - (number.1 * number.1),
        (number.0 * number.1) * 2.0,
    )
}

pub fn add_complex_numbers(number1: ComplexNumber, number2: ComplexNumber) -> ComplexNumber {
    (number1.0 + number2.0, number1.1 + number2.1)
}
