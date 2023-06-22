// macros

/// Prints to the standard output.
///
/// Equivalent to the [`println!`] macro except that a newline is not printed at
/// the end of the message.
///
/// [`println!`]: crate::println
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::io::__print_impl(format_args!($fmt $(, $($arg)+)?));
    }
}

/// Prints to the standard output, with a newline.
#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::io::__print_impl(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
