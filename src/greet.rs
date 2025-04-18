use crate::messages::my_package::{Greetings, Response};
use crate::utils::*;

#[unsafe(no_mangle)]
pub extern "C" fn rust_protos_greet(ptr: *const u8, len: usize, out_ptr: *mut *mut u8, out_len: *mut usize) -> i32 {
    let greetings = match get_call_message::<Greetings>(ptr, len) {
        Ok(m) => m,
        Err(err) => {
            if set_call_result(&err, out_ptr, out_len){
                return -2;
            }
            else {
                return -3;
            }
        }
    };
    let response = Response { text: format!("Hello {}, have a good day!!!", greetings.name) };
    if !set_call_result(&response, out_ptr, out_len){
        return -1;
    };
    0
}