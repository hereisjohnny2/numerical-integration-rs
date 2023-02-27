use std::io;

fn read_number(description: &str) -> Result<f64, io::Error> {
    println!("{description}");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim().parse::<f64>() {
        Ok(num) => Ok(num),
        Err(_) => match input.trim().parse::<i64>() {
            Ok(num) => Ok(num as f64),
            Err(_) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Could not parse input as number",
            )),
        },
    }
}

fn main() -> Result<(), io::Error> {
    let x = read_number("Enter value x:")?;
    let y = read_number("Enter value y:")?;

    let sum = x + y;
    println!("The sum of {} and {} is {}.", x, y, sum);

    Ok(())
}

