#![no_std]
#![no_main]

use core::arch::asm;

#[macro_use]
extern crate user_lib;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}

#[no_mangle]
fn main() -> i32 {
    let ptr: usize = 0x8020aea8; // app00中的 Hello, world! 字段
    let len: usize = 13;
    syscall(64, [1, ptr, len]);
    println!("");
    println!("end");
    0
}
