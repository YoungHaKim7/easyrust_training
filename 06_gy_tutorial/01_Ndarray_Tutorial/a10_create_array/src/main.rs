use ndarray::{Array, Array1, ShapeBuilder};

fn main() {
    let b01 = Array::from_shape_vec((3, 3), Array1::range(0., 9., 1.).to_vec());
    let b02 = Array::from_shape_vec((2, 2).strides((1, 2)), vec![1., 2., 3., 4.]).unwrap();
    println!("create array : {:?}", b01);
    println!("create array : {:?}", b02);
}
