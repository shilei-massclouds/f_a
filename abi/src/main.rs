extern "Rust" {
    fn sys_call_nn();
}

fn main() {
    println!("Client: ");
    unsafe { sys_call_nn() };
}
