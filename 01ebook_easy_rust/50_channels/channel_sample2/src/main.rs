use std::sync::mpsc::channel;

fn main() {
    let (sender, receiver) = channel();
    let sender_clone = sender.clone();

    std::thread::scope(|_| {
        // move sender in
        sender.send("Send a &str this time").unwrap();
    });

    std::thread::scope(move |_| {
        // move sender_clone in
        sender_clone.send("And here is another &str").unwrap();
    });

    println!("{}", receiver.recv().unwrap());
}
