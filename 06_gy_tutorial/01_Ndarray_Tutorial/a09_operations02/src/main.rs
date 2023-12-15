use ndarray::{array, ArrayView1};

fn main() {
    let owned1 = array![1, 2];
    let owned2 = array![3, 4];
    let view1 = ArrayView1::from(&[5, 6]);
    let view2 = ArrayView1::from(&[7, 8]);
    let mut mutable = array![9, 10];

    let sum1 = &view1 + &view2; // Allocates a new array. Note the explicit `&`.
                                // let sum2 = view1 + &view2; // This doesn't work because `view1` is not an owned array.
    let sum3 = owned1 + view1; // Consumes `owned1`, updates it, and returns it.
    let sum4 = owned2 + &view2; // Consumes `owned2`, updates it, and returns it.
    mutable += &view2;
    // dbg!(&owned1);
    // dbg!(&owned2);
    // dbg!(&view1);
    // dbg!(&view2);
    // dbg!(&mutable);
    dbg!(sum1);
    dbg!(sum3);
    dbg!(sum4);
}
