fn main() {
    let mut num_vec = Vec::new();
    println!("{}", num_vec.capacity()); // 0 elements: prints 0
    num_vec.push('a');
    println!("{}", num_vec.capacity()); // 0 elements: prints 0
    num_vec.push('a');
    num_vec.push('a');
    num_vec.push('a');
    println!("{}", num_vec.capacity()); // 0 elements: prints 0
    num_vec.push('a');
    println!("{}", num_vec.capacity()); // 0 elements: prints 0
}
