// 1.70
use std::cell::OnceCell;

fn main() {
    let cell = OnceCell::new();

    let value: &str = cell.get_or_init(|| "Hello, World OnceCell std".to_string());
    println!("{value}");
    println!("{}", cell.get().is_some());
}
