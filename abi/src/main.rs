use core::alloc::Layout;

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
        println!("[caller]: {:?}", ptr);
        unsafe { sys_call_rcn(s) };
    }

    {
        let s = String::from("hello");
        let ptr = &s as *const _;
        println!("fn sys_call_cn(s: String);");
        println!("[caller]: {:?}; inner: {:?}", ptr, s.as_ptr());
        unsafe { sys_call_cn(s) };
    }

    {
        let v = vec![1, 2, 3];
        let ptr = &v as *const _;
        println!("fn sys_call_vn(v: Vec);");
        println!("[caller]: {:?}; inner: {:?}", ptr, v.as_ptr());
        unsafe { sys_call_vn(v) };
    }

    {
        let v = vec![1, 2, 3];
        let ptr = &v as *const _;
        println!("fn sys_call_rvn(v: &Vec);");
        println!("[caller]: {:?}; inner: {:?}", ptr, v.as_ptr());
        unsafe { sys_call_rvn(&v) };
    }
}
