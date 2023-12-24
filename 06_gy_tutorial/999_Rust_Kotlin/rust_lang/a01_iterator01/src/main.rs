fn main() {
    let mut my_vec = vec!["name 1", "name 2", "name3"];

    my_vec.push("name 4");
    println!("Before my vec = {my_vec:?}");
    my_vec.pop();
    println!("After my vec = {:?}", &my_vec);
}
