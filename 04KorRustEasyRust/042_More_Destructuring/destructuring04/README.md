# Result :


```
$ cargo run

   Compiling destructuring04 v0.1.0 (/Users/globalyoung/Documents/test/test/rust/testeasyrust/04KorRustEasyRust/042_More_Destructuring/destructuring04)
warning: fields `real_name` and `happiness` are never read
 --> src/main.rs:5:5
  |
3 | struct Person {
  |        ------ fields in this struct
4 |     name: String,
5 |     real_name: String,
  |     ^^^^^^^^^
6 |     height: u8,
7 |     happiness: bool,
  |     ^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: fields `name` and `height` are never read
  --> src/main.rs:12:5
   |
11 | struct Person2 {
   |        ------- fields in this struct
12 |     name: String,
   |     ^^^^
13 |     height: u8,
   |     ^^^^^^
   |
   = note: `Person2` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

warning: `destructuring04` (bin "destructuring04") generated 2 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/destructuring04`

Person2 type is : Person2 { name: "Papa Doc", height: 170 }

```


<br>



