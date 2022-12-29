# Result:(Error code)


```
cargo run
   Compiling generics05 v0.1.0 (/Users/globalyoung/Documents/test/test/test/testeasyrust/04KorRustEasyRust/044_Generics/generics05)
error[E0277]: `Book` doesn't implement `std::fmt::Display`
  --> src/main.rs:14:24
   |
14 |     let z = give_thing(Book);
   |             ---------- ^^^^ `Book` cannot be formatted with the default formatter
   |             |
   |             required by a bound introduced by this call
   |
   = help: the trait `std::fmt::Display` is not implemented for `Book`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
note: required by a bound in `give_thing`
  --> src/main.rs:6:18
   |
6  | fn give_thing<T: Display + Debug>(input: T) -> T {
   |                  ^^^^^^^ required by this bound in `give_thing`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `generics05` due to previous error
```
