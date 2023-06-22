//
// Experiment:
// Various types of arguments & return_value cross Rust-ABI.
// [n]: no args or ret
// [b]: basic types
// [r]: reference
//

use core::alloc::Layout;

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
