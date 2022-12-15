use std::num::ParseIntError;

fn try_to_make_number(int_input: &str, float_input: &str) -> Result<i32, ParseIntError> {
    let my_number = int_input.parse::<i32>()?;
    let other_number = float_input.parse::<f32>()?;
    Ok(())
}

fn main() {}
