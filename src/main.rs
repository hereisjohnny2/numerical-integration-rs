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

fn main() -> Result<(), io::Error> {
    println!("Numerical Integarion in Rust - v0.1");

    println!("Enter the integration parameters");

    let n = read_number::<i32>("Number of iterations:")?;
    let lower_bound = read_number::<f64>("Lower bound:")?;
    let upper_bound = read_number::<f64>("Upper bound:")?;
    let dx = (upper_bound - lower_bound) / n as f64;

    println!("Enter the paramters of the quadratic function (f(x) = c0 + c1*x + c2*x²)");

    let c0 = read_number::<f64>("c0:")?;
    let c1 = read_number::<f64>("c1:")?;
    let c2 = read_number::<f64>("c2:")?;

    println!(
        "n = {}, lower = {}, upper = {}, dx = {}",
        n, lower_bound, upper_bound, dx
    );
    println!("f(x) = {} + {}*x + {}*x²", c0, c1, c2);

    let mut x = lower_bound;
    let mut area = (c0 + c1 * x + c2 * x * x) * 0.5;

    for _ in 1..n {
        x += dx;
        area += c0 + c1 * x + c2 * x * x;
    }

    x = upper_bound;
    area += (c0 + c1 * x + c2 * x * x) * 0.5;
    area *= dx;

    println!("Area = {}", area);
    Ok(())
}
