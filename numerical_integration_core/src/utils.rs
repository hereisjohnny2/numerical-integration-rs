use std::{io, str::FromStr};
pub fn read_number<T: FromStr>(description: &str) -> Result<T, io::Error> {
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
