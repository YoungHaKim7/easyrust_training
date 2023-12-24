fn main() {
    let mut map_iter: Vec<u64> = (1..=5).map(|v| v * 10).collect();
    map_iter.iter_mut().for_each(|element| {
        println!("test {:?}", element);
    });
    println!("map_iter = {:?}", map_iter);
}
