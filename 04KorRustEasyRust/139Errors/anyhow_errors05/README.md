# Result:

```
$ cargo add anyhow
    Updating crates.io index
      Adding anyhow v1.0.68 to dependencies.
             Features:
             + std
             - backtrace

$ cargo run

   Compiling anyhow v1.0.68
   Compiling anyhow_errors05 v0.1.0 (/Users/globalyoung/Documents/test/test/rust/testeasyrust/04KorRustEasyRust/139Errors/anyhow_errors05)
warning: unused variable: `my_integer`
 --> src/main.rs:4:9
  |
4 |     let my_integer = int.parse::<i32>()?;
  |         ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_my_integer`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `my_float`
 --> src/main.rs:5:9
  |
5 |     let my_float = float.parse::<f64>()?;
  |         ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_my_float`

warning: `anyhow_errors05` (bin "anyhow_errors05") generated 2 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/anyhow_errors05`

Err(invalid float literal)
Err(invalid digit found in string)

```
