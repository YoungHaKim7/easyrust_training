#[derive(Debug)]
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = Option::None;

    dbg!(some_number);
    dbg!(some_char);
    dbg!(absent_number);
}
