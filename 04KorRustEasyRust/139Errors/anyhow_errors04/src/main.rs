use anyhow::{anyhow, Error};

fn try_to_make_numbers(int: &str, float: &str) -> Result<(), Error> {
    let x = 9;
    if x == 9 {
        return Err(anyhow!("Uhoh, x shouldn't be 9"));
    }

    Ok(())
}

fn main() {
    let first_try = try_to_make_numbers("8", "thtkdkdfj");
    let second_try = try_to_make_numbers("dklfjsdlkfj", "8.7");
    println!("{first_try:?}");
    println!("{second_try:?}");
}
