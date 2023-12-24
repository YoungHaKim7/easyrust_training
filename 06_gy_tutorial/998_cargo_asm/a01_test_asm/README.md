# cargo asm

```bash

$ cargo asm

<&T as core::fmt::Debug>::fmt
<alloc::vec::Vec<T,A> as core::fmt::Debug>::fmt
a01_test_asm::main
core::ops::function::FnOnce::call_once{{vtable.shim}}
core::ptr::drop_in_place<&u64>
core::ptr::drop_in_place<alloc::vec::Vec<u64>>
std::rt::lang_start
std::rt::lang_start::{{closure}}
std::sys_common::backtrace::__rust_begin_short_backtrace

// cargo asm 뒤에 보고 싶은거 하고 --rust

$ cargo asm core::ops::function::FnOnce::call_once{{vtable.shim}} --rust

file /rustc/a28077b28a02b92985b3a3faecf92813155f1ea1/library/std/src does not exist!
file /rustc/a28077b28a02b92985b3a3faecf92813155f1ea1/library/core/src/ops does not exist!
 push    rax
 mov     rdi, qword, ptr, [rdi]
     call    std::sys_common::backtrace::__rust_begin_short_backtrace
 xor     eax, eax
 pop     rcx
 ret    
```


# cargo expand

```

$ cargo expand
    Checking a01_test_asm v0.1.0 (/home/gyoung/my_project/Rust_lang/easyrust_training/06_gy_tutorial/998_cargo_asm/a01_test_asm)
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {
    let vals_map: Vec<u64> = (1..10).map(|v| v + 2).collect();
    {
        ::std::io::_print(format_args!("vals map: {0:?}\n", vals_map));
    };
    let mut number_false = false;
    while !number_false {
        let my_number = 3;
        if my_number < 5 {
            {
                ::std::io::_print(format_args!("condition was true\n"));
            };
            number_false = true;
        } else {
            {
                ::std::io::_print(format_args!("condition was false\n"));
            };
            number_false = false;
        }
    }
}
```
