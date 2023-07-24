#[derive(Debug, Copy, Clone)]
pub struct ComplexNumber {
    pub real: f64,
    pub imaginary: f64,
}

// checks if two complex numbers are equal, within a certain epsilon
const EPSILON: f64 = 0.0000000000000001;
pub fn are_equal(number1: ComplexNumber, number2: ComplexNumber) -> bool {
    (number1.real - number2.real).abs() < EPSILON
        && (number1.imaginary - number2.imaginary).abs() < EPSILON
}

// gets the square of the magnitude. This is faster than getting the actual magnitude
// because it doesn't need to take the square root
pub fn get_magnitude_squared(complex_number: ComplexNumber) -> f64 {
    (complex_number.real * complex_number.real)
        + (complex_number.imaginary * complex_number.imaginary)
}

pub fn square_complex_number(number: ComplexNumber) -> ComplexNumber {
    // (r1 * r1) + (r1 * i2) + (i1 + r2) + (i1 * i2)
    ComplexNumber {
        real: (number.real * number.real) - (number.imaginary * number.imaginary),
        imaginary: (number.real * number.imaginary) * 2.0,
    }
}

pub fn add_complex_numbers(number1: ComplexNumber, number2: ComplexNumber) -> ComplexNumber {
    ComplexNumber {
        real: number1.real + number2.real,
        imaginary: number1.imaginary + number2.imaginary,
    }
}

// returns a complex number with a real and imaginary part of 0. This is more semantic
// than creating a new complex number with 0.0 for both parts
pub fn zero() -> ComplexNumber {
    ComplexNumber {
        real: 0.0,
        imaginary: 0.0,
    }
}
