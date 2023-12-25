fn use_string(s: String) {
    println!("{}", s);
}

fn borrow_string(s: &String) {
    println!("{}", &s);
}

fn main() {
    let msg = "Hello world".to_string();
    use_string(msg);

    // println!("{}", msg); // borrow of moved value: `msg`

    let new_msg = "Hello world!".to_string();
    borrow_string(&new_msg);
    println!("{}", new_msg);
}
