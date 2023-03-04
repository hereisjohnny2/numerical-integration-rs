use super::function::Function;

pub struct Polynomial {
    coefficients: Vec<f64>,
    degree: usize,
}

impl Polynomial {
    pub fn new(coefficients: Vec<f64>) -> Self {
        let degree = coefficients.len();
        Polynomial {
            coefficients,
            degree,
        }
    }

    pub fn degree(&self) -> usize {
        self.degree
    }
}

impl Function for Polynomial {
    fn f(&self, x: f64) -> f64 {
        let mut output = 0.0;

        for i in 0..self.degree {
            output += self.coefficients.get(i).unwrap() * x.powi(i as i32);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_polynomial_from_vector() {
        let coefficients = vec![1.0, 2.0, 3.0, 4.0];
        let poly = Polynomial::new(coefficients);

        assert_eq!(4, poly.degree());
    }

    #[test]
    fn should_apply_function_for_a_given_number() {
        let coefficients = vec![1.0, 2.0, 3.0, 4.0];
        let poly = Polynomial::new(coefficients);
        let expected = 1.0 + 2.0 * 10.0 + 3.0 * 10.0_f64.powi(2) + 4.0 * 10.0_f64.powi(3);
        let result = poly.f(10.0);

        assert_eq!(expected, result);
    }
}
