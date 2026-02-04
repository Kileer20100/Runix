

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        $crate::drivers::vga::xd8000::println::println(format_args!($($arg)*));
    };
}
#[macro_export]
macro_rules! println_warn {
    ($($arg:tt)*) => {
        $crate::drivers::vga::xd8000::println::println_warn(format_args!($($arg)*));
    };
}
#[macro_export]
macro_rules! println_error {
    ($($arg:tt)*) => {
        $crate::drivers::vga::xd8000::println::println_error(format_args!($($arg)*));
    };
}
