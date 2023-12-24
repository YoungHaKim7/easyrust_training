# Result

```
$ cargo r

x print 3
this will happen    
```

<hr>

- cargo expand && cargo asm

```

$ cargo expand

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub fn f(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;
    {
        ::std::io::_print(format_args!("this will happen\n"));
    };
    if before != after {
        {
            ::std::io::_print(format_args!("This will never happen\n"));
        };
    }
}
fn main() {
    let x = 3;
    let mut y = 6;
    {
        ::std::io::_print(format_args!("x print {0}\n", x));
    };
    f(&x, &mut y);
}

~~~~~~~~cargo asm

$ cargo asm

a02_reference_def::main
core::ops::function::FnOnce::call_once{{vtable.shim}}
core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
std::rt::lang_start
std::rt::lang_start::{{closure}}
std::sys_common::backtrace::__rust_begin_short_backtrace

~~~~~~~~ cargo asm main ~~~~~

$ cargo asm a02_reference_def::main

a02_reference_def::main:
 push    rbx
 sub     rsp, 80
 mov     dword, ptr, [rsp, +, 12], 3
 lea     rax, [rsp, +, 12]
 mov     qword, ptr, [rsp, +, 64], rax
 mov     rax, qword, ptr, [rip, +, _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h410bb60a6a051641E@GOTPCREL]
 mov     qword, ptr, [rsp, +, 72], rax
 lea     rax, [rip, +, .L__unnamed_2]
 mov     qword, ptr, [rsp, +, 16], rax
 mov     qword, ptr, [rsp, +, 24], 2
 mov     qword, ptr, [rsp, +, 48], 0
 lea     rax, [rsp, +, 64]
 mov     qword, ptr, [rsp, +, 32], rax
 mov     qword, ptr, [rsp, +, 40], 1
 mov     rbx, qword, ptr, [rip, +, _ZN3std2io5stdio6_print17h63a00216c7cec9b0E@GOTPCREL]
 lea     rdi, [rsp, +, 16]
 call    rbx
 lea     rax, [rip, +, .L__unnamed_3]
 mov     qword, ptr, [rsp, +, 16], rax
 mov     qword, ptr, [rsp, +, 24], 1
 lea     rax, [rip, +, .L__unnamed_4]
 mov     qword, ptr, [rsp, +, 32], rax
 xorps   xmm0, xmm0
 movups  xmmword, ptr, [rsp, +, 40], xmm0
 lea     rdi, [rsp, +, 16]
 call    rbx
 add     rsp, 80
 pop     rbx
 ret

```
