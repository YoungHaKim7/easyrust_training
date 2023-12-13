use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("The secret number is :{secret_number}");

        println!("Please input your guess.");

        let mut guess_str = String::new();
        io::stdin()
            .read_line(&mut guess_str)
            .expect("Failed to read line");
        let guess: u32 = guess_str.trim().parse().unwrap();
        // .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big"),
            std::cmp::Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
