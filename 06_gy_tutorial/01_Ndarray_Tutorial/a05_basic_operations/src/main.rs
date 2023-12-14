use ndarray::{array, Array};
use std::f64::INFINITY as inf;

fn main() {
    let a = array![[10., 20., 30., 40.,],];
    let b = Array::range(0., 4., 1.);

    assert_eq!(&a + &b, array![[10.0, 21.0, 32.0, 43.0]]);
    assert_eq!(&a - &b, array![[10.0, 19.0, 28.0, 37.0]]);
    assert_eq!(&a * &b, array![[0.0, 20.0, 60.0, 120.0]]);
    assert_eq!(&a / &b, array![[inf, 20.0, 15.0, 13.333333333333334]]);

    println!("a {a:?}");
    println!("b {b:?}");
    println!("matrix a + b ={:?}", (&a + &b));
    println!("matrix a - b ={:?}", (&a - &b));
    println!("matrix a * b ={:?}", (&a * &b));
    println!("matrix a / b ={:?}", (&a / &b));
}
