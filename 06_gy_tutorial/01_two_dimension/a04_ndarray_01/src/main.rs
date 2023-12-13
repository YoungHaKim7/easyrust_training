use ndarray::Array2;
fn main() {
    let mut array = Array2::zeros((4, 3));
    array[[1, 1]] = 7;
    println!("array : {array}");
}
