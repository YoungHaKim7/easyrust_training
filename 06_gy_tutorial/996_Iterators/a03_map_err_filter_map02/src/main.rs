use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let strings = vec!["tofu", "93", "18"];
    let strings02 = vec!["93", "18"];
    let numbers: Result<Vec<_>, _> = strings02.into_iter().map(|s| s.parse::<i32>()).collect();
    match numbers {
        Ok(data1) => {
            println!("ok: data1{:?}", data1)
        }
        Err(e) => eprint!("Error parsing number: {}\n", e),
    };

    // println!("Result: {:?}", &numbers);
    Ok(())
}
