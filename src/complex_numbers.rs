pub struct ComplexNumber {
    pub real: f32,
    pub imaginary: f32,
}

pub fn get_magnitude(complex_number: &ComplexNumber) -> f32 {
    (complex_number.real * complex_number.real)
        + (complex_number.imaginary * complex_number.imaginary)
}

pub fn square_complex_number(number: &ComplexNumber) -> ComplexNumber {
    // (r1 * r1) + (r1 * i2) + (i1 + r2) + (i1 * i2)
    ComplexNumber {
        real: (number.real * number.real) - (number.imaginary * number.imaginary),
        imaginary: (number.real * number.imaginary) * 2.0,
    }
}

pub fn add_complex_numbers(number1: &ComplexNumber, number2: &ComplexNumber) -> ComplexNumber {
    ComplexNumber {
        real: number1.real + number2.real,
        imaginary: number1.imaginary + number2.imaginary,
    }
}

pub fn zero() -> ComplexNumber {
    ComplexNumber {
        real: 0.0,
        imaginary: 0.0,
    }
}
