pub fn f(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;
    println!("this will happen");
    if before != after {
        println!("This will never happen");
    }
}

fn main() {
    let x = 3;
    let mut y = 6;
    println!("x print {}", x);
    f(&x, &mut y);
}
