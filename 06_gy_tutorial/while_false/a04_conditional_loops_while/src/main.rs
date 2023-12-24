use std::time::Duration;

fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
        std::thread::sleep(Duration::from_secs(1));
    }

    println!("LIFT OFF!");
}
