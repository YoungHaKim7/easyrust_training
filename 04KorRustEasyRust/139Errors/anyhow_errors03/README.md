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
   Compiling anyhow_errors03 v0.1.0 (/Users/globalyoung/Documents/test/test/rust/testeasyrust/04KorRustEasyRust/139Errors/anyhow_errors03)
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

warning: unused return value of `anyhow::__private::must_use` that must be used
 --> src/main.rs:6:9
  |
6 |         anyhow!("Uh oh, x shouldn't be 9");
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_must_use)]` on by default
  = note: this warning originates in the macro `anyhow` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: `anyhow_errors03` (bin "anyhow_errors03") generated 3 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/anyhow_errors03`

Ok(())
Ok(())

```
