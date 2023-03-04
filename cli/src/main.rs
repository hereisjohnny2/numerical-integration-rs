use std::io;

use numerical_integration_core::{utils, functions::f2g, integrals::trapezoid};

fn main() -> Result<(), io::Error> {
    println!("Enter the integration parameters");
    let n = utils::read_number::<i32>("Number of iterations:")?;
    let lower_bound = utils::read_number::<f64>("Lower bound:")?;
    let upper_bound = utils::read_number::<f64>("Upper bound:")?;

    println!("Enter the paramters of the quadratic function (f(x) = c0 + c1*x + c2*x²)");
    let c0 = utils::read_number::<f64>("c0:")?;
    let c1 = utils::read_number::<f64>("c1:")?;
    let c2 = utils::read_number::<f64>("c2:")?;

    println!(
        "n = {}, lower = {}, upper = {}",
        n, lower_bound, upper_bound
    );
    println!("f(x) = {} + {}*x + {}*x²", c0, c1, c2);

    let f2g = f2g::F2G::new(c0, c1, c2);
    let trapezoid = trapezoid::IntTrapezoid::new(lower_bound, upper_bound, n);
    let area = trapezoid.area(&f2g);

    println!("Area = {}", area);
    Ok(())
}
