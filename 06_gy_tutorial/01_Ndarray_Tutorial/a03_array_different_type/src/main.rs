use ndarray::{Array, Ix3};

fn main() {
    let a = Array::<bool, Ix3>::from_elem((3, 2, 4), false);
    println!("{a:?}");
}
