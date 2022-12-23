use anyhow::{Error, Context};

fn try_to_make_numbers(int: &str, float:&str) -> Result<(),Error> {
    let my_integer = int.parse::<i32>().with_context(|| "Extra info in here")?;
    let my_float = float.parse::<f64>().with_context(|| "Extra flota info in here")?;
    Ok(())
}
fn main() {
    let first_try = try_to_make_numbers("8","thtkdkal");
    let second_try = try_to_make_numbers("dkfickdhw","8.7");
    println!("{first_try:?}");
    println!("{second_try:?}");
}
