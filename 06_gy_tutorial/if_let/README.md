# cargo asm 


```
$ cargo asm
a01_if_let::main
core::ops::function::FnOnce::call_once{{vtable.shim}}
core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
std::rt::lang_start
std::rt::lang_start::{{closure}}
std::sys_common::backtrace::__rust_begin_short_backtrace


$ cargo asm core::ops::function::FnOnce::call_once{{vtable.shim}} --rust
file /rustc/a28077b28a02b92985b3a3faecf92813155f1ea1/library/core/src/ops does not exist!
file /rustc/a28077b28a02b92985b3a3faecf92813155f1ea1/library/std/src does not exist!
 push    rax
 mov     rdi, qword, ptr, [rdi]
     call    std::sys_common::backtrace::__rust_begin_short_backtrace
 xor     eax, eax
 pop     rcx
 ret

```
