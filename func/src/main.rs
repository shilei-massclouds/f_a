use core::alloc::Layout;

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
}
