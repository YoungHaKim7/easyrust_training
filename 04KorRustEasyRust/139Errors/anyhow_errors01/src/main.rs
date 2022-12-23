use anyhow;
use std::error::Error;

fn try_to_make_numbers(int: &str, float: &str) -> Result<(), Box<dyn Error>> {
    let my_integer = int.parse::<i32>()?;
    let my_float = int.parse::<f64>()?;
    Ok(())
}

fn main() {
    let first_try = try_to_make_numbers("8", "thtkdkdfj");
    let second_try = try_to_make_numbers("dklfjsdlkfj", "8.7");
}
