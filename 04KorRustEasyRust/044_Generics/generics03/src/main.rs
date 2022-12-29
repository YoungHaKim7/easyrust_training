fn give_thing<GenericType>(input: GenericType) -> GenericType {
    input
}

fn main() {
    let x = give_thing(String::from("Take this things"));
    let y = give_thing(9);
    println!("{x}");
    println!("{y}");
}
