use super::function::Function;

pub struct Polynomial {
    coefficients: Vec<f64>,
    degree: usize,
}

impl Polynomial {
    pub fn new(coefficients: Vec<f64>) -> Self {
        let degree = coefficients.len();

        if degree < 1 {
            panic!("it's not possible to create a polynomial degree equals to 0");
        }

        Polynomial {
            coefficients,
            degree,
        }
    }

    pub fn degree(&self) -> usize {
        self.degree
    }
}

impl ToString for Polynomial {
    fn to_string(&self) -> String {
        let mut expression = String::from(format!("f(x) = {}", self.coefficients.get(0).unwrap()));
        for i in 1..self.degree {
            let coeff = self.coefficients.get(i).unwrap();

            if coeff < &0.0 {
                expression.push_str(format!(" - {}", coeff).as_str());
            } else {
                expression.push_str(format!(" + {}", coeff).as_str());
            }

            if i == 1 {
                expression.push_str(format!(" * x").as_str());
            } else if i > 1 {
                expression.push_str(format!(" * x^{}", i).as_str());
            }
        }

        expression
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

    #[test]
    #[should_panic]
    fn should_panic_if_coeffs_len_is_zero() {
        Polynomial::new(vec![]);
    }

    #[test]
    fn should_get_to_stringession() {
        let coefficients_0 = vec![1.0];
        let poly_0 = Polynomial::new(coefficients_0);
        let expected_0 = "f(x) = 1";
        let result_0 = poly_0.to_string();

        assert_eq!(expected_0, result_0);

        let coefficients_1 = vec![1.0, 2.0];
        let poly_1 = Polynomial::new(coefficients_1);
        let expected_1 = "f(x) = 1 + 2 * x";
        let result_1 = poly_1.to_string();

        assert_eq!(expected_1, result_1);

        let coefficients_4 = vec![1.0, 2.0, 3.0, 4.0];
        let poly_4 = Polynomial::new(coefficients_4);
        let expected_4 = "f(x) = 1 + 2 * x + 3 * x^2 + 4 * x^3";
        let result_4 = poly_4.to_string();

        assert_eq!(expected_4, result_4);
    }
}
