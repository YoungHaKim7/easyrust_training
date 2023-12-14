use std::f64::consts::PI;

use ndarray::{array, Array, Array1, ArrayBase, Dim, OwnedRepr, ShapeError};

fn from_shape02<D>() -> Result<ArrayBase<OwnedRepr<f64>, D>, ShapeError> {
    let _ = Array::from_shape_vec((3, 3), Array1::range(0., 9., 1.).to_vec()).expect("failed");
    Ok((ArrayBase<OwnedRepr && OwnedRepr<f64>, D>))
}

// fn from_shape02<D>() -> Result<ArrayBase<OwnedRepr<f64>, D>, ShapeError>
// where
//     D: Dimension,
// {
//     let _ = Array::from_shape_vec((3, 3), Array1::range(0., 9., 1.).to_vec());
//     // Ok(ArrayBase::<OwnedRepr<f64>, D>)
//     Ok(ArrayBase<OwnedRepr && OwnedRepr<f64>, D>)
//     // comparison operators cannot be chained
// }

fn main() {
    let a: Array1<f64> = array![0., 30., 45., 60., 90.];

    println!("angle: {:?}", &a);
    println!("sine(a): {}", (&a * PI / 180_f64).map(|x| x.sin()));

    let c = array![40., 100., 10.]; // No redundant type needed
    let d: Array1<f64> = array![20., 10., 100.];

    println!(
        "b: {}",
        format!(
            "{:?} {:?}",
            &from_shape02::<Dim<[usize; 2]>>().unwrap(),
            "{:?}"
        )
    );

    println!("a * 2: {}", (&a * 2.)); // Compatible types

    println!("c + d: {:?}", (&c + &d)); // Compatible types

    println!("c * d: {:?}", (&c * &d)); // Compatible types

    println!("average(a): {}", (a.sum() / a.len() as f64));

    println!("mean(c): {:?}", c.mean()); // Use Array::mean
}
