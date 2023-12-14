use ndarray::Array3;

fn main() {
    let mut array = Array3::zeros((3, 3, 3));
    array[[1, 1, 1]] = 7;
    println!("array : {array}");
}
