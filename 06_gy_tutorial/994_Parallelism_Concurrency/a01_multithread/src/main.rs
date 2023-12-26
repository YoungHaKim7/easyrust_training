use std::thread;
use std::time::Duration;

fn main() {
    let num_threads = 4;
    let mut handles = Vec::new();

    for i in 0..num_threads {
        let handle = thread::spawn(move || {
            println!("Thread {} started.", i);
            let fibonacci = fibonacci_sequence(i * 10);
            println!(
                "Thread {} computed Fibonacci sequence of length {}",
                i,
                fibonacci.len()
            );
            println!("Thread {}: Fibonacci sequence: {:?}", i, fibonacci);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn fibonacci_sequence(n: usize) -> Vec<u32> {
    let mut sequence = vec![0, 1];
    if n > 1 {
        for _ in 2..(n + 1) {
            let next = sequence[sequence.len() - 1] + sequence[sequence.len() - 2];
            sequence.push(next);
        }
    }
    sequence
}
