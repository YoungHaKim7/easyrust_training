# Result

```rs
Struct std::sync::RwLockReadGuard

Struct std::sync::RwLockWriteGuard


use std::sync::Mutex;

let mutex = Mutex::new(0);

impl<T: ?Sized> Mutex<T>
pub fn lock(&self) -> LockResult<MutexGuard<'_, T>>

```


- https://doc.rust-lang.org/stable/std/sync/struct.RwLockReadGuard.html
- https://doc.rust-lang.org/stable/std/sync/struct.RwLockWriteGuard.html
- https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html
