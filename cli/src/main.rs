use std::io;

use numerical_integration_core::{functions::polynomial::Polynomial, integrals::trapezoid, utils};

fn main() -> Result<(), io::Error> {
    println!("--- Integration Parameters ---");
    let n = utils::read_number::<i32>("Number of iterations:")?;
    let lower_bound = utils::read_number::<f64>("Lower bound:")?;
    let upper_bound = utils::read_number::<f64>("Upper bound:")?;

    println!("\n--- Function Coeficients ---");
    let degree = utils::read_number::<i32>("Enter polynomial function degree: ")?;
    let mut coefficients: Vec<f64> = Vec::new();

    for i in 0..(degree + 1) {
        let coeff = utils::read_number::<f64>(&format!("c{}:", i).to_string())?;
        coefficients.push(coeff);
    }

    println!(
        "n = {}, lower = {}, upper = {}",
        n, lower_bound, upper_bound
    );

    let poly_2 = Polynomial::new(coefficients);
    println!("{}", poly_2.to_string());

    let trapezoid = trapezoid::IntTrapezoid::new(lower_bound, upper_bound, n);
    let area = trapezoid.area(&poly_2);

    println!("Area = {}", area);

    Ok(())
}
