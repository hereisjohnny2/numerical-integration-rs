pub struct F2G {
    c0: f64,
    c1: f64,
    c2: f64,
}

impl F2G {
    pub fn new(c0: f64, c1: f64, c2: f64) -> Self {
        F2G { c0, c1, c2 }
    }

    pub fn f(&self, x: f64) -> f64 {
        self.c0 + self.c1 * x + self.c2 * x * x
    }
}
