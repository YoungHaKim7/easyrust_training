fn main() {
    let width = 4;
    let height = 4;
    let mut array = vec![vec![0; width]; height];
    println!("example array: {:?}", array);

    println!("---------------------");
    println!("---------------------");
    let mut grid_raw = vec![0; width * height];

    // Vector of 'width' elements slices
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();

    // Final 2d array `&mut [&mut [_]]`
    let grid = grid_base.as_mut_slice();

    // Accessing data
    grid[0][0] = 4;
    println!("grid : {grid:?}");
}
