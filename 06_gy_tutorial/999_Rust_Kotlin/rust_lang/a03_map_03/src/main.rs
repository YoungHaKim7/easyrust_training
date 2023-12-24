fn main() {
    let map_iter: Vec<u64> = (1..=5).map(|v| v * 10).collect();
    println!("map_iter = {:?}", map_iter);
}
