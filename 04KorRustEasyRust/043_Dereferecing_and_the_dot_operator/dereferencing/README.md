# Result:

```
$ cargo run
   Compiling dereferencing v0.1.0 (/Users/globalyoung/Documents/test/test/rust/testeasyrust/04KorRustEasyRust/043_Dereferecing_and_the_dot_operator/dereferencing)
warning: unused variable: `reference_item`
  --> src/main.rs:15:9
   |
15 |     let reference_item = &item;
   |         ^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_reference_item`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: `dereferencing` (bin "dereferencing") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/dereferencing`

Are they equal? true

```
