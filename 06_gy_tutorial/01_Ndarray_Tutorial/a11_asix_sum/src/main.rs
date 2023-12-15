use ndarray::{arr2, Axis};
fn main() {
    let a = arr2(&[[1., 2., 3.], [4., 5., 6.]]);
    dbg!(&a);
    println!("sum axis : {}", a.sum_axis(Axis(0)));
    println!("sum axis : {}", a.sum_axis(Axis(1)));
}
