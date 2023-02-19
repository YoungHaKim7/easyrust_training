fn main() {
    let ten_chars = ('a'..).take(10).collect::<Vec<_>>();
    println!("{ten_chars:?}");
}
