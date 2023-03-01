use std::{io, str::FromStr};

fn read_number<T: FromStr>(description: &str) -> Result<T, io::Error> {
    println!("{description}");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim().parse::<T>() {
        Ok(num) => Ok(num),
        Err(_) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Could not parse input as number",
        )),
    }
}

struct IntTrapezoid {
    lower_bound: f64,
    upper_bound: f64,
    n: i32,
}

impl IntTrapezoid {
    fn area(&self, f2g: &F2G) -> f64 {
        let dx = (self.upper_bound - self.lower_bound) / self.n as f64;
        let mut area = (f2g.f(self.lower_bound) + f2g.f(self.upper_bound)) * 0.5;

        let mut x = self.lower_bound;
        for _ in 1..self.n {
            x += dx;
            area += f2g.f(x);
        }

        area * dx
    }
}

struct F2G {
    c0: f64,
    c1: f64,
    c2: f64,
}

impl F2G {
    fn f(&self, x: f64) -> f64 {
        self.c0 + self.c1 * x + self.c2 * x * x
    }
}

fn main() -> Result<(), io::Error> {
    println!("Numerical Integarion in Rust - v0.3");

    println!("Enter the integration parameters");

    let n = read_number::<i32>("Number of iterations:")?;
    let lower_bound = read_number::<f64>("Lower bound:")?;
    let upper_bound = read_number::<f64>("Upper bound:")?;

    println!("Enter the paramters of the quadratic function (f(x) = c0 + c1*x + c2*x²)");

    let c0 = read_number::<f64>("c0:")?;
    let c1 = read_number::<f64>("c1:")?;
    let c2 = read_number::<f64>("c2:")?;

    println!(
        "n = {}, lower = {}, upper = {}",
        n, lower_bound, upper_bound
    );
    println!("f(x) = {} + {}*x + {}*x²", c0, c1, c2);

    let f2g = F2G { c0, c1, c2 };
    let trapezoid = IntTrapezoid { lower_bound, upper_bound, n };

    let area = trapezoid.area(&f2g);

    println!("Area = {}", area);
    Ok(())
}
