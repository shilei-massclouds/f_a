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
    }

    println!("\n##############");
    println!("FunctionCall: all tests ok!");
    println!("##############\n");
}
