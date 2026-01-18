use crate::messages::{Greetings, Response, CallStatus};
use crate::buffer_utils::*;

#[unsafe(no_mangle)]
pub extern "C" fn lib_greet_greet(
    ptr: *const u8,
    len: usize,
    out_ptr: *mut *mut u8,
    out_len: *mut usize,
) -> i32 {
    let greetings = match get_call_message::<Greetings>(ptr, len) {
        Ok(m) => m,
        Err(_) => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::DecodeError.into();
        }
    };
    set_call_result(
        Response {
            text: format!("Hello {}, have a good day!!!", greetings.name),
        },
        out_ptr,
        out_len
    );
    CallStatus::Ok.into()
}
