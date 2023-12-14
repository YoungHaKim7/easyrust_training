use std::f64::consts::PI;

use ndarray::{array, Array, Array1, ShapeError};

fn main() -> Result<(), ShapeError> {
    let a: Array1<f64> = array![0., 30., 45., 60., 90.];

    println!("angle {:?}", &a);
    println!("sine(a) {}", (&a * PI / 180_f64).map(|x| x.sin()));

    let b = Array::from_shape_vec((3, 3), Array1::range(0., 9., 1.).to_vec())?;

    let c = array![40., 100., 10.];
    let d = array![20., 10., 100.];

    println!("b: {:?}", &b);
    println!("a * 2 {}", (&a * 2.));
    println!("c + d {:?}", (&c + &d));
    println!("c * d {}", (&c * &d));
    println!("average(a) {}", (a.sum() / a.len() as f64));
    println!("mean(b) = {:?}", &b.mean());
    println!("mean(c) = {:?}", &c.mean());
    Ok(())
}
