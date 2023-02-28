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

fn apply_fx(x: f64, c0: f64, c1: f64, c2: f64) -> f64 {
    c0 + c1 * x + c2 * x * x
}

fn calculate_area(n: i32, lower_bound: f64, upper_bound: f64, c0: f64, c1: f64, c2: f64) -> f64 {
    let dx = (upper_bound - lower_bound) / n as f64;
    let mut area = (apply_fx(lower_bound, c0, c1, c2) + apply_fx(upper_bound, c0, c1, c2)) * 0.5;

    let mut x = lower_bound;
    for _ in 1..n {
        x += dx;
        area += apply_fx(x, c0, c1, c2);
    }

    area * dx
}

fn main() -> Result<(), io::Error> {
    println!("Numerical Integarion in Rust - v0.2");

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

    let area = calculate_area(n, lower_bound, upper_bound, c0, c1, c2);

    println!("Area = {}", area);
    Ok(())
}

