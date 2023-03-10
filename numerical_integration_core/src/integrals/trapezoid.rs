use crate::functions::function;

pub struct IntTrapezoid {
    lower_bound: f64,
    upper_bound: f64,
    n: i32,
}

impl IntTrapezoid {
    pub fn new(lower_bound: f64, upper_bound: f64, n: i32) -> Self {
        IntTrapezoid {
            lower_bound,
            upper_bound,
            n,
        }
    }

    pub fn area(&self, func: &dyn function::Function) -> f64 {
        let dx = (self.upper_bound - self.lower_bound) / self.n as f64;
        let mut area = (func.f(self.lower_bound) + func.f(self.upper_bound)) * 0.5;

        let mut x = self.lower_bound;
        for _ in 1..self.n {
            x += dx;
            area += func.f(x);
        }

        area * dx
    }
}
