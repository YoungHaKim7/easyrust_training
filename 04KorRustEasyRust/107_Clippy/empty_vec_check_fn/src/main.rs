fn print_vec_ref(input: &[i32]) {
    if input.is_empty() {
        println!("Vec is empty")
    } else {
        input.iter().for_each(|num| println!("{num}"));
    }
}

fn main() {
    let my_vec = vec![8, 9, 10];
    let my_array = [8, 9, 10];

    print_vec_ref(&my_vec);
    print_vec_ref(&my_array);
}
