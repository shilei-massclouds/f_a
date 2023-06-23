//
// Experiment:
// Various types of arguments & return_value cross Rust-ABI.
// [n]: no args or ret
// [b]: basic types
// [r]: reference
//

#![cfg_attr(target_os = "none", no_std)]

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
use alloc::sync::Arc;
use alloc::boxed::Box;

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

// Counter example: return Vec directly.
// Okay for func; but segment fault for abi.
#[no_mangle]
pub fn sys_call_usize_with_vec() -> Vec<usize> {
    let v = alloc::vec!(1, 2, 3);

    let ptr = &v as *const _;
    println!("[callee]: ret {:?}; vec.buf {:?}", ptr, v.as_ptr());
    v
}

#[no_mangle]
pub fn sys_call_usize_with_arc_vec() -> Arc<Vec<usize>> {
    let av = Arc::new(alloc::vec!(1, 2, 3));

    let ptr = &av as *const _;
    println!("[callee]: ret {:?}; vec.buf {:?}", ptr, av.as_ptr());
    av.clone()
}

#[no_mangle]
pub fn sys_call_usize_with_vec_leak<'a>() -> &'a mut [usize] {
    let v: Vec<usize> = alloc::vec!(1, 2, 3);
    let v = v.leak();

    let ptr = &v as *const _;
    println!("[callee]: ret {:?}; vec.buf {:?}", ptr, (&v).as_ptr());
    v
}

#[derive(Clone, Debug)]
pub struct VecItem(usize);

impl VecItem {
    fn new(n: usize) -> Self {
        Self { 0: n }
    }
}

impl Drop for VecItem {
    fn drop(&mut self) {
        println!("VecItem {} drop success!", self.0);
    }
}

// Add this test to make sure that vec's LEAKING buffer will
// safely dropped on the other size.
#[no_mangle]
pub fn sys_call_usize_with_vec_leak2<'a>() -> &'a mut [VecItem] {
    let v: Vec<VecItem> = alloc::vec!(
        VecItem::new(1), VecItem::new(2), VecItem::new(3)
    );
    println!("[callee]: before leak: vec.buf {:?}", v.as_slice().as_ptr());
    let v = v.leak();
    println!("[callee]: after  leak: vec.buf {:?}", v.as_ptr());
    v
}

#[no_mangle]
pub fn sys_call_usize_with_box_leak<'a>() -> &'a mut Vec<VecItem> {
    let v: Vec<VecItem> = alloc::vec!(
        VecItem::new(1), VecItem::new(2), VecItem::new(3)
    );
    let buf_ptr = (&v).as_ptr();
    let ptr = Box::leak(Box::new(v));
    println!("[callee]: ret {:?}; vec.buf {:?}", ptr as *mut _, buf_ptr);
    ptr
}

//
// Panic
//
#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
