fn main() {
    let x_int = vec![1, 2, 34, 5, 6, 8];
    // let mut iter_x = x_int.iter();
    for (i, element) in x_int.iter().enumerate() {
        println!("index[{}] :  {}", i, element);
    }
}
