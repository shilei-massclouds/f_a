//
// Experiment:
// Various types of arguments & return_value cross Rust-ABI.
// [n]: no args or ret
// [b]: basic types
// [r]: reference
//

#![cfg_attr(target_os = "none", no_std)]

#[cfg(target_os = "none")]
extern crate alloc;

#[cfg(target_os = "none")]
use alloc::{string::String, vec::Vec};

#[cfg(target_os = "none")]
mod macros;

#[cfg(target_os = "none")]
mod io;

#[cfg(target_os = "none")]
mod mem;

use core::alloc::Layout;

#[derive(Debug)]
pub enum EnumNumber {
    One,
    Two(usize),
    Three(String),
}

#[no_mangle]
pub fn sys_call_nn() {
    println!("fn sys_call_nn();\n");
}

#[no_mangle]
pub fn sys_call_bn(val: usize) {
    let ptr = &val as *const _;
    println!("[callee]: {:?}\n", ptr);
}

#[no_mangle]
pub fn sys_call_rbn(val: &usize) {
    let ptr = &(*val) as *const _;
    println!("[callee]: {:?}\n", ptr);
}

#[no_mangle]
pub fn sys_call_sn(layout: Layout) {
    let ptr = &layout as *const _;
    println!("[callee]: {:?}\n", ptr);
}

#[no_mangle]
pub fn sys_call_rsn(layout: &Layout) {
    let ptr = &(*layout) as *const _;
    println!("[callee]: {:?}\n", ptr);
}

#[no_mangle]
pub fn sys_call_rcn(s: &str) {
    let ptr = &(*s) as *const _;
    println!("[callee]: {:?}; as_ptr(): {:?}; val: {:?}\n", ptr, s.as_ptr(), s);
}

#[no_mangle]
pub fn sys_call_cn(s: String) {
    let ptr = &s as *const _;
    println!("[callee]: {:?}; as_ptr(): {:?}; val: {:?}\n", ptr, s.as_ptr(), s);
}

#[no_mangle]
pub fn sys_call_vn(v: Vec<usize>) {
    let ptr = &v as *const _;
    println!("[callee]: {:?}; as_ptr(): {:?}; val: {:?}\n", ptr, v.as_ptr(), v);
}

#[no_mangle]
pub fn sys_call_rvn(v: &Vec<usize>) {
    let ptr = &(*v) as *const _;
    println!("[callee]: {:?}; as_ptr(): {:?}; val: {:?}\n", ptr, v.as_ptr(), v);
}

#[no_mangle]
pub fn sys_call_enum(n: EnumNumber) {
    let ptr = &n as *const _;
    match n {
        EnumNumber::One => {
            println!("[callee]: {:?}; val: {:?}\n", ptr, n);
        },
        EnumNumber::Two(_v) => {
            println!("[callee]: {:?}; val: {:?}\n", ptr, n);
        },
        EnumNumber::Three(ref s) => {
            println!("[callee]: {:?}; as_ptr(): {:?}; val: {:?}\n",
                ptr, s.as_ptr(), n);
        },
    }
}

#[no_mangle]
pub fn sys_call_ref_enum(n: &EnumNumber) {
    let ptr = &(*n) as *const _;
    match n {
        EnumNumber::One => {
            println!("[callee]: {:?}; val: {:?}\n", ptr, n);
        },
        EnumNumber::Two(_v) => {
            println!("[callee]: {:?}; val: {:?}\n", ptr, n);
        },
        EnumNumber::Three(s) => {
            println!("[callee]: {:?}; as_ptr(): {:?}; val: {:?}\n",
                ptr, s.as_ptr(), n);
        },
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AxError {
    Error1,
    Error2,
}

pub type AxResult<T = ()> = Result<T, AxError>;

#[no_mangle]
pub fn sys_call_usize_with_result(n: usize) -> AxResult {
    let ret = match n {
        0 => Ok(()),
        1 => Err(AxError::Error1),
        _ => Err(AxError::Error2),
    };

    let ptr = &ret as *const _;
    println!("[callee]: input {}; ret {:?}", n, ptr);
    ret
}

//
// Panic
//
#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
