// io

use core::fmt::{Write, Error};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, buf: &str) -> Result<(), Error> {
        unsafe {
            syscalls::syscall!(
                syscalls::Sysno::write, 1, buf.as_ptr() as *const _, buf.len())
                .unwrap();
        }
        Ok(())
    }
}

pub fn __print_impl(args: core::fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}
