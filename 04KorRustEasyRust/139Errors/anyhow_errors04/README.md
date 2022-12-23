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
   Compiling anyhow_errors04 v0.1.0 (/Users/globalyoung/Documents/test/test/rust/testeasyrust/04KorRustEasyRust/139Errors/anyhow_errors04)
warning: unused variable: `int`
 --> src/main.rs:3:24
  |
3 | fn try_to_make_numbers(int: &str, float: &str) -> Result<(), Error> {
  |                        ^^^ help: if this is intentional, prefix it with an underscore: `_int`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `float`
 --> src/main.rs:3:35
  |
3 | fn try_to_make_numbers(int: &str, float: &str) -> Result<(), Error> {
  |                                   ^^^^^ help: if this is intentional, prefix it with an underscore: `_float`

warning: `anyhow_errors04` (bin "anyhow_errors04") generated 2 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/debug/anyhow_errors04`

Err(Uhoh, x shouldn't be 9)
Err(Uhoh, x shouldn't be 9)

```
