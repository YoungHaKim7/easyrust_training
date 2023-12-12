use std::io;

fn main() -> Result<(), io::Error> {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    match io::stdin().read_line(&mut guess) {
        Ok(it) => it,
        Err(err) => return Err(err),
    };
    println!("You guessed: {guess}");
    Ok(())
}
