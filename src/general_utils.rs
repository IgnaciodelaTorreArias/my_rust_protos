
#[macro_export]
macro_rules! set_empty_output {
    ($out_ptr:expr, $out_len:expr) => {
        unsafe {
            *$out_ptr = std::ptr::null_mut();
            *$out_len = 0;
        }
    };
}