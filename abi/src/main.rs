extern "Rust" {
    fn sys_call_nn();
    fn sys_call_bn(v: usize);
    fn sys_call_rbn(v: &usize);
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
}
