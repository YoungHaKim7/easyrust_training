#  프로그래밍 언어 러스트를 배웁시다! 139 Easy Rust in Korean: Errors

https://youtu.be/hrPO-J_texs

<br>

# Result: 

  ```
  Compiling errors v0.1.0 (D:\young_project\rust_lang\testeasyrust\04KorRustEasyRust\139Errors\errors)
error[E0277]: `?` couldn't convert the error to `ParseIntError`
 --> src\main.rs:5:50
  |
3 | fn try_to_make_number(int_input: &str, float_input: &str) -> Result<i32, ParseIntError> {
  |                                                              -------------------------- expected `ParseIntError` because of this
4 |     let my_number = int_input.parse::<i32>()?;
5 |     let other_number = float_input.parse::<f32>()?;
  |                                                  ^ the trait `From<ParseFloatError>` is not implemented for `ParseIntError`
  |
  = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
  = help: the following other types implement trait `FromResidual<R>`:
            <Result<T, F> as FromResidual<Result<Infallible, E>>>
            <Result<T, F> as FromResidual<Yeet<E>>>
  = note: required for `Result<i32, ParseIntError>` to implement `FromResidual<Result<Infallible, ParseFloatError>>`

error[E0308]: mismatched types
   --> src\main.rs:6:8
    |
6   |     Ok(())
    |     -- ^^ expected `i32`, found `()`
    |     |
    |     arguments to this enum variant are incorrect
    |
note: tuple variant defined here
   --> C:\Users\user\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\result.rs:508:5
    |
508 |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
```
