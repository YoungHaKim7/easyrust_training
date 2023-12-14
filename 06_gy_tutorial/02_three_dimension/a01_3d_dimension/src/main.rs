fn main() {
    let mut array: [[[i32; 3]; 3]; 3] = Default::default();

    for x in 0..3 {
        for y in 0..3 {
            for z in 0..3 {
                array[x][y][z] = 0;
            }
        }
    }

    println!("array : {array:?}");
    array[0][0][1] = 12;
    println!("array : {array:?}");
    array[1][0][0] = 21;
    println!("array : {array:?}");
    array[2][0][0] = 31;
    println!("array : {array:?}");
}
