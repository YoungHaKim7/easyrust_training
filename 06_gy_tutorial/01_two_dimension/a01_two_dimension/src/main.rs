fn main() {
    let mut state = [[0u8; 4]; 6];
    state[0][1] = 42;
    println!("two dimention : state : {state:?}");

    let x: [Box<[u8]>; 3] = [Box::new([1, 2, 3]), Box::new([4]), Box::new([5, 6])];
    let y: &[Box<[u8]>] = &x;
    println!("x : {x:?}");
    println!("y : {y:?}");
}
