use super::function::Function;

pub struct F2G {
    c0: f64,
    c1: f64,
    c2: f64,
}

impl F2G {
    pub fn new(c0: f64, c1: f64, c2: f64) -> Self {
        F2G { c0, c1, c2 }
    }
}

impl Function for F2G {
    fn f(&self, x: f64) -> f64 {
        self.c0 + self.c1 * x + self.c2 * x * x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_2g_polynomial() {
        let f2g = F2G::new(1.0, 2.0, 3.0);
        let expected_result_0 = 1.0;
        let expected_result_1 = 1.0 + 2.0 + 3.0;
        let expected_result_3 = 1.0 + 2.0 * 3.0 + 3.0 * 3.0 * 3.0;

        assert_eq!(f2g.f(0.0), expected_result_0);
        assert_eq!(f2g.f(1.0), expected_result_1);
        assert_eq!(f2g.f(3.0), expected_result_3);
    }
}
