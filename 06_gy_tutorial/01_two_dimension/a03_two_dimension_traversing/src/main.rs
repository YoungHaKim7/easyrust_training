fn main() {
    const M: usize = 2;
    const N: usize = 4;

    let mut grid = [[0 as u8; N]; M];

    // Traversing
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            print!("{} ", col);
        }
        println!()
    }
    grid[1][2] = 5;
    println!("grid : {:?}", grid);

    // or
    // for el in grid.iter().flat_map(|r| r.iter()) {
    //     println!("{el}");
    // }
}
