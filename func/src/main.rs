use core::alloc::Layout;
use sys::EnumNumber;

fn main() {
    println!("\n##############");
    println!("FunctionCall: ");
    println!("##############\n");

    sys::sys_call_nn();

    {
        let val = 3;
        let ptr = &val as *const _;
        println!("fn sys_call_bn(v: usize);");
        println!("[caller]: {:?}", ptr);
        sys::sys_call_bn(val);
    }

    {
        let val = 3;
        let ptr = &val as *const _;
        println!("fn sys_call_rbn(v: &usize);");
        println!("[caller]: {:?}", ptr);
        sys::sys_call_rbn(&val);
    }

    {
        let layout = Layout::from_size_align(64, 64).unwrap();
        let ptr = &layout as *const _;
        println!("fn sys_call_sn(layout: Layout);");
        println!("[caller]: {:?}", ptr);
        sys::sys_call_sn(layout);
    }

    {
        let layout = Layout::from_size_align(64, 64).unwrap();
        let ptr = &layout as *const _;
        println!("fn sys_call_rsn(layout: &Layout);");
        println!("[caller]: {:?}", ptr);
        sys::sys_call_rsn(&layout);
    }

    {
        let s: &str = "hello";
        let ptr = s as *const _;
        println!("fn sys_call_rcn(s: &str);");
        println!("[caller]: {:?}; as_ptr(): {:?}", ptr, s.as_ptr());
        sys::sys_call_rcn(s);
    }

    {
        let s = String::from("hello");
        let ptr = &s as *const _;
        println!("fn sys_call_cn(s: String);");
        println!("[caller]: {:?}; as_ptr(): {:?}", ptr, s.as_ptr());
        sys::sys_call_cn(s);
    }

    {
        let v = vec![1, 2, 3];
        let ptr = &v as *const _;
        println!("fn sys_call_vn(v: Vec);");
        println!("[caller]: {:?}; as_ptr(): {:?}", ptr, v.as_ptr());
        sys::sys_call_vn(v);
    }

    {
        let v = vec![1, 2, 3];
        let ptr = &v as *const _;
        println!("fn sys_call_rvn(v: &Vec);");
        println!("[caller]: {:?}; as_ptr(): {:?}", ptr, v.as_ptr());
        sys::sys_call_rvn(&v);
    }

    {
        println!("fn sys_call_enum(n: EnumNumber);");

        let one = EnumNumber::One;
        let ptr = &one as *const _;
        println!("[caller]: {:?}; val: {:?}", ptr, one);
        sys::sys_call_enum(one);

        let two = EnumNumber::Two(100);
        let ptr = &two as *const _;
        println!("[caller]: {:?}; val: {:?}", ptr, two);
        sys::sys_call_enum(two);

        let s = String::from("Hello");
        let s_ptr = s.as_ptr();
        let three = EnumNumber::Three(s);
        let ptr = &three as *const _;
        println!("[caller]: {:?}; as_ptr(): {:?}; val: {:?}",
            ptr, s_ptr, three);
        sys::sys_call_enum(three);
    }

    {
        println!("fn sys_call_ref_enum(n: &EnumNumber);");

        let one = EnumNumber::One;
        let ptr = &one as *const _;
        println!("[caller]: {:?}; val: {:?}", ptr, one);
        sys::sys_call_ref_enum(&one);

        let two = EnumNumber::Two(100);
        let ptr = &two as *const _;
        println!("[caller]: {:?}; val: {:?}", ptr, two);
        sys::sys_call_ref_enum(&two);

        let s = String::from("Hello");
        let s_ptr = s.as_ptr();
        let three = EnumNumber::Three(s);
        let ptr = &three as *const _;
        println!("[caller]: {:?}; as_ptr(): {:?}; val: {:?}",
            ptr, s_ptr, three);
        sys::sys_call_ref_enum(&three);
    }

    {
        println!("fn sys_call_usize_with_result(n: usize) -> AxResult;");
        fn test(n: usize) {
            let ret = sys::sys_call_usize_with_result(n);
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
        let v = sys::sys_call_usize_with_box_leak();
        let ptr = v as *mut _;
        println!("[caller]: ret {:?}; vec.buf {:?}\n", ptr, v.as_ptr());
        unsafe { core::ptr::drop_in_place(v.as_mut_slice()) }
        println!("[caller]: drop ok!\n");
    }

    {
        println!("fn sys_call_usize_with_vec_leak() -> &'a mut [usize];");
        let v = sys::sys_call_usize_with_vec_leak();
        let ptr = &v as *const _;
        println!("[caller]: ret {:?}; vec.buf {:?}\n", ptr, v.as_ptr());
        unsafe { core::ptr::drop_in_place(v) }
        println!("[caller]: drop ok!\n");
    }

    {
        println!("fn sys_call_usize_with_vec_leak2() -> &'a mut [VecItem];");
        let v = sys::sys_call_usize_with_vec_leak2();
        let ptr = &v as *const _;
        println!("[caller]: ret {:?}; vec.buf {:?}\n", ptr, v.as_ptr());
        unsafe { core::ptr::drop_in_place(v) }
        println!("[caller]: drop ok!\n");
    }

    {
        println!("fn sys_call_with_struct() -> Layout;");
        let layout = sys::sys_call_with_struct();
        let ptr = &layout as *const _;
        println!("[caller]: val: {:?}; ptr {:?}", layout, ptr);
    }

    {
        println!();
        println!("fn sys_call_with_opt_struct() -> Option<Layout>;");
        let layout = sys::sys_call_with_opt_struct(1);
        assert!(!layout.is_none());
        if let Some(v) = layout {
            let ptr = &v as *const _;
            println!("[caller]: val: {:?}; ptr {:?}", v, ptr);
        }

        let layout = sys::sys_call_with_opt_struct(0);
        assert!(layout.is_none());
    }


    //
    // These counter examples wil cause segment fault or memory leak!
    // Note: Some of these are okay for functional call.
    //
    //counter_examples();

    println!("\n##############");
    println!("FunctionCall: all tests ok!");
    println!("##############\n");
}

#[allow(dead_code)]
fn counter_examples() {
    // Counter example: return Vec directly.
    // Okay for func; but segment fault for abi.
    {
        println!("fn sys_call_usize_with_vec() -> Vec<usize>;");
        let v = sys::sys_call_usize_with_vec();
        let ptr = &v as *const _;
        println!("[caller]: ret {:?}; vec.buf {:?}\n", ptr, v.as_ptr());
    }

    {
        println!("fn sys_call_usize_with_vec() -> Arc<Vec<usize>>;");
        let av = sys::sys_call_usize_with_arc_vec();
        let ptr = &av as *const _;
        println!("[caller]: ret {:?}; vec.buf {:?}\n", ptr, av.as_ptr());
    }
}
