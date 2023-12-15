use std::f64::consts::PI;

use ndarray::{array, Array, Array1, ShapeBuilder};

fn main() {
    let a: Array1<f64> = array![0., 30., 45., 60., 90.];

    println!("angle: {:?}", &a);
    println!("sine(a): {}", (&a * PI / 180_f64).map(|x| x.sin()));

    let b01 = Array::from_shape_vec((3, 3), Array1::range(0., 9., 1.).to_vec());
    let b02 = Array::from_shape_vec((2, 2).strides((1, 2)), vec![1., 2., 3., 4.]).unwrap();
    println!("create array 01 bool : {:?}", b01.is_ok());
    println!("create array : {:?}", b02);

    let c = array![40., 100., 10.]; // No redundant type needed
    let d: Array1<f64> = array![20., 10., 100.];

    println!("a * 2: {}", (&a * 2.)); // Compatible types

    println!("c + d: {:?}", (&c + &d)); // Compatible types

    println!("c * d: {:?}", (&c * &d)); // Compatible types

    println!("average(a): {}", (a.sum() / a.len() as f64));

    println!("mean(c): {:?}", c.mean()); // Use Array::mean
}
