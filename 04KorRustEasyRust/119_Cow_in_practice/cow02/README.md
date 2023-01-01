# Result : 

'''
$ cargo run

   Compiling cow02 v0.1.0 (/home/young/my_project/rust/testeasyrust/04KorRustEasyRust/119_Cow_in_practice/cow02)
warning: field `name` is never read
 --> src/main.rs:5:5
  |
4 | struct User<'a> {
  |        ---- field in this struct
5 |     name: Cow<'a, str>,
  |     ^^^^
  |
  = note: `User` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` on by default

warning: `cow02` (bin "cow02") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/cow02`


User 1 is User { name: "User 1" } and User 2 is User { name: "User 2" }

```
