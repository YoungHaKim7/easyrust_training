fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = match y {
        Some(y_val) => x + y_val,
        None => {
            // Handle the case where y is None
            0 // Or any other appropriate value
        }
    };
    dbg!(sum);
}
