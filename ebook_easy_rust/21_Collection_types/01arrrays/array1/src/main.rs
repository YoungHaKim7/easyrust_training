fn main() {
    let array1 = ["One", "Two"]; // This one is type [&str;2]
    let array2 = ["One", "Two", "Five"]; // But this one is type [&str;3]. Different type!

    println!("{array1:?}");
    println!("{array2:?}");
}
