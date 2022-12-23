# Result : 

```
$ cargo add anyhow
    Updating crates.io index
      Adding anyhow v1.0.68 to dependencies.
             Features:
             + std
             - backtrace
ls
Cargo.lock Cargo.toml README.md  src        target

$ cargo run
   Compiling anyhow v1.0.68
   Compiling anyhow_errors_context01 v0.1.0 (/Users/globalyoung/Documents/test/test/rust/testeasyrust/04KorRustEasyRust/139Errors/anyhow_errors_context01)
warning: unused variable: `my_integer`
 --> src/main.rs:4:9
  |
4 |     let my_integer = int.parse::<i32>().with_context(|| "Extra info in here")?;
  |         ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_my_integer`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `my_float`
 --> src/main.rs:5:9
  |
5 |     let my_float = float.parse::<f64>().with_context(|| "Extra flota info in here")?;
  |         ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_my_float`

warning: `anyhow_errors_context01` (bin "anyhow_errors_context01") generated 2 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/anyhow_errors_context01`


Err(Extra flota info in here

Caused by:
    invalid float literal)
Err(Extra info in here

Caused by:
    invalid digit found in string)

```

<br>

<hr>

# 프로그래밍 언어 러스트를 배웁시다! 141 Easy Rust in Korean: Anyhow crate


https://youtu.be/Ar9NWLxLwEQ
