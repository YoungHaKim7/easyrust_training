use std::sync::{Mutex, RwLock};

fn main() {
    let rwlock = RwLock::new(1);
    let reader = rwlock.read().unwrap();
    let writer = rwlock.write().unwrap();
    let my_mutex = Mutex::new(1);
    let locked = my_mutex.lock().unwrap();
}
