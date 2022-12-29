use std::fmt::{Debug, Display};

#[derive(Debug)]
struct Book;

fn give_thing<T: Display + Debug>(input: T) -> T {
    println!("{input:?}"); // Display
    input
}

fn main() {
    let x = give_thing(String::from("Take this things"));
    let y = give_thing(9);
    let z = give_thing(Book);
    println!("{x}");
    println!("{y}");
}
