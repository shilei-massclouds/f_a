use core::alloc::Layout;
use std::sync::Arc;

extern "Rust" {
    fn sys_call_nn();
    fn sys_call_bn(v: usize);
    fn sys_call_rbn(v: &usize);
    fn sys_call_sn(layout: Layout);
    fn sys_call_rsn(layout: &Layout);
    fn sys_call_rcn(s: &str);
    fn sys_call_cn(s: String);
    fn sys_call_vn(v: Vec<usize>);
    fn sys_call_rvn(v: &Vec<usize>);
    fn sys_call_enum(n: EnumNumber);
    fn sys_call_ref_enum(n: &EnumNumber);
    fn sys_call_usize_with_result(n: usize) -> AxResult;
    fn sys_call_usize_with_vec() -> Vec<usize>;
    fn sys_call_usize_with_arc_vec() -> Arc<Vec<usize>>;
    fn sys_call_usize_with_vec_leak<'a>() -> &'a mut [usize];
    fn sys_call_usize_with_vec_leak2<'a>() -> (*const VecItem, usize);
    fn sys_call_usize_with_box_leak<'a>() -> &'a mut Vec<VecItem>;
}

#[derive(Clone, Debug)]
struct VecItem(usize);

impl Drop for VecItem {
    fn drop(&mut self) {
        println!("VecItem {} drop success!", self.0);
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AxError {
    Error1,
    Error2,
}

pub type AxResult<T = ()> = Result<T, AxError>;

#[derive(Debug)]
enum EnumNumber {
    One,
    Two(usize),
    Three(String),
}

fn main() {
    println!("\n##############");
    println!("Rust-ABI: ");
    println!("##############\n");

    unsafe { sys_call_nn() };

    {
        let val = 3;
        let ptr = &val as *const _;

        println!("fn sys_call_bn(v: usize);");
        println!("[caller]: {:?}", ptr);
        unsafe { sys_call_bn(val) };
    }

    {
        let val = 3;
        let ptr = &val as *const _;
        println!("fn sys_call_rbn(v: &usize);");
        println!("[caller]: {:?}", ptr);
        unsafe { sys_call_rbn(&val) };
    }

    {
        let layout = Layout::from_size_align(64, 64).unwrap();
        let ptr = &layout as *const _;
        println!("fn sys_call_sn(layout: Layout);");
        println!("[caller]: {:?}", ptr);
        unsafe { sys_call_sn(layout) };
    }

    {
        let layout = Layout::from_size_align(64, 64).unwrap();
        let ptr = &layout as *const _;
        println!("fn sys_call_rsn(layout: &Layout);");
        println!("[caller]: {:?}", ptr);
        unsafe { sys_call_rsn(&layout) };
    }

    {
        let s: &str = "hello";
        let ptr = s as *const _;
        println!("fn sys_call_rcn(s: &str);");
        println!("[caller]: {:?}; as_ptr(): {:?}", ptr, s.as_ptr());
        unsafe { sys_call_rcn(s) };
    }

    {
        let s = String::from("hello");
        let ptr = &s as *const _;
        println!("fn sys_call_cn(s: String);");
        println!("[caller]: {:?}; as_ptr(): {:?}", ptr, s.as_ptr());
        unsafe { sys_call_cn(s) };
    }

    {
        let v = vec![1, 2, 3];
        let ptr = &v as *const _;
        println!("fn sys_call_vn(v: Vec);");
        println!("[caller]: {:?}; as_ptr(): {:?}", ptr, v.as_ptr());
        unsafe { sys_call_vn(v) };
    }

    {
        let v = vec![1, 2, 3];
        let ptr = &v as *const _;
        println!("fn sys_call_rvn(v: &Vec);");
        println!("[caller]: {:?}; as_ptr(): {:?}", ptr, v.as_ptr());
        unsafe { sys_call_rvn(&v) };
    }

    {
        println!("fn sys_call_enum(n: EnumNumber);");

        let one = EnumNumber::One;
        let ptr = &one as *const _;
        println!("[caller]: {:?}; val: {:?}", ptr, one);
        unsafe { sys_call_enum(one) };

        let two = EnumNumber::Two(100);
        let ptr = &two as *const _;
        println!("[caller]: {:?}; val: {:?}", ptr, two);
        unsafe { sys_call_enum(two) };

        let s = String::from("Hello");
        let s_ptr = s.as_ptr();
        let three = EnumNumber::Three(s);
        let ptr = &three as *const _;
        println!("[caller]: {:?}; as_ptr(): {:?}; val: {:?}",
            ptr, s_ptr, three);
        unsafe { sys_call_enum(three) };
    }

    {
        println!("fn sys_call_ref_enum(n: &EnumNumber);");

        let one = EnumNumber::One;
        let ptr = &one as *const _;
        println!("[caller]: {:?}; val: {:?}", ptr, one);
        unsafe { sys_call_ref_enum(&one) };

        let two = EnumNumber::Two(100);
        let ptr = &two as *const _;
        println!("[caller]: {:?}; val: {:?}", ptr, two);
        unsafe { sys_call_ref_enum(&two) };

        let s = String::from("Hello");
        let s_ptr = s.as_ptr();
        let three = EnumNumber::Three(s);
        let ptr = &three as *const _;
        println!("[caller]: {:?}; as_ptr(): {:?}; val: {:?}",
            ptr, s_ptr, three);
        unsafe { sys_call_ref_enum(&three) };
    }

    {
        println!("fn sys_call_usize_with_result(n: usize) -> AxResult;");
        fn test(n: usize) {
            let ret = unsafe { sys_call_usize_with_result(n) };
            let ptr = &ret as *const _;
            match ret {
                Ok(_v) => {
                    println!("[caller]: input {}; got {:?} [Ok]", n, ptr);
                },
                Err(e) => {
                    println!("[caller]: input {}; got {:?} [Err: {:?}]",
                        n, ptr, e);
                }
            }
        }
        test(0);
        test(1);
        test(2);
        println!();
    }

    {
        println!("fn sys_call_usize_with_box_leak<'a>() -> &'a mut Vec<usize>;");
        let v = unsafe { sys_call_usize_with_box_leak() };
        let ptr = v as *mut _;
        println!("[caller]: ret {:?}; vec.buf {:?}\n", ptr, v.as_ptr());
        unsafe { core::ptr::drop_in_place(v.as_mut_slice()) }
        println!("[caller]: drop ok!\n");
    }

    //
    // These counter examples wil cause segment fault or memory leak!
    //
    //counter_examples();

    println!("\n##############");
    println!("Rust-ABI: all tests ok!");
    println!("##############\n");
}

#[allow(dead_code)]
fn counter_examples() {

    // Counter example: return Vec directly.
    // Okay for func; but segment fault for abi.
    {
        println!("fn sys_call_usize_with_vec() -> Vec<usize>;");
        let v = unsafe { sys_call_usize_with_vec() };
        let ptr = &v as *const _;
        println!("[caller]: ret {:?}; vec.buf {:?}\n", ptr, v.as_ptr());
    }

    // Counter example: return Vec leaking buffer.
    // Okay for func; but segment fault for abi.
    {
        println!("fn sys_call_usize_with_vec() -> Arc<Vec<usize>>;");
        let av = unsafe { sys_call_usize_with_arc_vec() };
        let ptr = &av as *const _;
        println!("[caller]: ret {:?}; vec.buf {:?}\n", ptr, av.as_ptr());
    }

    // Counter example: don't know how to drop leaking memory
    // from the other side.
    {
        println!("fn sys_call_usize_with_vec_leak() -> &'a mut [usize];");
        let v = unsafe { sys_call_usize_with_vec_leak() };
        let ptr = &v as *const _;
        println!("[caller]: ret {:?}; vec.buf {:?}\n", ptr, v.as_ptr());
        // NOTE: I don't know how to drop vec's LEAKING buffer
        // from the other side.
    }

    // Counter example: don't know how to drop leaking memory
    // from the other side.
    {
        println!("fn sys_call_usize_with_vec_leak2() -> &'a mut [VecItem];");
        let (ptr, len) = unsafe { sys_call_usize_with_vec_leak2() };
        let v = unsafe { core::slice::from_raw_parts(ptr, len) };
        let ptr = &v as *const _;
        println!("[caller]: ret {:?}; vec.buf {:?}\n", ptr, v.as_ptr());
        // NOTE: I don't know how to drop vec's LEAKING buffer
        // from the other side.
        // Never see VecItem.drop is called automatically.
        // 1) drop(v) doesn't work! And drop(iter) in loop doesn't work!
        // 2) Vec::from(v) and clear() doesn't work! It seems okay! But
        // in fact, it just copys the memory and frees the copy.
    }
}
