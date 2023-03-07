use super::function::Function;

pub struct Exponential {
    a: f64,
    b: f64,
}

impl Exponential {
    pub fn new(a: f64, b: f64) -> Self {
        Exponential { a, b }
    }
}

impl ToString for Exponential {
    fn to_string(&self) -> String {
        let mut output = String::from("f(x) =");

        if self.a != 1.0 {
            output.push_str(&format!(" {} *", self.a).to_string())
        }

        output.push_str(&format!(" e^"));

        if self.b != 1.0 {
            output.push_str(&format!("{}*", self.b).to_string())
        }

        output.push_str("x");

        output
    }
}

impl Function for Exponential {
    fn f(&self, x: f64) -> f64 {
        self.a * (self.b * x).exp()
    }
}

mod tests {
    use super::*;

    #[test]
    fn should_create_an_exponential_function() {
        let exp_ab = Exponential::new(10.0, 2.5);
        let expected = 10.0 * ((2.5 * 2.0) as f64).exp();

        assert_eq!(expected, exp_ab.f(2.0));
    }

    #[test]
    fn should_format_function_to_string() {
        let exp_ab = Exponential::new(10.0, 2.5);
        let expected_ab = "f(x) = 10 * e^2.5*x";

        assert_eq!(expected_ab, exp_ab.to_string());

        let exp_a = Exponential::new(1.0, 2.5);
        let expected_a = "f(x) = e^2.5*x";

        assert_eq!(expected_a, exp_a.to_string());

        let exp_b = Exponential::new(10.0, 1.0);
        let expected_b = "f(x) = 10 * e^x";

        assert_eq!(expected_b, exp_b.to_string());
    }
}
