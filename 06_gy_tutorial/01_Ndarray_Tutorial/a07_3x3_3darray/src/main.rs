use ndarray::Array3;

fn main() {
    let mut array = Array3::zeros((3, 3, 3));
    array[[0, 0, 0]] = 111;
    println!("3D array {array:?}");
    println!();
    array[[1, 0, 0]] = 211;
    println!("3D array {array:?}");
    println!();
    array[[2, 1, 0]] = 321;
    println!("3D array {array:?}");
    println!();
    array[[2, 2, 0]] = 421;
    println!("3D array {array:?}");
    println!();
    array[[2, 2, 2]] = 444;
    println!("3D array {array:?}");
    println!();
}
