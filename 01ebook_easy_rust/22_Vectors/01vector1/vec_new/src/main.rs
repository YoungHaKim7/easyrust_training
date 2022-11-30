fn main() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");
    let name3 = "GlobalYoung".to_string();

    let mut my_vec = Vec::new();

    let a = my_vec.push(name1);
    //    let b = my_vec.push(name2);
    //    let c = my_vec.push(name1);
    println!("{a:?}");
}
