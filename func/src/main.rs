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
}
