# Result:

```
$ cargo run
   Compiling references v0.1.0 (/Users/globalyoung/Documents/test/test/rust/testeasyrust/04KorRustEasyRust/042_Dereferecing_and_the_dot_operator/references)
error[E0277]: can't compare `{integer}` with `&{integer}`
 --> src/main.rs:5:47
  |
5 |     println!("Are the the same?{}", my_number == reference);
  |                                               ^^ no implementation for `{integer} == &{integer}`
  |
  = help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`
  = help: the following other types implement trait `PartialEq<Rhs>`:
            f32
            f64
            i128
            i16
            i32
            i64
            i8
            isize
          and 6 others

For more information about this error, try `rustc --explain E0277`.
error: could not compile `references` due to previous error
```
